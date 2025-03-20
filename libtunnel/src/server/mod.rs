mod config;
mod profile;
mod session;

use crate::{protocol, str_hash, Hash, Message};
pub use config::Config as ServerConfig;
use nullnet_liberror::{location, Error, ErrorHandler, Location};
pub use profile::Profile;
pub use session::Session;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{oneshot, RwLock},
    task::JoinHandle,
};

/// `Server` manages the lifecycle of active profiles and their associated sessions.
/// It handles requests for profile registration, session management, and server shutdown.
#[derive(Debug)]
pub struct Server<T: Profile + Send + Sync + 'static> {
    /// A manager responsible for handling the active sessions in the server.
    sessions_manager: Arc<session::Manager>,
    /// A thread-safe collection of active profiles, indexed by their unique hash.
    active_profiles: Arc<RwLock<HashMap<Hash, T>>>,
    /// The handle for the server's background task that runs the main server loop.
    handle: JoinHandle<()>,
    /// A sender used to initiate the shutdown of the server.
    shutdown_tx: oneshot::Sender<()>,
}

impl<T: Profile + Send + Sync + 'static> Server<T> {
    /// Creates a new instance of the server with the provided configuration.
    ///
    /// # Parameters
    /// - `config`: The server configuration containing the address to bind to.
    ///
    /// # Returns
    /// A new `Server` instance.
    pub fn new(config: ServerConfig) -> Self {
        let sessions_manager = Arc::new(session::Manager::new());
        let active_profiles = Arc::new(RwLock::new(HashMap::new()));

        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let addr = config.addr;
        let manager = sessions_manager.clone();
        let profiles = active_profiles.clone();

        let handle = tokio::spawn(async move {
            tokio::select! {
                _ = shutdown_rx => {
                    log::debug!("Server: Received shutdown signal");
                },
                _ = main_loop(addr, manager, profiles) => {
                    log::debug!("Server: Main loop completed");
                }
            }
        });

        Self {
            sessions_manager,
            active_profiles,
            shutdown_tx,
            handle,
        }
    }

    /// Inserts a new profile into the active profiles map.
    ///
    /// # Parameters
    /// - `profile`: The profile to be registered.
    ///
    /// # Returns
    /// - `Ok(())`: If the profile is successfully registered.
    /// - `Err(Error)`: If the profile is already registered.
    pub async fn insert_profile(&self, profile: T) -> Result<(), Error> {
        let hash = str_hash(&profile.get_unique_id());

        let mut lock = self.active_profiles.write().await;

        if lock.contains_key(&hash) {
            return Err("Server: Cannot register profile because it is already registered.")
                .handle_err(location!());
        }

        lock.insert(hash, profile);

        Ok(())
    }

    /// Removes a profile from the active profiles map and terminates the associated session.
    ///
    /// # Parameters
    /// - `id`: The unique ID of the profile to be removed.
    ///
    /// # Returns
    /// - `Ok(())`: If the profile is successfully removed.
    pub async fn remove_profile(&self, id: &str) -> Result<(), Error> {
        let hash = str_hash(id);

        if let Some(profile) = self.active_profiles.write().await.remove(&hash) {
            let _ = self.sessions_manager.terminate_session(&profile).await;
        }

        Ok(())
    }

    /// Checks if a profile is currently active.
    ///
    /// # Parameters
    /// - `id`: The unique ID of the profile to check.
    ///
    /// # Returns
    /// - `Some(true)`: If the profile exists and is active.
    /// - `Some(false)`: If the profile exists but is inactive.
    /// - `None`: If no profile exists for the given ID.
    pub async fn is_profile_active(&self, id: &str) -> Option<bool> {
        let hash = str_hash(id);
        self.sessions_manager.is_session_active(&hash).await
    }

    /// Checks if a profile is currently online by verifying whether there are active sessions.
    ///
    /// # Parameters
    /// - `id`: The unique ID of the profile to check.
    ///
    /// # Returns
    /// - `true`: If there is an active session associated with the profile ID.
    /// - `false`: Otherwise.
    pub async fn is_profile_online(&self, id: &str) -> bool {
        let hash = str_hash(id);
        self.sessions_manager.has_session(&hash).await
    }

    /// Shuts down the server and terminates all active sessions.
    ///
    /// This function sends a shutdown signal, terminates all sessions, and waits for the
    /// server background task to complete.
    pub async fn shutdown(self) {
        self.sessions_manager.terminate_all().await;
        if self.shutdown_tx.send(()).is_ok() {
            let _ = self.handle.await;
        } else {
            self.handle.abort();
        }
    }
}

/// The main loop of the server, responsible for accepting client connections and handling requests.
///
/// It listens for incoming connections, processes control and data connection requests, and
/// interacts with the profile and session managers accordingly.
async fn main_loop<T: Profile + Send + Sync + 'static>(
    addr: SocketAddr,
    manager: Arc<session::Manager>,
    profiles: Arc<RwLock<HashMap<Hash, T>>>,
) -> Result<(), Error> {
    let listener = TcpListener::bind(addr).await.handle_err(location!())?;

    loop {
        let (mut stream, addr) = listener.accept().await.handle_err(location!())?;
        log::debug!("Server: Client connected from {}", addr);

        let Ok(message) = protocol::expect_open_message(&mut stream).await else {
            log::error!("Server:  Unexpected opening message, aborting connection ...");
            continue;
        };

        match message {
            Message::OpenSessionRequest(payload) => {
                if let Some(profile) = profiles.read().await.get(payload.data.as_slice()) {
                    match protocol::write_message(&mut stream, Message::Acknowledgment).await {
                        Ok(_) => {
                            if let Err(err) = manager.spawn_session(stream, profile).await {
                                log::error!("Server: failed to request data channel. {:?}", err);
                            }
                        }
                        Err(err) => {
                            log::error!(
                                "Server: Failed to send Acknowledgment message. {}",
                                err.to_str()
                            )
                        }
                    };
                } else if let Err(err) =
                    protocol::write_message(&mut stream, Message::Rejection).await
                {
                    log::error!("Server: Failed to send Rejection message. {}", err.to_str());
                }
            }
            Message::OpenChannelRequest(payload) => {
                if manager.session_exists(&payload.data).await {
                    match protocol::write_message(&mut stream, Message::Acknowledgment).await {
                        Ok(_) => {
                            if let Err(err) = manager.request_channel(&payload.data, stream).await {
                                log::error!("Server: failed to request data channel. {:?}", err);
                            }
                        }
                        Err(err) => {
                            log::error!(
                                "Server: Failed to send Acknowledgment message. {}",
                                err.to_str()
                            );
                        }
                    };
                } else if let Err(err) =
                    protocol::write_message(&mut stream, Message::Rejection).await
                {
                    log::error!("Server: Failed to send Rejection message. {}", err.to_str());
                }
            }
            _ => {
                log::error!("Server: Unexpected opening message, aborting connection ...");
            }
        }
    }
}
