use crate::datastore::auth::GrpcInterface;
use nullnet_libappguard::AppGuardGrpcInterface;
use nullnet_libwallguard::WallGuardGrpcInterface;

pub struct DatastoreConfig {
    pub(crate) id: String,
    pub(crate) secret: String,
    pub(crate) grpc: GrpcInterface,
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
    pub async fn new(
        id: String,
        secret: String,
        server_kind: ServerKind,
        addr: String,
        port: u16,
        tls: bool,
    ) -> Result<Self, String> {
        let grpc = match server_kind {
            ServerKind::AppGuard => {
                GrpcInterface::AppGuard(AppGuardGrpcInterface::new(&addr, port, tls).await?)
            }
            ServerKind::WallGuard => {
                GrpcInterface::WallGuard(WallGuardGrpcInterface::new(&addr, port).await)
            }
        };

        Ok(Self { id, secret, grpc })
    }
}

pub enum ServerKind {
    AppGuard,
    WallGuard,
}
