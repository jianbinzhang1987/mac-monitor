use std::collections::HashMap;
use smoltcp::socket::udp::{self, Socket};
use smoltcp::wire::{IpAddress, Ipv4Address};
use hickory_proto::op::{Message, Query, ResponseCode};
use hickory_proto::rr::{Name, RData, Record, RecordType};
use hickory_proto::serialize::binary::{BinDecodable, BinDecoder, BinEncodable, BinEncoder};
use smoltcp::iface::SocketHandle;
use smoltcp::iface::SocketSet;
use smoltcp::wire::IpEndpoint;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct DnsServer {
    handle: SocketHandle,
    // Fake IP -> Real Domain
    forward_map: HashMap<IpAddress, Name>,
    // Real Domain -> Fake IP
    reverse_map: HashMap<Name, IpAddress>,
    next_fake_ip: Ipv4Address,
}

impl DnsServer {
    pub fn new(sockets: &mut SocketSet<'static>) -> Self {
        let rx_meta = udp::PacketMetadata::EMPTY;
        let tx_meta = udp::PacketMetadata::EMPTY;
        let rx_buffer = udp::PacketBuffer::new(vec![rx_meta; 64], vec![0; 4096]);
        let tx_buffer = udp::PacketBuffer::new(vec![tx_meta; 64], vec![0; 4096]);
        let mut socket = Socket::new(rx_buffer, tx_buffer);
        socket.bind(53).unwrap();
        let handle = sockets.add(socket);

        Self {
            handle,
            forward_map: HashMap::new(),
            reverse_map: HashMap::new(),
            next_fake_ip: Ipv4Address::new(10, 0, 1, 1),
        }
    }
    
    fn new_fake_ip(&mut self) -> IpAddress {
        let ip = self.next_fake_ip;
        self.next_fake_ip.0[3] += 1;
        // TODO: Handle IP exhaustion
        IpAddress::from(ip)
    }

    pub fn handle_queries(&mut self, sockets: &mut SocketSet) {
        let socket = sockets.get_mut::<udp::Socket>(self.handle);
        while let Ok((data, endpoint)) = socket.recv() {
            let mut decoder = BinDecoder::new(data);
            if let Ok(message) = Message::read(&mut decoder) {
                if let Some(query) = message.queries().get(0) {
                    if let Some(response) = self.build_response(&message, query) {
                        let mut buffer = vec![0; 512];
                        let mut encoder = BinEncoder::new(&mut buffer);
                        response.emit(&mut encoder).unwrap();
                        let len = encoder.offset();
                        socket.send_slice(&buffer[..len], endpoint).unwrap();
                    }
                }
            }
        }
    }

    fn build_response(&mut self, request: &Message, query: &Query) -> Option<Message> {
        if query.query_class() != hickory_proto::rr::DNSClass::IN || query.query_type() != RecordType::A {
            // We only handle IN class A queries for now
            return None;
        }

        let mut response = Message::new();
        response.set_id(request.id());
        response.set_message_type(hickory_proto::op::MessageType::Response);
        response.set_recursion_available(true);
        response.add_query(query.clone());
        response.set_response_code(ResponseCode::NoError);

        let name = query.name();
        let fake_ip = if let Some(ip) = self.reverse_map.get(name) {
            *ip
        } else {
            let ip = self.new_fake_ip();
            self.reverse_map.insert(name.clone(), ip);
            self.forward_map.insert(ip, name.clone());
            ip
        };

        let ipv4_addr = if let IpAddress::Ipv4(v4) = fake_ip {
            std::net::Ipv4Addr::from(v4.0)
        } else {
            std::net::Ipv4Addr::UNSPECIFIED
        };
        let record = Record::from_rdata(name.clone(), 60, RData::A(hickory_proto::rr::rdata::A(ipv4_addr)));
        response.add_answer(record);

        Some(response)
    }

    pub fn get_domain_by_ip(&self, ip: &IpAddress) -> Option<&Name> {
        self.forward_map.get(ip)
    }
}