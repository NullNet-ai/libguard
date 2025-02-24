use crate::postgres_logger::PostgresEntry;
use nullnet_libdatastore::{BatchCreateBody, BatchCreateRequest, CreateParams, CreateRequest, DatastoreClient, DatastoreConfig, Error as DSError, ErrorKind as DSErrorKind, LoginBody, LoginData, LoginRequest, Query, Response as DSResponse};

#[derive(Debug, Clone)]
pub(crate) struct DatastoreWrapper {
    inner: DatastoreClient,
}

impl DatastoreWrapper {
    pub(crate) fn new() -> Self {
        let config = DatastoreConfig::from_env();
        let inner = DatastoreClient::new(config);
        Self { inner }
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn login(
        &self,
        account_id: String,
        account_secret: String,
    ) -> Result<String, DSError> {
        let request = LoginRequest {
            body: Some(LoginBody {
                data: Some(LoginData {
                    account_id,
                    account_secret,
                }),
            }),
        };

        let response = self.inner.login(request).await?;

        Ok(response.token)
    }

    pub(crate) async fn logs_insert_single(
        &self,
        token: &str,
        log: PostgresEntry,
    ) -> Result<DSResponse, DSError> {
        let body = serde_json::to_string(&log).map_err(|e| DSError {
            kind: DSErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        let request = CreateRequest {
            params: Some(CreateParams {
                table: String::from("logs"),
            }),
            query: Some(Query {
                pluck: String::from("id"),
                durability: String::from("soft"),
            }),
            body,
        };

        self.inner.create(request, token).await
    }

    pub(crate) async fn logs_insert_batch(
        &self,
        token: &str,
        logs: Vec<PostgresEntry>,
    ) -> Result<DSResponse, DSError> {
        let records = serde_json::to_string(&logs).map_err(|e| DSError {
            kind: DSErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        let request = BatchCreateRequest {
            params: Some(CreateParams {
                table: String::from("logs"),
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

        self.inner.batch_create(request, token).await
    }
}

// TODO test logs serialization
