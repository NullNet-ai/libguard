mod control_connection;
mod control_connection_manager;
mod profile;
mod profile_manager;

pub use profile::*;

use crate::{protocol, str_hash, Hash, Message, Payload};
use control_connection_manager::ControlConnectionManager;
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use profile_manager::ProfileManager;
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub struct Server {
    bind_addr: SocketAddr,
    profile_manager: Arc<Mutex<ProfileManager>>,
    connections_manager: Arc<Mutex<ControlConnectionManager>>,
    heartbeat_interval: Option<Duration>,
}

impl Server {
    pub fn new(bind_addr: SocketAddr, heartbeat_interval: Option<Duration>) -> Self {
        let profile_manager = Arc::new(Mutex::new(ProfileManager::new()));
        let connections_manager = Arc::new(Mutex::new(ControlConnectionManager::new()));

        Self {
            bind_addr,
            profile_manager,
            connections_manager,
            heartbeat_interval,
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        let listener = TcpListener::bind(self.bind_addr)
            .await
            .handle_err(location!())?;

        loop {
            let (mut stream, addr) = listener.accept().await.handle_err(location!())?;
            log::debug!("Client connected from {}", addr);

            let Ok(message) = protocol::expect_open_message(&mut stream).await else {
                log::error!("Unexpected opening message, aborting connection ...");
                continue;
            };

            match message {
                Message::ControlConnectionRequest(payload) => {
                    self.on_control_connection_established(stream, payload)
                        .await;
                }
                Message::DataConnectionRequest(payload) => {
                    self.on_data_connection_established(stream, payload).await;
                }
                _ => {
                    log::error!("Unexpected opening message, aborting connection ...");
                    continue;
                }
            }
        }
    }

    async fn on_control_connection_established(&mut self, mut stream: TcpStream, payload: Payload) {
        let profile_manager = self.profile_manager.clone();
        let connections_manager = self.connections_manager.clone();
        let heartbeat_interval = self.heartbeat_interval;

        tokio::spawn(async move {
            let client_id_hash = payload.data;

            if let Some(profile) = profile_manager.lock().await.get(&client_id_hash) {
                match protocol::write_message(&mut stream, Message::Acknowledgment).await {
                    Ok(_) => {
                        connections_manager
                            .lock()
                            .await
                            .open_connection(stream, &profile, heartbeat_interval)
                            .await;
                    }
                    Err(err) => {
                        log::error!("Failed to send Acknowledgment message. {}", err.to_str())
                    }
                };
            } else if let Err(err) = protocol::write_message(&mut stream, Message::Rejection).await
            {
                log::error!("Failed to send Rejection message. {}", err.to_str());
            }
        });
    }

    async fn on_data_connection_established(&mut self, mut stream: TcpStream, payload: Payload) {
        let connections_manager = self.connections_manager.clone();

        tokio::spawn(async move {
            let control_id: Hash = payload.data;

            if connections_manager.lock().await.exists(&control_id) {
                match protocol::write_message(&mut stream, Message::Acknowledgment).await {
                    Ok(_) => {
                        // @TODO: Since open_data_channel would block in an attempt
                        // to receive a visitor stream, we might want ot add timeout
                        // or improve the API to not hold the lock on connections_manager
                        if let Err(err) = connections_manager
                            .lock()
                            .await
                            .open_data_channel(&control_id, stream)
                            .await
                        {
                            log::error!("Failed to open data channel, {}", err.to_str());
                        }
                    }
                    Err(err) => {
                        log::error!("Failed to send Acknowledgment message. {}", err.to_str());
                    }
                };
            } else if let Err(err) = protocol::write_message(&mut stream, Message::Rejection).await
            {
                log::error!("Failed to send Rejection message. {}", err.to_str());
            }
        });
    }

    pub async fn register_profile(&mut self, profile: ClientProfile) -> Result<(), Error> {
        self.profile_manager.lock().await.register(profile)
    }

    pub async fn remove_profile(&mut self, id: &str) -> Result<(), Error> {
        let hash = str_hash(id);

        self.profile_manager.lock().await.remove(&hash);
        self.connections_manager.lock().await.remove(&hash).await?;

        Ok(())
    }
}
