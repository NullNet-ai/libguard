use std::time::Duration;

use crate::datastore::auth::AuthHandler;

pub async fn routine(auth_handler: AuthHandler) {
    loop {
        let mut client = auth_handler.client.clone();
        let Ok(mut heartbeat_stream) = client
            .heartbeat(auth_handler.id.clone(), auth_handler.secret.clone())
            .await
        else {
            log::warn!("Failed to send heartbeat to the server. Retrying in 10 seconds...");
            tokio::time::sleep(Duration::from_secs(10)).await;
            continue;
        };

        while let Some(Ok(heartbeat_response)) = heartbeat_stream.next().await {
            let mut t = auth_handler.token.write().await;
            *t = heartbeat_response.token();
            drop(t);
        }
    }
}

pub(crate) enum GenericHeartbeatResponse {
    AppGuard(nullnet_libappguard::HeartbeatResponse),
    WallGuard(nullnet_libwallguard::HeartbeatResponse),
}

impl GenericHeartbeatResponse {
    fn token(&self) -> String {
        match self {
            GenericHeartbeatResponse::AppGuard(response) => response.token.clone(),
            GenericHeartbeatResponse::WallGuard(response) => response.token.clone(),
        }
    }
}

impl From<nullnet_libappguard::HeartbeatResponse> for GenericHeartbeatResponse {
    fn from(val: nullnet_libappguard::HeartbeatResponse) -> Self {
        GenericHeartbeatResponse::AppGuard(val)
    }
}

impl From<nullnet_libwallguard::HeartbeatResponse> for GenericHeartbeatResponse {
    fn from(val: nullnet_libwallguard::HeartbeatResponse) -> Self {
        GenericHeartbeatResponse::WallGuard(val)
    }
}
