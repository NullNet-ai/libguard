mod heartbeat;

pub use crate::datastore::config::DatastoreConfig;
pub(crate) use crate::datastore::grpc_interface::GrpcInterface;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AuthHandler {
    id: String,
    secret: String,
    token: Arc<RwLock<String>>,
    client: GrpcInterface,
}

impl AuthHandler {
    #[must_use]
    pub async fn new(datastore_config: DatastoreConfig) -> Self {
        let client = datastore_config.connect().await;

        let auth = Self {
            id: datastore_config.id,
            secret: datastore_config.secret,
            client,
            token: Arc::new(RwLock::new(String::new())),
        };

        let auth_2 = auth.clone();
        tokio::spawn(async move { heartbeat::routine(auth_2).await });

        while auth.token.read().await.is_empty() {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        auth
    }

    pub async fn get_token(&self) -> String {
        self.token.read().await.clone()
    }
}
