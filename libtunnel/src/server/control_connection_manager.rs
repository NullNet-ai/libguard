use super::{control_connection::ControlConnection, profile::ClientProfile};
use crate::{str_hash, Hash};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use std::collections::HashMap;
use tokio::net::TcpStream;

pub struct ControlConnectionManager {
    connections: HashMap<Hash, ControlConnection>,
}

impl ControlConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub async fn open_connection(&mut self, stream: TcpStream, profile: &ClientProfile) {
        let connection = ControlConnection::new(stream, profile);
        let hash = str_hash(&profile.id);

        if let Some(_prev) = self.connections.insert(hash, connection) {
            // ????????????????????????????
            // ?? _prev.shutdown().await ??
            // ????????????????????????????
            todo!("Not implemented");
        }
    }

    pub async fn open_data_channel(
        &mut self,
        control_id: &Hash,
        stream: TcpStream,
    ) -> Result<(), Error> {
        let control = self.connections.get_mut(control_id);

        if control.is_none() {
            return Err("Failed to open data channel, control connection does not exist")
                .handle_err(location!())?;
        }

        control.unwrap().open_data_channel(stream).await?;

        Ok(())
    }

    pub fn exists(&self, hash: &Hash) -> bool {
        self.connections.contains_key(hash)
    }
}
