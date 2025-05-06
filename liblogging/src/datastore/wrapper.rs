use crate::datastore::auth::{AuthHandler, GrpcInterface};
use crate::datastore::config::DatastoreConfig;
use crate::datastore::generic_log::GenericLog;

pub(crate) struct ServerWrapper {
    inner: GrpcInterface,
    auth: AuthHandler,
}

impl ServerWrapper {
    pub(crate) async fn new(datastore_config: DatastoreConfig) -> Self {
        let inner = datastore_config.grpc.clone();
        let auth = AuthHandler::new(datastore_config).await;

        Self { inner, auth }
    }

    pub(crate) async fn logs_insert(&mut self, logs: Vec<GenericLog>) -> Result<(), String> {
        let token = self.auth.get_token().await;

        self.inner.handle_logs(token, logs).await
    }
}
