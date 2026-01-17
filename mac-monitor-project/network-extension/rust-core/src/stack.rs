use crate::device::TunDevice;
use crate::dns::DnsServer;
use crate::mitm::MitmProxy;
use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::socket::tcp;
use smoltcp::time::Instant;
use smoltcp::wire::IpAddress;
use std::sync::{Arc, Mutex};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct NetworkStack {
    pub device: TunDevice,
    pub interface: Interface,
    pub sockets: SocketSet<'static>,
    pub mitm_proxy: Arc<Mutex<MitmProxy>>,
    pub dns_server: DnsServer,
    pub active_connections: std::collections::HashSet<smoltcp::iface::SocketHandle>,
}

impl NetworkStack {
    pub fn new() -> Result<StackHandle> {
        let mut device = TunDevice::new(1500);
        let mut sockets = SocketSet::new(vec![]);

        let config = Config::new(smoltcp::wire::HardwareAddress::Ip);
        let interface = Interface::new(config, &mut device, Instant::now());

        let dns_server = DnsServer::new(&mut sockets);
        let mitm_proxy = Arc::new(Mutex::new(MitmProxy::new()));

        let stack = Arc::new(Mutex::new(NetworkStack {
            device: device.clone(),
            interface,
            sockets,
            mitm_proxy,
            dns_server,
            active_connections: std::collections::HashSet::new(),
        }));

        Ok(StackHandle { device, stack })
    }

    pub fn poll(&mut self) {
        let timestamp = Instant::now();
        self.interface
            .poll(timestamp, &mut self.device, &mut self.sockets);
        self.dns_server.handle_queries(&mut self.sockets);

        // TCP 拦截与重定向逻辑
        self.handle_tcp_intercept();
    }

    fn handle_tcp_intercept(&mut self) {
        // 查找所有处于 SYN-RECEIVED 状态的连接并接受它们
        // 这里简化实现：检测到的任何 443 端口连接都会通过 MITM 处理
        
        // 我们需要通过 handle 列表来避免在迭代时借用错误
        let handles: Vec<_> = self.sockets.iter().map(|(handle, _)| handle).collect();

        for handle in handles {
            let socket = self.sockets.get_mut::<tcp::Socket>(handle);
            if socket.is_active() && socket.may_recv() {
                let mut data = vec![0u8; 4096];
                if let Ok(len) = socket.recv_slice(&mut data) {
                    if len > 0 {
                        let remote_endpoint = socket.remote_endpoint().unwrap();
                        let domain = self.dns_server
                            .get_domain_by_ip(&remote_endpoint.addr)
                            .map(|n| n.to_string())
                            .unwrap_or_else(|| remote_endpoint.addr.to_string());
                        
                        log::info!("Intercepted traffic for {}: {}", domain, len);
                        
                        let mut proxy = self.mitm_proxy.lock().unwrap();
                        // 调用 MITM 逻辑进行审计上报
                        if let Ok(_) = proxy.handle_https_connection(&domain, &data[..len], remote_endpoint.port) {
                            // 审计逻辑处理完成后，数据透传（此处为简化版实现）
                        }
                    }
                }
            }
        }
    }
}

pub struct StackHandle {
    pub device: TunDevice,
    pub stack: Arc<Mutex<NetworkStack>>,
}

impl StackHandle {
    pub fn process_inbound_packet(&self, data: &[u8]) -> Result<()> {
        self.device.receive_packet(data);
        Ok(())
    }

    pub fn get_outbound_packet(&self, buffer: &mut [u8]) -> Result<usize> {
        if let Some(packet) = self.device.get_tx_packet() {
            let len: usize = packet.len();
            if len <= buffer.len() {
                buffer[..len].copy_from_slice(&packet);
                Ok(len)
            } else {
                Err("Buffer too small".into())
            }
        } else {
            Err("No packet available".into())
        }
    }

    pub fn poll(&self) {
        if let Ok(mut stack) = self.stack.lock() {
            stack.poll();
        }
    }
}