use super::control_connection::ControlConnection;
use super::control_connection_manager::ControlConnectionManager;
use super::{profile::ClientProfile, profile_manager::ProfileManager};
use crate::{protocol, Message, Payload};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub struct Server {
    bind_addr: SocketAddr,
    profile_manager: Arc<Mutex<ProfileManager>>,
    connections_manager: Arc<Mutex<ControlConnectionManager>>,
}

impl Server {
    pub async fn run(&mut self) -> Result<(), Error> {
        let listener = TcpListener::bind(self.bind_addr)
            .await
            .handle_err(location!())?;

        loop {
            let (mut stream, addr) = listener.accept().await.handle_err(location!())?;
            log::info!("Client connected from {}", addr);

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
                    self.on_data_connection_established(payload).await;
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

        tokio::spawn(async move {
            // @TODO: Parse payload
            let client_id = String::from("");

            if let Some(profile) = profile_manager.lock().await.get(&client_id) {
                match protocol::write_message(&mut stream, Message::Acknowledgment).await {
                    Ok(_) => {
                        connections_manager
                            .lock()
                            .await
                            .open_connection(stream, &profile)
                            .await;
                    }
                    Err(err) => {
                        log::error!("Failed to sernd Acknowledgment message. {}", err.to_str())
                    }
                };
            } else {
                match protocol::write_message(&mut stream, Message::Rejection).await {
                    Err(err) => {
                        log::error!("Failed to sernd Rejection message. {}", err.to_str())
                    }
                    _ => {}
                };
            }
        });
    }

    async fn on_data_connection_established(&mut self, payload: Payload) {}

    pub async fn register_profile(&mut self, profile: ClientProfile) -> Result<(), Error> {
        self.profile_manager.lock().await.register(profile)
    }

    pub async fn remove_profile(&mut self, id: &str) -> Result<(), Error> {
        // @TODO: After profile has been removed, we need to shutdown open channels
        self.profile_manager.lock().await.remove(id)
    }
}
