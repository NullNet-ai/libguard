mod config;
use std::time::Duration;

pub use config::*;

use crate::{protocol, str_hash, Hash, Message, Payload};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use tokio::{io::copy_bidirectional, net::TcpStream, sync::broadcast};

pub struct Client {
    config: Config,
    shutdown_tx: broadcast::Sender<()>,
    shutdown_rx: broadcast::Receiver<()>,
}

impl Client {
    pub fn new(config: Config) -> Self {
        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        Self {
            config,
            shutdown_rx,
            shutdown_tx,
        }
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        self.shutdown_tx
            .send(())
            .map_err(|_| "Failed to send shutdown signal")
            .handle_err(location!())?;

        Ok(())
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                result = Self::run_control_connection(self.config.clone()) => {
                    if let Err(_) = result {
                        let timeout = self.config.reconnect_timeout.unwrap_or(Duration::from_secs(10));
                        tokio::time::sleep(timeout).await;
                        continue;
                    }
                },
                _ = self.shutdown_rx.recv() => {
                    break;
                }
            };
        }

        log::info!("libtunnel::Client has terminated");
    }

    async fn run_control_connection(config: Config) -> Result<(), Error> {
        log::info!("Requesting control connection from the server");

        let mut server_stream = TcpStream::connect(&config.server_addr)
            .await
            .handle_err(location!())?;

        let hash = str_hash(&config.id);
        Self::request_open_control_connection(&mut server_stream, hash).await?;

        log::info!("Control connection established");

        loop {
            tokio::select! {
                _ = tokio::time::sleep(config.heartbeat_timeout.unwrap()), if config.heartbeat_timeout.is_some() => {
                    Err("Heartbeat interval has reached").handle_err(location!())?;
                }
                message_result = Self::await_for_control_channel_message(&mut server_stream) => {
                    match message_result {
                        Ok(Message::ForwardConnectionRequest) => {
                            let config = config.clone();
                            tokio::spawn(async move {
                                log::info!("Received ForwardConnectionRequest message");
                                match Self::run_data_connection(config).await {
                                    Ok(_) => log::info!("Data connection terminated"),
                                    Err(err) => log::error!("Data connection error: {err:?}"),
                                }
                            });
                        }
                        Ok(Message::Heartbeat) => {
                            log::info!("Received Heartbeat message");
                        }
                        Err(err) => {
                            log::error!("Error happened when waiting for control connection message. {err:?}");
                            Err(err)?;
                        }
                        Ok(_) => {
                            Err("Unexpected message").handle_err(location!())?;
                        }
                    }
                }
            };
        }
    }

    async fn run_data_connection(config: Config) -> Result<(), Error> {
        log::info!("Requesting data connection from the server");

        let mut server_stream = TcpStream::connect(&config.server_addr)
            .await
            .handle_err(location!())?;

        let hash = str_hash(&config.id);
        Self::request_open_data_connection(&mut server_stream, hash).await?;

        let mut local_stream = TcpStream::connect(&config.local_addr)
            .await
            .handle_err(location!())?;

        copy_bidirectional(&mut server_stream, &mut local_stream)
            .await
            .handle_err(location!())?;

        Ok(())
    }

    async fn request_open_control_connection(
        stream: &mut TcpStream,
        hash: Hash,
    ) -> Result<(), Error> {
        let open_message = Message::ControlConnectionRequest(Payload { data: hash });
        protocol::write_with_confirmation(stream, open_message).await
    }

    async fn request_open_data_connection(stream: &mut TcpStream, hash: Hash) -> Result<(), Error> {
        let open_message = Message::DataConnectionRequest(Payload { data: hash });
        protocol::write_with_confirmation(stream, open_message).await
    }

    async fn await_for_control_channel_message(stream: &mut TcpStream) -> Result<Message, Error> {
        let message_size = Message::len_bytes(&Message::ForwardConnectionRequest);
        let message = protocol::expect_message(stream, message_size).await?;
        match protocol::expect_message(stream, message_size).await? {
            Message::ForwardConnectionRequest | Message::Heartbeat => Ok(message),
            _ => Err("Unexpected message").handle_err(location!()),
        }
    }
}
