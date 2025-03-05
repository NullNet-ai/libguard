use nullnet_liberror::{location, Error, ErrorHandler, Location};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

use super::profile::ClientProfile;
use crate::{protocol, Message};

const VISITORS_CHANNEL_SIZE: usize = 0x400;

pub struct ControlConnection {
    pub(crate) handle: JoinHandle<()>,
    pub(crate) visitor_rx: mpsc::Receiver<TcpStream>,
    pub(crate) shutdown_tx: broadcast::Sender<()>,
}

impl ControlConnection {
    pub fn new(
        stream: TcpStream,
        profile: &ClientProfile,
        heartbeat_interval: Option<Duration>,
    ) -> Self {
        log::info!("Opening new control connection for clinet {}", profile.id);

        let (shutdown_tx, shutdown_rx) = broadcast::channel(1);
        let (visitor_tx, visitor_rx) = mpsc::channel(VISITORS_CHANNEL_SIZE);

        let addr = profile.visitor_addr;
        let handle = tokio::spawn(async move {
            Self::run(stream, addr, visitor_tx, shutdown_rx, heartbeat_interval).await
        });

        Self {
            handle,
            visitor_rx,
            shutdown_tx,
        }
    }

    pub async fn shutdown(self) -> Result<(), Error> {
        let _ = self.shutdown_tx.send(()).handle_err(location!());
        self.handle.await.handle_err(location!())
    }

    pub async fn open_data_channel(&mut self, mut client_stream: TcpStream) -> Result<(), Error> {
        log::debug!("Opening data channel, waiting for a visitor ...");
        let visitor = self.visitor_rx.recv().await;

        if visitor.is_none() {
            return Err("Failed to receive a visitor").handle_err(location!());
        }

        let mut visitor_stream = visitor.unwrap();

        tokio::spawn(async move {
            log::debug!("Data channel established");
            let _ = copy_bidirectional(&mut client_stream, &mut visitor_stream).await;
        });
        Ok(())
    }

    async fn run(
        stream: TcpStream,
        addr: SocketAddr,
        visitor_tx: mpsc::Sender<TcpStream>,
        mut shutdown_rx: broadcast::Receiver<()>,
        heartbeat_interval: Option<Duration>,
    ) {
        tokio::select! {
            _ = shutdown_rx.recv() => {
                log::debug!("Control connection received a shutdown signal");
            },
            result = Self::run_control_connection(stream, addr, visitor_tx, heartbeat_interval) => {
                if let Err(error) = result {
                    log::error!("Control connection error: {}", error.to_str())
                }
            }
        }

        log::info!("Control connection is terminated");
        // @TODO: Notify the manager
    }

    async fn run_control_connection(
        mut stream: TcpStream,
        addr: SocketAddr,
        visitor_tx: mpsc::Sender<TcpStream>,
        heartbeat_interval: Option<Duration>,
    ) -> Result<(), Error> {
        let listener = TcpListener::bind(addr).await.handle_err(location!())?;

        let heartbeat_interval = heartbeat_interval.map(|dur| dur.as_secs()).unwrap_or(0);

        loop {
            tokio::select! {
                visitor_result = listener.accept() => {
                    let (visitor, addr) = visitor_result.handle_err(location!())?;
                    log::debug!("Accepted visitor from: {}", addr);
                    protocol::write_message(&mut stream, Message::ForwardConnectionRequest).await?;
                    visitor_tx.send(visitor).await.handle_err(location!())?;
                },
                _ = tokio::time::sleep(Duration::from_secs(heartbeat_interval)), if heartbeat_interval > 0 => {
                    log::debug!("Sending heartbeat");
                    protocol::write_message(&mut stream, Message::Heartbeat).await?;
                }
            }
        }
    }
}
