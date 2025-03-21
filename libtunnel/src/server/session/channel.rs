use std::time::Duration;

use crate::common::copy_bidirectional_with_timeout;
use tokio::{
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
    /// Creates a new `Channel` and spawns a background task to manage data transfer.
    /// This task handles bidirectional data transfer between two TCP streams with a timeout.
    ///
    /// # Arguments
    /// - `s1`: The first `TcpStream` for data transfer.
    /// - `s2`: The second `TcpStream` for data transfer.
    /// - `complete_tx`: A sender to notify when the data transfer completes.
    /// - `idle_timeout`: The timeout duration for idle connections before shutdown.
    ///
    /// # Returns
    /// A `Channel` struct containing the unique identifier, task handle, and shutdown sender.
    ///
    /// # Notes
    /// - If a shutdown signal is received, the task stops without notifying completion.
    /// - If data transfer completes, the task notifies via `complete_tx`.
    pub fn new(
        s1: TcpStream,
        s2: TcpStream,
        complete_tx: mpsc::Sender<ChannelId>,
        idle_timeout: Duration,
    ) -> Self {
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let id = Uuid::new_v4();

        let handle = tokio::spawn(launch_data_channel(
            id,
            s1,
            s2,
            idle_timeout,
            shutdown_rx,
            complete_tx,
        ));

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

/// Handles bidirectional data transfer between two TCP streams with a shutdown signal.
/// This function runs inside a spawned task, where it waits for either a shutdown signal
/// or completion of the data transfer operation. Upon completion, the function either shuts
/// down gracefully or notifies that the data transfer is complete.
///
/// # Arguments
///
/// * `id` - A unique identifier for the channel to identify the data transfer task.
/// * `s1` - The first `TcpStream` participating in the bidirectional transfer.
/// * `s2` - The second `TcpStream` participating in the bidirectional transfer.
/// * `idle_timeout` - A `Duration` that specifies the timeout period for idle connections before shutting down.
/// * `shutdown_rx` - A `oneshot::Receiver<()>` to listen for a shutdown signal that will cancel the data transfer.
/// * `complete_tx` - An `mpsc::Sender<ChannelId>` used to notify the caller that the data transfer has completed.
async fn launch_data_channel(
    id: ChannelId,
    s1: TcpStream,
    s2: TcpStream,
    idle_timeout: Duration,
    shutdown_rx: oneshot::Receiver<()>,
    complete_tx: mpsc::Sender<ChannelId>,
) {
    tokio::select! {
        _ = shutdown_rx => {
            log::debug!("Channel {}: Shutdown signal received, aborting...", id);
        },
        _ = copy_bidirectional_with_timeout(s1, s2, idle_timeout) => {
            log::debug!("Channel {}: Data transfer completed, closing...", id);

            if let Err(err) = complete_tx.send(id).await {
                log::error!("Channel {}: Failed to send complete notification: {}", id, err);
            }
        }
    }
}
