use smoltcp::phy::{self, Device, DeviceCapabilities, Medium};
use smoltcp::time::Instant;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// TUN设备抽象，用于与Swift NetworkExtension交互
#[derive(Clone)]
pub struct TunDevice {
    rx_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    tx_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    mtu: usize,
}

impl TunDevice {
    pub fn new(mtu: usize) -> Self {
        Self {
            rx_queue: Arc::new(Mutex::new(VecDeque::new())),
            tx_queue: Arc::new(Mutex::new(VecDeque::new())),
            mtu,
        }
    }

    /// 接收来自Swift的入站数据包
    pub fn receive_packet(&self, data: &[u8]) {
        let mut queue = self.rx_queue.lock().unwrap();
        queue.push_back(data.to_vec());
    }

    /// 获取要发送到Swift的出站数据包
    pub fn get_tx_packet(&self) -> Option<Vec<u8>> {
        let mut queue = self.tx_queue.lock().unwrap();
        queue.pop_front()
    }

    pub fn rx_queue(&self) -> Arc<Mutex<VecDeque<Vec<u8>>>> {
        self.rx_queue.clone()
    }

    pub fn tx_queue(&self) -> Arc<Mutex<VecDeque<Vec<u8>>>> {
        self.tx_queue.clone()
    }
}

pub struct RxToken {
    buffer: Vec<u8>,
}

impl phy::RxToken for RxToken {
    fn consume<R, F>(mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        f(&mut self.buffer)
    }
}

pub struct TxToken {
    queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
}

impl phy::TxToken for TxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        let mut buffer = vec![0u8; len];
        let result = f(&mut buffer);

        let mut queue = self.queue.lock().unwrap();
        queue.push_back(buffer);

        result
    }
}

impl Device for TunDevice {
    type RxToken<'a> = RxToken where Self: 'a;
    type TxToken<'a> = TxToken where Self: 'a;

    fn receive(&mut self, _timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        let mut rx_queue = self.rx_queue.lock().unwrap();

        if let Some(buffer) = rx_queue.pop_front() {
            let rx = RxToken { buffer };
            let tx = TxToken {
                queue: self.tx_queue.clone(),
            };
            Some((rx, tx))
        } else {
            None
        }
    }

    fn transmit(&mut self, _timestamp: Instant) -> Option<Self::TxToken<'_>> {
        Some(TxToken {
            queue: self.tx_queue.clone(),
        })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = self.mtu;
        caps.medium = Medium::Ip;
        caps
    }
}
