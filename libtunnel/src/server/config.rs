use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

/// Configuration for the server.
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Server address (IP and port).
    pub addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        let default_ip = Ipv4Addr::from_bits(0);
        let default_port = 9000;
        Self {
            addr: SocketAddr::V4(SocketAddrV4::new(default_ip, default_port)),
        }
    }
}
