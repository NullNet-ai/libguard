use crate::datastore::generic_log::GenericLog;
use nullnet_libappguard::AppGuardGrpcInterface;
use nullnet_libwallguard::WallGuardGrpcInterface;

#[derive(Clone)]
pub(crate) enum GrpcInterface {
    AppGuard(AppGuardGrpcInterface),
    WallGuard(WallGuardGrpcInterface),
}

impl GrpcInterface {
    pub(crate) async fn handle_logs(
        &mut self,
        token: String,
        logs: Vec<GenericLog>,
    ) -> Result<(), String> {
        match self {
            GrpcInterface::AppGuard(client) => {
                let logs = nullnet_libappguard::appguard::Logs {
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
