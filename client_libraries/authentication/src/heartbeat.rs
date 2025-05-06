use std::time::Duration;

use crate::AuthHandler;
use nullnet_libappguard::DeviceStatus;

pub async fn routine(auth_handler: AuthHandler) {
    loop {
        let mut client = auth_handler.client.clone();
        let Ok(mut heartbeat_stream) = client
            .heartbeat(auth_handler.app_id.clone(), auth_handler.app_secret.clone())
            .await
        else {
            log::warn!("Failed to send heartbeat to the server. Retrying in 10 seconds...");
            tokio::time::sleep(Duration::from_secs(10)).await;
            continue;
        };

        while let Some(Ok(heartbeat_response)) = heartbeat_stream.next().await {
            handle_hb_response(&heartbeat_response);
            let mut t = auth_handler.token.write().await;
            *t = heartbeat_response.token();
            drop(t);
        }
    }
}

fn handle_hb_response(response: &GenericHeartbeatResponse) {
    match DeviceStatus::try_from(response.status()) {
        Ok(DeviceStatus::Archived | DeviceStatus::Deleted) => {
            log::warn!("Device has been archived or deleted, aborting execution ...",);
            std::process::exit(0);
        }
        Ok(_) => {}
        Err(_) => log::error!("Unknown device status value {}", response.status()),
    }
}

pub(crate) enum GenericHeartbeatResponse {
    AppGuard(nullnet_libappguard::HeartbeatResponse),
    WallGuard(nullnet_libwallguard::HeartbeatResponse),
}

impl GenericHeartbeatResponse {
    fn status(&self) -> i32 {
        match self {
            GenericHeartbeatResponse::AppGuard(response) => response.status,
            GenericHeartbeatResponse::WallGuard(response) => response.status,
        }
    }

    fn token(&self) -> String {
        match self {
            GenericHeartbeatResponse::AppGuard(response) => response.token.clone(),
            GenericHeartbeatResponse::WallGuard(response) => response.token.clone(),
        }
    }
}

impl Into<GenericHeartbeatResponse> for nullnet_libappguard::HeartbeatResponse {
    fn into(self) -> GenericHeartbeatResponse {
        GenericHeartbeatResponse::AppGuard(self)
    }
}

impl Into<GenericHeartbeatResponse> for nullnet_libwallguard::HeartbeatResponse {
    fn into(self) -> GenericHeartbeatResponse {
        GenericHeartbeatResponse::WallGuard(self)
    }
}
