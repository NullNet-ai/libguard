use std::str::FromStr;
use tonic::metadata::MetadataValue;
use tonic::Request;
use crate::datastore::store_service_client::StoreServiceClient;
use crate::{
    AggregateRequest, BatchCreateRequest, CreateRequest, DatastoreConfig, DeleteRequest, Error,
    ErrorKind, GetByFilterRequest, GetByIdRequest, LoginRequest, LoginResponse, Response,
    UpdateRequest,
};
use tonic::transport::{Channel, ClientTlsConfig};

/// A client for interacting with the datastore service.
#[derive(Debug, Clone)]
pub struct DatastoreClient {
    /// Configuration for connecting to the datastore.
    config: DatastoreConfig,
}

impl DatastoreClient {
    /// Creates a new instance of `DatastoreClient` with the specified configuration.
    ///
    /// # Arguments
    /// * `config` - Configuration for the datastore connection.
    #[must_use]
    pub fn new(config: DatastoreConfig) -> Self {
        Self { config }
    }

    /// Establishes a connection to the datastore service.
    ///
    /// # Returns
    /// A `Result` containing a `StoreServiceClient` instance if successful, or an `Error` if the connection fails.
    async fn connect(&self) -> Result<StoreServiceClient<Channel>, Error> {
        let protocol = if self.config.tls { "https" } else { "http" };
        let host = self.config.host.as_str();
        let port = self.config.port;

        let mut endpoint = Channel::from_shared(format!("{protocol}://{host}:{port}"))
            .map_err(|e| Error {
                kind: ErrorKind::ErrorCouldNotConnectToDatastore,
                message: e.to_string(),
            })?
            .connect_timeout(std::time::Duration::from_secs(10));

        if self.config.tls {
            endpoint = endpoint
                .tls_config(ClientTlsConfig::new().with_native_roots())
                .map_err(|e| Error {
                    kind: ErrorKind::ErrorCouldNotConnectToDatastore,
                    message: e.to_string(),
                })?;
        }

        let channel: Channel = endpoint.connect().await.map_err(|e| Error {
            kind: ErrorKind::ErrorCouldNotConnectToDatastore,
            message: e.to_string(),
        })?;

        Ok(StoreServiceClient::new(channel))
    }

    /// Sets the authorization token for a request to the datastore.
    ///
    /// # Arguments
    /// * `request` - The request to set the token on.
    /// * `token` - The authorization token to set.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    fn set_token_for_request<T>(request: &mut Request<T>, token: &str) -> Result<(), Error> {
        let value = MetadataValue::from_str(token).map_err(|e| Error {
            kind: ErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        request.metadata_mut().insert("authorization", value);

        Ok(())
    }

    /// Authenticates with the datastore using the provided login request.
    ///
    /// # Arguments
    /// * `request` - The login request containing authentication details.
    ///
    /// # Returns
    /// A `Result` containing a `LoginResponse` if successful, or an `Error` if the login fails.
    #[allow(clippy::missing_errors_doc)]
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, Error> {
        let request = tonic::Request::new(request);

        let mut client_inner = self.connect().await?;

        let response = client_inner.login(request).await.map_err(|e| Error {
            kind: ErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        Ok(response.into_inner())
    }

    /// Batch creates multiple records in the datastore.
    ///
    /// # Arguments
    /// * `request` - The batch create request containing the records to create.
    ///
    /// # Returns
    /// A `Result` containing a `Response` if successful, or an `Error` if the operation fails.
    #[allow(clippy::missing_errors_doc)]
    pub async fn batch_create(
        &self,
        request: BatchCreateRequest,
        token: &str
    ) -> Result<Response, Error> {
        let mut request = tonic::Request::new(request);
        Self::set_token_for_request(&mut request, token)?;

        let mut client_inner = self.connect().await?;

        let response = client_inner
            .batch_create(request)
            .await
            .map_err(|e| Error {
                kind: ErrorKind::ErrorRequestFailed,
                message: e.to_string(),
            })?;

        Ok(response.into_inner())
    }

    /// Creates a single record in the datastore.
    ///
    /// # Arguments
    /// * `request` - The create request containing the record to create.
    ///
    /// # Returns
    /// A `Result` containing a `Response` if successful, or an `Error` if the operation fails.
    #[allow(clippy::missing_errors_doc)]
    pub async fn create(&self, request: CreateRequest, token: &str) -> Result<Response, Error> {
        let mut request = tonic::Request::new(request);
        Self::set_token_for_request(&mut request, token)?;

        let mut client_inner = self.connect().await?;

        let response = client_inner.create(request).await.map_err(|e| Error {
            kind: ErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        Ok(response.into_inner())
    }

    /// Deletes a record from the datastore.
    ///
    /// # Arguments
    /// * `request` - The delete request containing the record ID to delete.
    ///
    /// # Returns
    /// A `Result` containing a `Response` if successful, or an `Error` if the operation fails.
    #[allow(clippy::missing_errors_doc)]
    pub async fn delete(&self, request: DeleteRequest, token: &str) -> Result<Response, Error> {
        let mut request = tonic::Request::new(request);
        Self::set_token_for_request(&mut request, token)?;

        let mut client_inner = self.connect().await?;

        let response = client_inner.delete(request).await.map_err(|e| Error {
            kind: ErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        Ok(response.into_inner())
    }

    /// Updates a record in the datastore.
    ///
    /// # Arguments
    /// * `request` - The update request containing the updated record details.
    ///
    /// # Returns
    /// A `Result` containing a `Response` if successful, or an `Error` if the operation fails.
    #[allow(clippy::missing_errors_doc)]
    pub async fn update(&self, request: UpdateRequest, token: &str) -> Result<Response, Error> {
        let mut request = tonic::Request::new(request);
        Self::set_token_for_request(&mut request, token)?;

        let mut client_inner = self.connect().await?;

        let response = client_inner.update(request).await.map_err(|e| Error {
            kind: ErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        Ok(response.into_inner())
    }

    /// Retrieves records from the datastore using a filter.
    ///
    /// # Arguments
    /// * `request` - The filter request specifying the criteria for retrieval.
    ///
    /// # Returns
    /// A `Result` containing a `Response` if successful, or an `Error` if the operation fails.
    #[allow(clippy::missing_errors_doc)]
    pub async fn get_by_filter(
        &self,
        request: GetByFilterRequest,
        token: &str
    ) -> Result<Response, Error> {
        let mut request = tonic::Request::new(request);
        Self::set_token_for_request(&mut request, token)?;

        let mut client_inner = self.connect().await?;

        let response = client_inner
            .get_by_filter(request)
            .await
            .map_err(|e| Error {
                kind: ErrorKind::ErrorRequestFailed,
                message: e.to_string(),
            })?;

        Ok(response.into_inner())
    }

    /// Aggregates data in the datastore based on the provided request.
    ///
    /// # Arguments
    /// * `request` - The aggregation request containing the aggregation criteria.
    ///
    /// # Returns
    /// A `Result` containing a `Response` if successful, or an `Error` if the operation fails.
    #[allow(clippy::missing_errors_doc)]
    pub async fn aggregate(&self, request: AggregateRequest, token: &str) -> Result<Response, Error> {
        let mut request = tonic::Request::new(request);
        Self::set_token_for_request(&mut request, token)?;

        let mut client_inner = self.connect().await?;

        let response = client_inner.aggregate(request).await.map_err(|e| Error {
            kind: ErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        Ok(response.into_inner())
    }

    /// Retrieves a record from the datastore by its ID.
    ///
    /// # Arguments
    /// * `request` - The request containing the ID of the record to retrieve.
    ///
    /// # Returns
    /// A `Result` containing a `Response` if successful, or an `Error` if the operation fails.
    #[allow(clippy::missing_errors_doc)]
    pub async fn get_by_id(&self, request: GetByIdRequest, token: &str) -> Result<Response, Error> {
        let mut request = tonic::Request::new(request);
        Self::set_token_for_request(&mut request, token)?;

        let mut client_inner = self.connect().await?;

        let response = client_inner.get_by_id(request).await.map_err(|e| Error {
            kind: ErrorKind::ErrorRequestFailed,
            message: e.to_string(),
        })?;

        Ok(response.into_inner())
    }
}
