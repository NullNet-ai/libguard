use std::{net::SocketAddr, time::Duration};

#[derive(Clone, Debug)]
pub struct Config {
    pub id: String,
    pub server_addr: SocketAddr,
    pub local_addr: SocketAddr,

    pub reconnect_timeout: Option<Duration>,
    pub heartbeat_timeout: Option<Duration>,
}
