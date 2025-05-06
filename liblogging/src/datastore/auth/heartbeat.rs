use std::time::Duration;

use crate::datastore::auth::AuthHandler;

pub async fn routine(auth_handler: AuthHandler) {
    loop {
        let mut client = auth_handler.client.clone();
        let Ok(mut heartbeat_stream) = client
            .heartbeat(auth_handler.id.clone(), auth_handler.secret.clone())
            .await
        else {
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
