use tokio::net::TcpStream;

use super::{control_connection::ControlConnection, profile::ClientProfile};
use std::collections::HashMap;

pub struct ControlConnectionManager {
    connections: HashMap<String, ControlConnection>,
}

impl ControlConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub async fn open_connection(&mut self, stream: TcpStream, profile: &ClientProfile) {
        let connection = ControlConnection::new(stream, profile);
        if let Some(_prev) = self.connections.insert(profile.id.clone(), connection) {
            // ????????????????????????????
            // ?? _prev.shutdown().await ??
            // ????????????????????????????
            todo!("Not implemented");
        }
    }
}
