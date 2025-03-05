use std::net::SocketAddr;

/// Represents informatino about a registered client.
#[derive(Debug, Clone)]
pub struct ClientProfile {
    /// A unique identifier assigned to the client.
    pub id: String,
    /// The network address that a visitor can connect to in order to reach the client.
    pub visitor_addr: SocketAddr,
}
