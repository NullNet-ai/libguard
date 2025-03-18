use tokio::{
    io::copy_bidirectional,
    net::TcpStream,
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use uuid::Uuid;

/// Unique identifier for each channel.
pub type ChannelId = uuid::Uuid;

/// Represents a bidirectional data transfer channel between two `TcpStream`s.
/// The channel manages the lifecycle of a spawned background task handling the data transfer.
#[derive(Debug)]
pub struct Channel {
    id: ChannelId,
    handle: JoinHandle<()>,
    shutdown_tx: oneshot::Sender<()>,
}

impl Channel {
    /// Creates a new `Channel` instance and spawns a background task to manage data transfer.
    ///
    /// # Arguments
    ///
    /// * `s1` - The first `TcpStream` participating in bidirectional data transfer.
    /// * `s2` - The second `TcpStream` participating in bidirectional data transfer.
    /// * `complete_tx` - An `mpsc::Sender` used to notify when the data transfer completes.
    ///
    /// # Returns
    ///
    /// A `Channel` struct containing the unique identifier, task handle, and shutdown sender.
    /// # Notes
    ///
    /// * If a shutdown signal is received first, the task logs the event and stops without sending a completion notification.
    /// * If data transfer completes first, the task logs the event and notifies via `complete_tx`.
    pub fn new(s1: TcpStream, s2: TcpStream, complete_tx: mpsc::Sender<ChannelId>) -> Self {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let id = Uuid::new_v4();

        let handle = tokio::spawn(launch_data_channel(id, s1, s2, shutdown_rx, complete_tx));

        Self {
            id,
            handle,
            shutdown_tx,
        }
    }

    /// Initiates the shutdown process for this channel.
    /// Attempts to send a shutdown signal; if sending fails, the task is forcefully aborted.
    pub async fn shutdown(self) {
        if self.shutdown_tx.send(()).is_ok() {
            let _ = self.handle.await;
        } else {
            log::error!(
                "Channel {}: Failed to send shitdown signal, forcefully aborting the task",
                self.id
            );
            self.handle.abort();
        }
    }

    /// Returns the unique identifier of this channel.
    ///
    /// # Returns
    ///
    /// A `ChannelId` representing the unique identifier of the channel.
    pub fn get_id(&self) -> ChannelId {
        self.id
    }
}

/// Handles bidirectional data transfer between two TCP streams.
/// This function runs inside a spawned task and listens for either a shutdown signal or
/// completion of the data transfer.
///
/// # Arguments
///
/// * `id` - Unique identifier for the channel.
/// * `s1` - The first `TcpStream` participating in the transfer.
/// * `s2` - The second `TcpStream` participating in the transfer.
/// * `shutdown_rx` - A `oneshot::Receiver` to listen for shutdown signals.
/// * `complete_tx` - An `mpsc::Sender` to notify when the transfer is complete.
///
/// # Behavior
///
/// * If a shutdown signal is received first, the task logs the event and stops.
/// * If data transfer completes first, the task logs the event and notifies via `complete_tx`.
async fn launch_data_channel(
    id: ChannelId,
    mut s1: TcpStream,
    mut s2: TcpStream,
    shutdown_rx: oneshot::Receiver<()>,
    complete_tx: mpsc::Sender<ChannelId>,
) {
    tokio::select! {
        _ = shutdown_rx => {
            log::debug!("Channel {}: Shutdown signal received, aborting...", id);
        },
        _ = copy_bidirectional(&mut s1, &mut s2) => {
            log::debug!("Channel {}: Data transfer completed, closing...", id);

            if let Err(err) = complete_tx.send(id).await {
                log::error!("Channel {}: Failed to send complete notification: {}", id, err);
            }
        }
    }
}
