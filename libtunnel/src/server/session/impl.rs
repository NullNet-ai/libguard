use super::channel::{Channel, ChannelId};
use crate::{protocol, Message};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot, RwLock},
    task::JoinHandle,
};

/// Manages the lifecycle of channels connecting visitors to clients.
///
/// `Session` handles incoming visitor and client connections, establishes bidirectional
/// communication channels, and ensures proper cleanup during shutdown.
#[derive(Debug)]
pub struct Session {
    /// Channel used to send a shutdown signal to the session.
    shutdown_tx: oneshot::Sender<()>,
    /// Channel used to forward incoming client connections for processing.
    client_stream_tx: mpsc::Sender<TcpStream>,
    /// Shared, thread-safe map of active channels.
    active_channels: Arc<RwLock<HashMap<ChannelId, Channel>>>,
    /// Handle for the background task running the session.
    handle: JoinHandle<()>,
}

impl Session {
    /// Creates a new `Session` instance and starts the control session.
    ///
    /// # Arguments
    ///
    /// * `addr` - The socket address for visitor connections.
    /// * `control_stream` - The TCP stream used for communication with the control entity.
    /// * `channel_idle_timeout`: The timeout duration for idle channels before shutdown.
    ///
    /// # Returns
    ///
    /// A new `Session` instance that manages the channels.
    pub fn new(
        addr: SocketAddr,
        control_stream: TcpStream,
        channel_idle_timeout: Duration,
    ) -> Self {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let (client_stream_tx, client_stream_rx) = mpsc::channel::<TcpStream>(64);

        let channels = Arc::new(RwLock::new(HashMap::new()));

        let handle = tokio::spawn(run_control_session(
            addr,
            control_stream,
            shutdown_rx,
            channels.clone(),
            client_stream_rx,
            channel_idle_timeout,
        ));

        Self {
            shutdown_tx,
            handle,
            active_channels: channels,
            client_stream_tx,
        }
    }

    /// Requests a new channel by sending a client stream.
    ///
    /// # Arguments
    ///
    /// * `client_stream` - The TCP stream representing the client connection.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the request.
    pub async fn request_channel(&self, client_stream: TcpStream) -> Result<(), Error> {
        self.client_stream_tx
            .send(client_stream)
            .await
            .handle_err(location!())
    }

    /// Checks if there are any active channels.
    ///
    /// # Returns
    /// - `true` if there is at least one active channel.
    /// - `false` if there are no active channels.
    pub async fn has_active_channels(&self) -> bool {
        self.active_channels.read().await.len() > 0
    }

    /// Initiates shutdown of the session.
    ///
    /// If the shutdown signal fails to send, the session is forcefully aborted.
    pub async fn shutdown(self) {
        if self.shutdown_tx.send(()).is_ok() {
            let _ = self.handle.await;
        } else {
            log::error!(
                "Visitors Acceptor: Failed to send shutdown signal, forcefully aborting the task"
            );
            self.handle.abort();
        }
    }
}

/// Runs the control session for managing visitor connections, channel lifecycle,
/// and channel creation.
///
/// # Arguments
///
/// * `addr` - The socket address to listen for visitor connections.
/// * `control_stream` - The TCP stream used for communication with the control component.
/// * `shutdown_rx` - A `oneshot::Receiver` that listens for shutdown signals.
/// * `channels` - A shared, thread-safe map storing active channels.
/// * `client_stream_rx` - A receiver for handling incoming client connections.
/// * `channel_idle_timeout`: The timeout duration for idle channels before shutdown.
async fn run_control_session(
    addr: SocketAddr,
    control_stream: TcpStream,
    shutdown_rx: oneshot::Receiver<()>,
    channels: Arc<RwLock<HashMap<ChannelId, Channel>>>,
    client_stream_rx: mpsc::Receiver<TcpStream>,
    channel_idle_timeout: Duration,
) {
    let (channel_complete_tx, channel_complete_rx) = mpsc::channel(64);
    let (visitor_stream_tx, visitor_stream_rx) = mpsc::channel::<TcpStream>(64);

    tokio::select! {
        _ = shutdown_rx => {
            log::debug!("Session: Shutdown signal received");
        },
        _ = manage_incoming_visitors(addr, control_stream, visitor_stream_tx) => {
            log::debug!("Session: Stopped accepting new connections");
        }
        _ = manage_channel_lifecycle(channels.clone(), channel_complete_rx) => {
            log::debug!("Session: Stopped managing channels lifecycle");
        }
        _ = manage_channel_creation(visitor_stream_rx, client_stream_rx, channels.clone(), channel_complete_tx, channel_idle_timeout) => {
            log::debug!("Session: Stopped managing channels creation");
        }
    }

    for (_, channel) in channels.write().await.drain() {
        channel.shutdown().await;
    }
}

/// Accepts incoming visitor connections and forwards connection requests
/// to the control stream.
///
/// # Arguments
///
/// * `addr` - The socket address where the listener is bound.
/// * `control_stream` - The TCP stream for sending control messages.
/// * `visitor_stream_tx` - A sender for forwarding accepted visitor streams.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `Error` if binding or accepting connections fails.
async fn manage_incoming_visitors(
    addr: SocketAddr,
    mut control_stream: TcpStream,
    visitor_stream_tx: mpsc::Sender<TcpStream>,
) -> Result<(), Error> {
    let listener = TcpListener::bind(addr).await.handle_err(location!())?;
    loop {
        let (visitor, addr) = listener.accept().await.handle_err(location!())?;
        log::debug!("Session: Accepted a visitor from {}", addr);
        protocol::write_message(&mut control_stream, Message::ForwardConnectionRequest).await?;
        visitor_stream_tx
            .send(visitor)
            .await
            .handle_err(location!())?;
    }
}

/// Manages the lifecycle of active channels by removing completed ones.
///
/// # Arguments
///
/// * `channels` - A shared, thread-safe map storing active channels.
/// * `channel_complete_rx` - A receiver for listening to completed channel notifications.
async fn manage_channel_lifecycle(
    channels: Arc<RwLock<HashMap<ChannelId, Channel>>>,
    mut channel_complete_rx: mpsc::Receiver<ChannelId>,
) {
    loop {
        if let Some(channel_id) = channel_complete_rx.recv().await {
            channels.write().await.remove(&channel_id);
        } else {
            return;
        }
    }
}

/// Matches visitor and client connections to create new channels.
///
/// # Arguments
///
/// * `visitor_stream_rx` - A receiver for incoming visitor connections.
/// * `client_stream_rx` - A receiver for incoming client connections.
/// * `channels` - A shared, thread-safe map storing active channels.
/// * `channel_complete_tx` - A sender for notifying completed channels.
/// * `channel_idle_timeout`: The timeout duration for idle channels before shutdown.
async fn manage_channel_creation(
    mut visitor_stream_rx: mpsc::Receiver<TcpStream>,
    mut client_stream_rx: mpsc::Receiver<TcpStream>,
    channels: Arc<RwLock<HashMap<ChannelId, Channel>>>,
    channel_complete_tx: mpsc::Sender<ChannelId>,
    channel_idle_timeout: Duration,
) {
    loop {
        let (visitor, client) = tokio::join!(visitor_stream_rx.recv(), client_stream_rx.recv(),);

        if visitor.is_none() || client.is_none() {
            return;
        }

        let channel = Channel::new(
            visitor.unwrap(),
            client.unwrap(),
            channel_complete_tx.clone(),
            channel_idle_timeout,
        );

        if channels
            .write()
            .await
            .insert(channel.get_id(), channel)
            .is_some()
        {
            panic!("Channel id collision detected, refine the ID generation mechanism");
        }
    }
}
