use crate::datastore::auth::GrpcInterface;
use nullnet_libappguard::AppGuardGrpcInterface;
use nullnet_libwallguard::WallGuardGrpcInterface;
use std::time::Duration;
use tokio::time::sleep;

pub struct DatastoreConfig {
    pub(crate) id: String,
    pub(crate) secret: String,
    pub(crate) server_kind: ServerKind,
    pub(crate) addr: String,
    pub(crate) port: u16,
    pub(crate) tls: bool,
}

impl DatastoreConfig {
    /// Creates a new `DatastoreConfig` instance.
    ///
    /// # Arguments
    ///
    /// * `id` - The app or account ID to use for login.
    /// * `secret` - The app or account secret to use for login.
    /// * `server_kind` - The kind of server to connect to (i.e., `AppGuard` or `WallGuard`).
    /// * `addr` - The IP address of the server (use 0.0.0.0 if running from the server itself).
    /// * `port` - The port of the server.
    /// * `tls` - Whether to use TLS or not for communication with the server.
    #[allow(clippy::missing_errors_doc)]
    #[must_use]
    pub fn new(
        id: String,
        secret: String,
        server_kind: ServerKind,
        addr: String,
        port: u16,
        tls: bool,
    ) -> Self {
        Self {
            id,
            secret,
            server_kind,
            addr,
            port,
            tls,
        }
    }

    pub(crate) async fn connect(&self) -> GrpcInterface {
        // Create a new gRPC client based on the server kind
        loop {
            match self.server_kind {
                ServerKind::AppGuard => {
                    match AppGuardGrpcInterface::new(&self.addr, self.port, self.tls).await {
                        Ok(client) => return GrpcInterface::AppGuard(client),
                        Err(_) => {
                            sleep(Duration::from_secs(1)).await;
                        }
                    }
                }
                ServerKind::WallGuard => {
                    return GrpcInterface::WallGuard(
                        WallGuardGrpcInterface::new(&self.addr, self.port).await,
                    );
                }
            }
        }
    }
}

pub enum ServerKind {
    AppGuard,
    WallGuard,
}
