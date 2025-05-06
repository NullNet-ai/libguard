use crate::datastore::auth::{AuthHandler, GrpcInterface};
use crate::datastore::config::DatastoreConfig;

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

#[derive(Clone)]
pub(crate) struct GenericLog {
    pub(crate) timestamp: String,
    pub(crate) level: String,
    pub(crate) message: String,
}

impl From<GenericLog> for nullnet_libappguard::Log {
    fn from(val: GenericLog) -> nullnet_libappguard::Log {
        nullnet_libappguard::Log {
            timestamp: val.timestamp,
            level: val.level,
            message: val.message,
        }
    }
}

impl From<GenericLog> for nullnet_libwallguard::Log {
    fn from(val: GenericLog) -> nullnet_libwallguard::Log {
        nullnet_libwallguard::Log {
            timestamp: val.timestamp,
            level: val.level,
            message: val.message,
        }
    }
}
