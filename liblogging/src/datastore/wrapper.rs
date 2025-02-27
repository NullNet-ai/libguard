use crate::datastore::credentials::DatastoreCredentials;
use crate::datastore::entry::DatastoreEntry;
use crate::datastore::token::TokenWrapper;
use nullnet_libdatastore::{
    BatchCreateBody, BatchCreateRequest, CreateParams, CreateRequest, DatastoreClient,
    DatastoreConfig, LoginBody, LoginData, LoginRequest, Query, ResponseData,
};
use nullnet_liberror::{location, Error, ErrorHandler, Location};

#[derive(Debug)]
pub(crate) struct DatastoreWrapper {
    inner: DatastoreClient,
    datastore_credentials: DatastoreCredentials,
    token: Option<TokenWrapper>,
}

impl DatastoreWrapper {
    pub(crate) fn new(datastore_credentials: DatastoreCredentials) -> Self {
        let config = DatastoreConfig::from_env();
        let inner = DatastoreClient::new(config);

        Self {
            inner,
            datastore_credentials,
            token: None,
        }
    }

    #[allow(clippy::missing_errors_doc)]
    async fn login(&self) -> Result<String, Error> {
        let request = LoginRequest {
            body: Some(LoginBody {
                data: Some(LoginData {
                    account_id: self.datastore_credentials.app_id.clone(),
                    account_secret: self.datastore_credentials.app_secret.clone(),
                }),
            }),
        };

        let response = self.inner.login(request).await?;

        Ok(response.token)
    }

    async fn get_and_set_token_safe(&mut self) -> Result<String, Error> {
        let is_expired = self.token.as_ref().is_none_or(TokenWrapper::is_expired);

        if is_expired {
            let new_token_string = self.login().await?;
            let new_token = TokenWrapper::from_jwt(new_token_string).handle_err(location!())?;
            self.token = Some(new_token);
        }

        Ok(self.token.as_ref().unwrap().jwt.clone())
    }

    pub(crate) async fn logs_insert_single(
        &mut self,
        log: DatastoreEntry,
    ) -> Result<ResponseData, Error> {
        let body = serde_json::to_string(&log).handle_err(location!())?;

        let request = CreateRequest {
            params: Some(CreateParams {
                table: String::from("wallguard_logs"),
            }),
            query: Some(Query {
                pluck: String::from("id"),
                durability: String::from("soft"),
            }),
            body,
        };

        let token = self.get_and_set_token_safe().await?;

        self.inner.create(request, &token).await
    }

    pub(crate) async fn logs_insert_batch(
        &mut self,
        logs: Vec<DatastoreEntry>,
    ) -> Result<ResponseData, Error> {
        let records = serde_json::to_string(&logs).handle_err(location!())?;

        let request = BatchCreateRequest {
            params: Some(CreateParams {
                table: String::from("wallguard_logs"),
            }),
            query: Some(Query {
                pluck: String::new(),
                durability: String::from("soft"),
            }),
            body: Some(BatchCreateBody {
                records,
                entity_prefix: String::from("LO"),
            }),
        };

        let token = self.get_and_set_token_safe().await?;

        self.inner.batch_create(request, &token).await
    }
}

// TODO test logs serialization
