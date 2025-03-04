use crate::datastore::config::DatastoreConfig;
use crate::datastore::token::TokenWrapper;
use nullnet_libwallguard::{Authentication, CommonResponse, Log, Logs, WallGuardGrpcInterface};

pub(crate) struct ServerWrapper {
    inner: WallGuardGrpcInterface,
    datastore_config: DatastoreConfig,
    token: Option<TokenWrapper>,
}

impl ServerWrapper {
    pub(crate) async fn new(datastore_config: DatastoreConfig) -> Self {
        let inner = WallGuardGrpcInterface::new(
            &datastore_config.server_addr,
            datastore_config.server_port,
        )
        .await;

        Self {
            inner,
            datastore_config,
            token: None,
        }
    }

    #[allow(clippy::missing_errors_doc)]
    async fn login(&mut self) -> Result<String, String> {
        self.inner
            .login(
                self.datastore_config.app_id.clone(),
                self.datastore_config.app_secret.clone(),
            )
            .await
    }

    async fn get_and_set_token_safe(&mut self) -> Result<String, String> {
        let is_expired = self.token.as_ref().is_none_or(TokenWrapper::is_expired);

        if is_expired {
            let new_token_string = self.login().await?;
            let new_token = TokenWrapper::from_jwt(new_token_string)?;
            self.token = Some(new_token);
        }

        Ok(self.token.as_ref().unwrap().jwt.clone())
    }

    pub(crate) async fn logs_insert(&mut self, logs: Vec<Log>) -> Result<CommonResponse, String> {
        // println!("Attempt to send 1 log entry to the datastore");

        let token = self.get_and_set_token_safe().await?;

        self.inner
            .handle_logs(Logs {
                auth: Some(Authentication { token }),
                logs,
            })
            .await
    }
}
