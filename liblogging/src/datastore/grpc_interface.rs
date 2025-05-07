use crate::datastore::generic_log::GenericLog;
use futures_util::StreamExt;
use nullnet_libappguard::AppGuardGrpcInterface;
use nullnet_libwallguard::WallGuardGrpcInterface;

#[derive(Clone)]
pub(crate) enum GrpcInterface {
    AppGuard(AppGuardGrpcInterface),
    WallGuard(WallGuardGrpcInterface),
}

impl GrpcInterface {
    pub(crate) async fn heartbeat(
        &mut self,
        app_id: String,
        app_secret: String,
    ) -> Result<GenericHeartbeatResponseStreaming, String> {
        match self {
            GrpcInterface::AppGuard(client) => client
                .heartbeat(app_id, app_secret)
                .await
                .map(GenericHeartbeatResponseStreaming::AppGuard),
            GrpcInterface::WallGuard(client) => client
                .heartbeat(app_id, app_secret, String::new(), String::new())
                .await
                .map(GenericHeartbeatResponseStreaming::WallGuard),
        }
    }

    pub(crate) async fn handle_logs(
        &mut self,
        token: String,
        logs: Vec<GenericLog>,
    ) -> Result<(), String> {
        match self {
            GrpcInterface::AppGuard(client) => {
                let logs = nullnet_libappguard::Logs {
                    token,
                    logs: logs.into_iter().map(Into::into).collect(),
                };
                client.handle_logs(logs).await
            }
            GrpcInterface::WallGuard(client) => {
                let logs = nullnet_libwallguard::Logs {
                    token,
                    logs: logs.into_iter().map(Into::into).collect(),
                };
                client.handle_logs(logs).await.map(|_| ())
            }
        }
    }
}

pub(crate) enum GenericHeartbeatResponseStreaming {
    AppGuard(nullnet_libappguard::Streaming<nullnet_libappguard::HeartbeatResponse>),
    WallGuard(nullnet_libwallguard::Streaming<nullnet_libwallguard::HeartbeatResponse>),
}

impl GenericHeartbeatResponseStreaming {
    pub(crate) async fn next(&mut self) -> Option<Result<GenericHeartbeatResponse, String>> {
        match self {
            GenericHeartbeatResponseStreaming::AppGuard(stream) => stream
                .next()
                .await
                .map(|response| response.map(Into::into).map_err(|e| e.to_string())),
            GenericHeartbeatResponseStreaming::WallGuard(stream) => stream
                .next()
                .await
                .map(|response| response.map(Into::into).map_err(|e| e.to_string())),
        }
    }
}

pub(crate) enum GenericHeartbeatResponse {
    AppGuard(nullnet_libappguard::HeartbeatResponse),
    WallGuard(nullnet_libwallguard::HeartbeatResponse),
}

impl GenericHeartbeatResponse {
    pub(crate) fn token(&self) -> String {
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
