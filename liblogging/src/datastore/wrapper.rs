use crate::datastore::config::DatastoreConfig;
use crate::datastore::generic_log::GenericLog;
use crate::datastore::grpc_interface::GrpcInterface;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) struct ServerWrapper {
    inner: GrpcInterface,
    token: Arc<RwLock<String>>,
}

impl ServerWrapper {
    pub(crate) async fn new(datastore_config: DatastoreConfig) -> Self {
        let inner = datastore_config.connect().await;

        Self {
            inner,
            token: datastore_config.token,
        }
    }

    pub(crate) async fn logs_insert(&mut self, logs: Vec<GenericLog>) -> Result<(), String> {
        let token = self.token.read().await.clone();

        self.inner.handle_logs(token, logs).await
    }
}
