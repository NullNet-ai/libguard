use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub id: String,
    pub server_addr: SocketAddr,
    pub local_addr: SocketAddr,
}
