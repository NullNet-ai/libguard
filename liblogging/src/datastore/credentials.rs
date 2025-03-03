#[derive(Debug)]
/// Datastore credentials
pub struct DatastoreConfig {
    pub(crate) app_id: String,
    pub(crate) app_secret: String,
    pub(crate) server_addr: String,
    pub(crate) server_port: u16,
}

impl DatastoreConfig {
    /// Create new `DatastoreConfig`
    ///
    /// # Arguments
    ///
    /// * `id` - Application or Account ID
    /// * `secret` - Application or Account Secret
    /// * `server_addr` - Server address
    /// * `server_port` - Server port
    pub fn new<S: Into<String>>(id: S, secret: S, server_addr: S, server_port: u16) -> Self {
        Self {
            app_id: id.into(),
            app_secret: secret.into(),
            server_addr: server_addr.into(),
            server_port,
        }
    }
}
