use crate::datastore::auth::GrpcInterface;

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
    /// * `grpc` - The gRPC interface of the server to use for communication.
    pub fn new(id: String, secret: String, grpc: GrpcInterface) -> Self {
        Self { id, secret, grpc }
    }
}
