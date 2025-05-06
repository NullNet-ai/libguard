use crate::heartbeat::GenericHeartbeatResponse;
use futures_util::StreamExt;
use nullnet_libappguard::AppGuardGrpcInterface;
use nullnet_libwallguard::WallGuardGrpcInterface;

#[derive(Clone)]
pub enum GrpcInterface {
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
                .map(|s| GenericHeartbeatResponseStreaming::AppGuard(s)),
            GrpcInterface::WallGuard(client) => client
                .heartbeat(app_id, app_secret, String::new(), String::new())
                .await
                .map(|s| GenericHeartbeatResponseStreaming::WallGuard(s)),
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
