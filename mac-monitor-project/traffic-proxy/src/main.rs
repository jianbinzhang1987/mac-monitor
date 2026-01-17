use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::env;

use bytes::Bytes;
use http_body_util::{BodyExt, Full, Empty};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode, body::Incoming};
use hyper_util::rt::TokioIo;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::TlsAcceptor;
use log::{info, error, debug};
use rustls::RootCertStore;
use tokio_rustls::TlsConnector;
use rustls::pki_types::ServerName;

use crate::mitm::MitmProxy;

mod mitm;
mod clock;

// Global MITM instance
lazy_static::lazy_static! {
    static ref MITM_PROXY: Mutex<MitmProxy> = Mutex::new(MitmProxy::new());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    
    // Install default crypto provider (ring)
    let _ = rustls::crypto::ring::default_provider().install_default();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8050));
    let listener = TcpListener::bind(addr).await?;
    info!("Starting traffic-proxy on http://{}", addr);

    // Initial setup (Mock for now, should read from files)
    init_config();

    // Periodic device info refresh
    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            if let Err(e) = crate::mitm::update_device_info_from_file("/tmp/mac_monitor_device_info.json") {
                debug!("Periodic device info update failed (file might not exist yet): {}", e);
            }
        }
    });

    loop {
        let (stream, addr) = listener.accept().await?;
        let src_port = addr.port();
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .preserve_header_case(true)
                .title_case_headers(true)
                .serve_connection(io, service_fn(move |req| {
                    proxy(req, src_port)
                }))
                .with_upgrades()
                .await
            {
                error!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

async fn proxy(req: Request<Incoming>, src_port: u16) -> Result<Response<Full<Bytes>>, hyper::Error> {
    if Method::CONNECT == req.method() {
        // HTTPS Tunneling
        if let Some(host) = req.uri().authority().map(|auth| auth.host()) {
             let host = host.to_string();
             tokio::task::spawn(async move {
                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, host, src_port).await {
                            if is_benign_error(&e) {
                                debug!("server connection benign error: {}", e);
                            } else {
                                error!("server connection error: {}", e);
                            }
                        };
                    }
                    Err(e) => error!("upgrade error: {}", e),
                }
            });
            Ok(Response::new(Full::new(Bytes::new())))
        } else {
             Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Full::new(Bytes::new()))
                .unwrap())
        }
    } else {
        // Plain HTTP Proxy
        let _host = req.uri().host().map(|h| h.to_string()).unwrap_or_default();
        let _path = req.uri().path().to_string();
        let _method = req.method().to_string();
        
        // Log request (Simplified for plain HTTP)
        // Note: Real implementation needs to forward to upstream.
        // For now, we return 501 Not Implemented as placeholder or simple echo.
        // To be fully functional, needs a Client to forward.
        
        // Let's implement simple forwarding
        handle_http_request(req, src_port).await
    }
}

async fn tunnel(upgraded: hyper::upgrade::Upgraded, host: String, src_port: u16) -> std::io::Result<()> {
    let server_config = {
        let mut proxy = MITM_PROXY.lock().unwrap();
        proxy.generate_cert_for_domain(&host)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
    };

    let acceptor = TlsAcceptor::from(server_config);
    let upgraded = TokioIo::new(upgraded);
    
    let tls_stream = acceptor.accept(upgraded).await?;
    let io = TokioIo::new(tls_stream);

    // Now we are inside the TLS tunnel, serving plaintext HTTP requests from the client
    // We need to act as the server for the client, and as a client for the real upstream.
    
    let service = service_fn(move |req| {
        let host = host.clone();
        handle_proxied_request(req, host, src_port)
    });

    if let Err(_err) = http1::Builder::new()
        .preserve_header_case(true)
        .title_case_headers(true)
        .serve_connection(io, service)
        .await
    {
        // Suppress known benign errors
        // error!("Failed to serve upgraded connection: {:?}", err); 
    }
    Ok(())
}

async fn handle_proxied_request(req: Request<Incoming>, domain: String, src_port: u16) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    // 1. Log the request
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    
    // Read Body 
    let (parts, body) = req.into_parts();
    let body_bytes = body.collect().await?.to_bytes();
    
    let log_payload = {
        let mut proxy = MITM_PROXY.lock().unwrap();
        proxy.prepare_audit_log(&domain, &method, &path, &body_bytes, src_port).unwrap_or(None)
    };
    
    if let Some(data) = log_payload {
        tokio::spawn(async move {
            use tokio::io::AsyncWriteExt;
            let socket_path = "/tmp/mac_monitor_audit.sock";
            match tokio::net::UnixStream::connect(socket_path).await {
                Ok(mut stream) => {
                    if let Err(e) = stream.write_all(&data).await {
                        error!("Failed to write to audit IPC: {}", e);
                    }
                },
                Err(e) => {
                    debug!("Audit IPC not available: {}", e);
                }
            }
        });
    }
    
    // 2. Forward to Upstream
    let mut root_store = RootCertStore::empty();
    root_store.extend(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned()
    );
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let connector = TlsConnector::from(Arc::new(config));

    let upstream_stream = TcpStream::connect(format!("{}:443", domain)).await?;
    let domain_host = ServerName::try_from(domain.as_str())
         .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?
         .to_owned();

    let upstream_tls = connector.connect(domain_host, upstream_stream).await?;
    
    let (mut sender, conn) = hyper::client::conn::http1::handshake(TokioIo::new(upstream_tls)).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            error!("Connection failed: {:?}", err);
        }
    });

    // Reconstruct request
    let mut builder = Request::builder()
        .method(parts.method)
        .uri(parts.uri)
        .version(parts.version);
    
    for (k, v) in parts.headers {
        if let Some(key) = k {
            builder = builder.header(key, v);
        }
    }
    
    let upstream_req = builder.body(Full::new(body_bytes))?;

    let res = sender.send_request(upstream_req).await?;
    
    let (res_parts, res_body) = res.into_parts();
    let res_bytes = res_body.collect().await?.to_bytes();
    
    Ok(Response::from_parts(res_parts, Full::new(res_bytes)))
}

async fn handle_http_request(req: Request<Incoming>, src_port: u16) -> Result<Response<Full<Bytes>>, hyper::Error> {
    let path = req.uri().path();
    
    // Check for /ssl
    if path == "/ssl" || path.ends_with("/ssl") {
         let proxy = MITM_PROXY.lock().unwrap();
         let pem = proxy.get_root_ca_pem();
         return Ok(Response::builder()
            .header("Content-Type", "application/x-pem-file")
            .header("Content-Disposition", "attachment; filename=\"mac-monitor-ca.pem\"")
            .body(Full::new(Bytes::from(pem)))
            .unwrap());
    }

    // 1. Extract Host and Port
    let host = req.uri().host().unwrap_or("127.0.0.1").to_string();
    let port = req.uri().port_u16().unwrap_or(80);
    let method = req.method().clone();
    let uri = req.uri().clone();

    // 2. Audit Log (Plain HTTP)
    let (parts, body) = req.into_parts();
    let body_bytes = body.collect().await?.to_bytes();
    
    let path_str = uri.path().to_string();
    let log_payload = {
        let mut proxy = MITM_PROXY.lock().unwrap();
        proxy.prepare_audit_log(&host, method.as_str(), &path_str, &body_bytes, src_port).unwrap_or(None)
    };
    
    if let Some(data) = log_payload {
        tokio::spawn(async move {
            let socket_path = "/tmp/mac_monitor_audit.sock";
            use tokio::io::AsyncWriteExt;
            if let Ok(mut stream) = tokio::net::UnixStream::connect(socket_path).await {
                let _ = stream.write_all(&data).await;
            }
        });
    }

    // 3. Forward to Upstream
    match TcpStream::connect(format!("{}:{}", host, port)).await {
        Ok(stream) => {
            let (mut sender, conn) = hyper::client::conn::http1::handshake(TokioIo::new(stream)).await?;
            tokio::task::spawn(async move {
                if let Err(e) = conn.await {
                    debug!("HTTP connection error: {}", e);
                }
            });

            let mut builder = Request::builder()
                .method(method)
                .uri(uri)
                .version(parts.version);
            for (k, v) in parts.headers {
                if let Some(key) = k {
                    builder = builder.header(key, v);
                }
            }
            
            let upstream_req = builder.body(Full::new(body_bytes)).unwrap();
            let res = sender.send_request(upstream_req).await?;
            
            let (res_parts, res_body) = res.into_parts();
            let res_bytes = res_body.collect().await?.to_bytes();
            Ok(Response::from_parts(res_parts, Full::new(res_bytes)))
        }
        Err(e) => {
            Ok(Response::builder()
                .status(StatusCode::BAD_GATEWAY)
                .body(Full::new(Bytes::from(format!("Failed to connect to {}: {}", host, e))))
                .unwrap())
        }
    }
}

fn is_benign_error(e: &dyn std::error::Error) -> bool {
    let s = e.to_string();
    s.contains("tls handshake eof") || 
    s.contains("SignatureAlgorithmsExtensionRequired") ||
    s.contains("CertificateUnknown") ||
    s.contains("UnknownCA") ||
    s.contains("broken pipe") ||
    s.contains("connection reset") ||
    s.contains("NotConnected")
}

fn init_config() {
    // Load device info etc.
    if let Err(e) = crate::mitm::update_device_info_from_file("/tmp/mac_monitor_device_info.json") {
        info!("Device info file not found or invalid ({}), using defaults.", e);
        let _ = crate::mitm::set_device_info(crate::mitm::DeviceInfo {
            ip: "127.0.0.1".into(),
            mac: "00:00:00:00:00:00".into(),
            pin_number: "unknown".into(),
            cpe_id: "unknown".into(),
            host_id: "unknown".into(),
        });
    } else {
        info!("Loaded device info from /tmp/mac_monitor_device_info.json");
    }
    // Policy defaults to empty which logs everything if no whitelists
}
