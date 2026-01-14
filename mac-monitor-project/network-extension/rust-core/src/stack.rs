use crate::device::TunDevice;
use crate::mitm::MitmProxy;
use smoltcp::iface::{Config, Interface, SocketSet};
use smoltcp::socket::{tcp, udp};
use smoltcp::time::Instant;
use smoltcp::wire::{IpAddress, IpCidr, Ipv4Address};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct NetworkStack {
    device: TunDevice,
    interface: Interface,
    sockets: SocketSet<'static>,
    mitm_proxy: Arc<Mutex<MitmProxy>>,
}

pub struct StackHandle {
    inner: Arc<Mutex<NetworkStack>>,
}

impl NetworkStack {
    pub fn new() -> Result<StackHandle> {
        let device = TunDevice::new(1500);

        let config = Config::new(smoltcp::wire::HardwareAddress::Ip);
        let mut interface = Interface::new(config, &mut device.clone(), Instant::now());

        // 配置接口地址 (10.0.0.1/24)
        interface.update_ip_addrs(|ip_addrs| {
            ip_addrs
                .push(IpCidr::new(IpAddress::v4(10, 0, 0, 1), 24))
                .unwrap();
        });

        // 配置默认路由
        interface
            .routes_mut()
            .add_default_ipv4_route(Ipv4Address::new(10, 0, 0, 1))
            .unwrap();

        let sockets = SocketSet::new(vec![]);
        let mitm_proxy = Arc::new(Mutex::new(MitmProxy::new()));

        let stack = NetworkStack {
            device,
            interface,
            sockets,
            mitm_proxy,
        };

        Ok(StackHandle {
            inner: Arc::new(Mutex::new(stack)),
        })
    }
}

impl StackHandle {
    pub fn process_inbound_packet(&self, data: &[u8]) -> Result<()> {
        let stack = self.inner.lock().unwrap();
        stack.device.receive_packet(data);
        Ok(())
    }

    pub fn get_outbound_packet(&self, buffer: &mut [u8]) -> Result<usize> {
        let stack = self.inner.lock().unwrap();
        if let Some(packet) = stack.device.get_tx_packet() {
            let len = packet.len().min(buffer.len());
            buffer[..len].copy_from_slice(&packet[..len]);
            Ok(len)
        } else {
            Ok(0)
        }
    }

    pub fn poll(&self) {
        let mut stack = self.inner.lock().unwrap();
        let timestamp = Instant::now();

        let NetworkStack {
            ref mut interface,
            ref mut device,
            ref mut sockets,
            ..
        } = *stack;

        interface.poll(timestamp, device, sockets);

        // 处理TCP连接和MitM逻辑
        // TODO: 遍历活跃的TCP socket，检测HTTPS流量并交给MitmProxy处理
    }

    pub fn create_tcp_socket(&self) -> tcp::Socket<'static> {
        let rx_buffer = tcp::SocketBuffer::new(vec![0; 65536]);
        let tx_buffer = tcp::SocketBuffer::new(vec![0; 65536]);
        tcp::Socket::new(rx_buffer, tx_buffer)
    }

    pub fn create_udp_socket(&self) -> udp::Socket<'static> {
        let rx_meta = udp::PacketMetadata::EMPTY;
        let tx_meta = udp::PacketMetadata::EMPTY;
        let rx_buffer = udp::PacketBuffer::new(vec![rx_meta; 64], vec![0; 65536]);
        let tx_buffer = udp::PacketBuffer::new(vec![tx_meta; 64], vec![0; 65536]);
        udp::Socket::new(rx_buffer, tx_buffer)
    }
}
