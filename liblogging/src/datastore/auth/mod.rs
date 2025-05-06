mod grpc_interface;
mod heartbeat;

pub use crate::datastore::auth::grpc_interface::GrpcInterface;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AuthHandler {
    app_id: String,
    app_secret: String,
    token: Arc<RwLock<String>>,
    client: GrpcInterface,
}

impl AuthHandler {
    #[must_use]
    pub async fn new(client: GrpcInterface) -> Self {
        let app_id = std::env::var("APP_ID").unwrap_or_default();
        let app_secret = std::env::var("APP_SECRET").unwrap_or_default();
        let auth = Self {
            app_id,
            app_secret,
            client: client.clone(),
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
