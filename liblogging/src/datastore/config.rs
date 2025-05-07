use crate::datastore::grpc_interface::GrpcInterface;
use nullnet_libappguard::AppGuardGrpcInterface;
use nullnet_libwallguard::WallGuardGrpcInterface;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

pub struct DatastoreConfig {
    pub(crate) token: Arc<RwLock<String>>,
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
    /// * `token` - The token to use for authenticating to datastore.
    /// * `server_kind` - The kind of server to connect to (i.e., `AppGuard` or `WallGuard`).
    /// * `addr` - The IP address of the server (use 0.0.0.0 if running from the server itself).
    /// * `port` - The port of the server.
    /// * `tls` - Whether to use TLS or not for communication with the server.
    #[allow(clippy::missing_errors_doc)]
    #[must_use]
    pub fn new(
        token: Arc<RwLock<String>>,
        server_kind: ServerKind,
        addr: String,
        port: u16,
        tls: bool,
    ) -> Self {
        Self {
            token,
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
