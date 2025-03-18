use std::{net::SocketAddr, time::Duration};

/// Configuration for the client, including details about connection settings and timeouts.
#[derive(Clone, Debug)]
pub struct ClientConfig {
    /// A unique identifier for the client.
    ///
    /// This ID is used for identifying the client during session and channel requests.
    pub id: String,
    /// The address of the server to connect to.
    pub server_addr: SocketAddr,
    /// The local address of the client to bind for incoming connections.
    pub local_addr: SocketAddr,
    /// The timeout duration for reconnecting to the server after a failure.
    pub reconnect_timeout: Option<Duration>,
}
