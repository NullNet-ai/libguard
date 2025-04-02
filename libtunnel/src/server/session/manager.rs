use super::r#impl::Session;
use crate::{Hash, server::profile::Profile, str_hash};
use nullnet_liberror::{Error, ErrorHandler, Location, location};
use std::{
    collections::{HashMap, hash_map},
    sync::Arc,
    time::Duration,
};
use tokio::{
    net::TcpStream,
    sync::{RwLock, mpsc},
};

/// `Manager` is responsible for handling active sessions, ensuring session lifecycle management,
/// and providing mechanisms to spawn, terminate, and interact with sessions.
#[derive(Debug, Clone)]
pub struct Manager {
    /// A thread-safe collection of active sessions, identified by a unique hash.
    sessions: Arc<RwLock<HashMap<Hash, Session>>>,
    /// A channel sender used to notify when a session has completed.
    notify_session_complete: mpsc::Sender<Hash>,
}

impl Manager {
    /// Creates a new instance of `Manager` with an empty session map.
    ///
    /// # Returns
    /// A new `Manager` instance.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(128);
        let sessions = Arc::new(RwLock::new(HashMap::new()));

        tokio::spawn(manage_session_lifetime(sessions.clone(), rx));

        Self {
            sessions,
            notify_session_complete: tx,
        }
    }

    /// Spawns a new session for a given profile and TCP stream.
    ///
    /// # Parameters
    /// - `stream`: The TCP stream representing the connection.
    /// - `profile`: The profile associated with the session.
    /// - `channel_idle_timeout`: The timeout duration for idle channels before shutdown.
    /// - `visitors_token` - Optinal token that will be used to authenticate incoming visitors.
    ///
    /// # Returns
    /// - `Ok(())`: If the session is successfully created.
    /// - `Err(Error)`: If a session with the same unique ID already exists.
    ///
    /// # Errors
    /// Returns an error if a session with the same unique identifier already exists.
    pub async fn spawn_session<T>(
        &self,
        stream: TcpStream,
        profile: &T,
        channel_idle_timeout: Duration,
    ) -> Result<(), Error>
    where
        T: Profile,
    {
        let id_hash = str_hash(&profile.get_unique_id());

        if let hash_map::Entry::Vacant(entry) = self.sessions.write().await.entry(id_hash) {
            let addr = profile.get_visitor_addr();
            let token = profile.get_visitor_token();
            let session = Session::new(
                addr,
                stream,
                token,
                channel_idle_timeout,
                self.notify_session_complete.clone(),
                id_hash,
            );
            entry.insert(session);
        } else {
            return Err(format!(
                "Session creation failed: A session with the same ID already exists. Hash [{:?}]",
                &id_hash,
            ))
            .handle_err(location!());
        }

        Ok(())
    }

    /// Terminates an existing session associated with a given profile.
    ///
    /// # Parameters
    /// - `profile`: The profile associated with the session to be terminated.
    ///
    /// # Returns
    /// - `Ok(())`: If the session is successfully terminated.
    /// - `Err(Error)`: If no session exists for the given profile.
    ///
    /// # Errors
    /// Returns an error if no session exists for the provided profile.
    pub async fn terminate_session<T>(&self, profile: &T) -> Result<(), Error>
    where
        T: Profile,
    {
        let id_hash = str_hash(&profile.get_unique_id());

        match self.sessions.write().await.remove(&id_hash) {
            Some(session) => {
                session.shutdown().await;
                Ok(())
            }
            None => Err(format!(
                "Session termination failed: No active session found for the given ID. Hash [{:?}]",
                id_hash
            ))
            .handle_err(location!()),
        }
    }

    /// Requests a new channel from an existing session.
    ///
    /// # Parameters
    /// - `hash`: The unique identifier of the session.
    /// - `stream`: The TCP stream representing the new connection for the requested channel.
    ///
    /// # Returns
    /// - `Ok(())`: If the channel request is successful.
    /// - `Err(Error)`: If the session does not exist or the request fails.
    ///
    /// # Errors
    /// Returns an error if no session exists for the provided hash.
    pub async fn request_channel(&self, hash: &Hash, stream: TcpStream) -> Result<(), Error> {
        let lock = self.sessions.read().await;

        if !lock.contains_key(hash) {
            return Err(format!(
                "Channel request failed: No active session found for the given hash. Hash [{:?}]",
                hash,
            ))
            .handle_err(location!());
        }

        lock.get(hash).unwrap().request_channel(stream).await
    }

    /// Checks if a session exists for a given hash.
    ///
    /// # Parameters
    /// - `hash`: The unique identifier of the session.
    ///
    /// # Returns
    /// - `true`: If a session exists for the given hash.
    /// - `false`: If no session exists.
    pub async fn session_exists(&self, hash: &Hash) -> bool {
        self.sessions.read().await.contains_key(hash)
    }

    /// Terminates all active sessions managed by the `Manager`.
    ///
    /// This function will gracefully shut down all sessions by calling the `shutdown`
    /// method on each session. After the termination process, the session map will be emptied.
    pub async fn terminate_all(&self) {
        for (_, session) in self.sessions.write().await.drain() {
            session.shutdown().await;
        }
    }

    /// Checks if a session is active for a given hash.
    ///
    /// # Parameters
    /// - `hash`: The unique identifier of the session to check.
    ///
    /// # Returns
    /// - `Some(true)`: If the session exists and has active channels.
    /// - `Some(false)`: If the session exists but does not have active channels.
    /// - `None`: If no session exists for the given hash.
    pub async fn is_session_active(&self, hash: &Hash) -> Option<bool> {
        match self.sessions.read().await.get(hash) {
            Some(session) => Some(session.has_active_channels().await),
            None => None,
        }
    }

    /// Determines whether a session exists for a given hash.
    ///
    /// # Parameters
    /// - `hash`: The unique identifier of the session.
    ///
    /// # Returns
    /// - `true`: If a session exists for the given hash.
    /// - `false`: If no session exists.
    ///
    /// This function is a shorthand for `session_exists`, providing a quick way to check session presence.
    pub async fn has_session(&self, hash: &Hash) -> bool {
        self.sessions.read().await.contains_key(hash)
    }
}

/// Continuously monitors session completion notifications and removes completed sessions.
///
/// # Parameters
/// - `sessions`: A thread-safe collection of active sessions, stored in an `Arc<RwLock<HashMap<Hash, Session>>>`.  
///   This allows concurrent read access while ensuring safe write operations.
/// - `receiver`: An `mpsc::Receiver<Hash>` that receives session hashes of completed sessions.
async fn manage_session_lifetime(
    sessions: Arc<RwLock<HashMap<Hash, Session>>>,
    mut receiver: mpsc::Receiver<Hash>,
) {
    while let Some(session_hash) = receiver.recv().await {
        sessions.write().await.remove(&session_hash);
    }
}
