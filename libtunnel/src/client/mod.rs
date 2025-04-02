mod config;
mod proto;

use crate::{Message, str_hash};
pub use config::*;
use nullnet_liberror::{Error, ErrorHandler, Location, location};
use proto::{await_channel_request, request_open_channel, request_open_session};
use std::time::Duration;
use tokio::{io::copy_bidirectional, net::TcpStream, sync::oneshot, task::JoinHandle};

pub struct Client {
    shutdown_tx: oneshot::Sender<()>,
    handle: JoinHandle<()>,
}

impl Client {
    /// Creates a new client instance, initializing the client and starting the main event loop.
    ///
    /// # Parameters
    /// - `config`: Configuration object containing the necessary details for the client.
    ///
    /// # Returns
    /// A new `Client` instance.
    pub fn new(config: ClientConfig) -> Self {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let handle = tokio::spawn(main_loop(config, shutdown_rx));

        Self {
            shutdown_tx,
            handle,
        }
    }

    /// Shuts down the client by sending a shutdown signal and waiting for the task to complete.
    ///
    /// # Returns
    /// A future that resolves once the client has successfully shut down.
    pub async fn shutdown(self) {
        if self.shutdown_tx.send(()).is_ok() {
            let _ = self.handle.await;
        } else {
            self.handle.abort();
        }
    }
}

/// Main event loop for the client, handling session management and shutdown.
async fn main_loop(config: ClientConfig, shutdown_rx: oneshot::Receiver<()>) {
    tokio::select! {
        _ = async {
            loop {
                let result = launch_session(config.clone()).await;
                if let Err(err) = result {
                    let timeout = config.reconnect_timeout.unwrap_or(Duration::from_secs(10));
                    log::error!("Client: Session error {}. Reconnecting in {} seconds ...", err.to_str(), timeout.as_secs());

                    tokio::time::sleep(timeout).await;
                    continue;
                }
            }
        } => {},

        _ = shutdown_rx => {
            log::debug!("Client received shutdown signal");
        }
    }
}

/// Launches a new session with the server, handling connection and communication.
async fn launch_session(config: ClientConfig) -> Result<(), Error> {
    log::debug!("Client: Requesting a session from the server");

    let mut server_stream = TcpStream::connect(&config.server_addr)
        .await
        .handle_err(location!())?;

    request_open_session(&mut server_stream, str_hash(&config.id)).await?;

    loop {
        match await_channel_request(&mut server_stream).await {
            Ok(Message::ForwardConnectionRequest) => {
                let config = config.clone();
                tokio::spawn(async move {
                    log::debug!("Client: Received ForwardConnectionRequest message");
                    match launch_data_channel(config).await {
                        Ok(_) => log::debug!("Client: Data channel terminated"),
                        Err(err) => log::error!("Client: Data channel error: {err:?}"),
                    }
                });
            }
            Ok(Message::Heartbeat) => tokio::task::yield_now().await,
            Err(err) => {
                Err(err)?;
            }
            Ok(_) => {
                Err("Client: Unexpected message, terminating session").handle_err(location!())?;
            }
        }
    }
}

/// Launches a data channel between the client and the server.
async fn launch_data_channel(config: ClientConfig) -> Result<(), Error> {
    log::debug!("Client: Launching data channel");

    let mut server_stream = TcpStream::connect(&config.server_addr)
        .await
        .handle_err(location!())?;

    let hash = str_hash(&config.id);
    request_open_channel(&mut server_stream, hash).await?;

    let mut local_stream = TcpStream::connect(&config.local_addr)
        .await
        .handle_err(location!())?;

    copy_bidirectional(&mut server_stream, &mut local_stream)
        .await
        .handle_err(location!())?;

    Ok(())
}
