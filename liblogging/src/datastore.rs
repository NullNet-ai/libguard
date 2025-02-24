use nullnet_libdatastore::{ErrorKind as DSErrorKind, Error as DSError, Response as DSResponse, BatchCreateBody, BatchCreateRequest, CreateParams, DatastoreClient, DatastoreConfig, Query, CreateRequest};
use crate::postgres_logger::PostgresEntry;

#[derive(Debug, Clone)]
pub struct DatastoreWrapper {
    inner: DatastoreClient,
}

impl DatastoreWrapper {
    pub fn new() -> Self {
        let config = DatastoreConfig::from_env();
        let inner = DatastoreClient::new(config);
        Self { inner }
    }

    pub fn set_token_for_request<T>(request: &mut Request<T>, token: &str) -> Result<(), DSError> {
        let value = MetadataValue::from_str(token).map_err(|e| DSError {
            kind: DSErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        request.metadata_mut().insert("authorization", value);

        Ok(())
    }

    async fn logs_insert_single(
        &self,
        token: &str,
        log: PostgresEntry,
    ) -> Result<DSResponse, DSError> {
        let mut request = CreateRequest {
            params: Some(CreateParams {
                table: String::from("logs"),
            }),
            query: Some(Query {
                pluck: String::from("id"),
                durability: String::from("hard"),
            }),
            body: serde_json::to_string(&log).map_err(|e| DSError {
                kind: DSErrorKind::ErrorRequestFailed,
                message: e.to_string(),
            })?,
        };

        Self::set_token_for_request(&mut request, token)?;

        self.inner.create(request).await
    }

    pub async fn logs_insert_batch(
        &self,
        token: &str,
        logs: Vec<PostgresEntry>,
    ) -> Result<DSResponse, DSError> {
        let records = serde_json::to_string(&logs).map_err(|e| DSError {
            kind: DSErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        let mut request = BatchCreateRequest {
            params: Some(CreateParams {
                table: String::from("logs"),
            }),
            query: Some(Query {
                pluck: String::new(),
                durability: String::from("soft"),
            }),
            body: Some(BatchCreateBody {
                records,
                entity_prefix: String::from("PK"),
            }),
        };

        Self::set_token_for_request(&mut request, token)?;

        self.inner.batch_create(request).await
    }
}

// TODO test logs serialization