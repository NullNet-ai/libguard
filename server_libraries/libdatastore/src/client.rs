use crate::datastore::store_service_client::StoreServiceClient;
use crate::utils::{authorize_request, validate_response_and_convert_to_reponse_data};
use crate::{datastore::*, DatastoreConfig, ResponseData};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use tonic::transport::{Channel, ClientTlsConfig};

/// A client for interacting with the datastore service.
#[derive(Debug, Clone)]
pub struct DatastoreClient {
    /// Configuration for connecting to the datastore.
    config: DatastoreConfig,
}

impl DatastoreClient {
    /// Creates a new instance of `DatastoreClient`.
    ///
    /// # Arguments
    /// * `config` - The configuration settings for connecting to the datastore.
    #[must_use]
    pub fn new(config: DatastoreConfig) -> Self {
        Self { config }
    }

    /// Establishes a connection to the datastore service.
    ///
    /// # Returns
    /// * `Ok(StoreServiceClient<Channel>)` - The client for interacting with the datastore service.
    /// * `Err(Error)` - If the connection fails.
    async fn connect(&self) -> Result<StoreServiceClient<Channel>, Error> {
        let protocol = if self.config.tls { "https" } else { "http" };
        let host = self.config.host.as_str();
        let port = self.config.port;

        let mut endpoint = Channel::from_shared(format!("{protocol}://{host}:{port}"))
            .handle_err(location!())?
            .connect_timeout(std::time::Duration::from_secs(10));

        if self.config.tls {
            endpoint = endpoint
                .tls_config(ClientTlsConfig::new().with_native_roots())
                .handle_err(location!())?;
        }

        let channel: Channel = endpoint.connect().await.handle_err(location!())?;

        Ok(StoreServiceClient::new(channel))
    }

    /// Logs in to the datastore service with the provided request.
    ///
    /// # Arguments
    /// * `request` - The login request containing the necessary credentials.
    ///
    /// # Returns
    /// * `Ok(LoginResponse)` - The response received after a successful login.
    /// * `Err(Error)` - If the login fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, Error> {
        let mut client_inner = self.connect().await?;

        let response = client_inner.login(request).await.handle_err(location!())?;

        Ok(response.into_inner())
    }

    /// Creates multiple records in the datastore with the provided request.
    ///
    /// # Arguments
    /// * `request` - The batch create request containing the records to be created.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the result of the operation.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn batch_create(
        &self,
        request: BatchCreateRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let mut client_inner = self.connect().await?;
        let request = authorize_request(request, token)?;

        let response = client_inner
            .batch_create(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.into_inner())
    }

    /// Creates a single record in the datastore with the provided request.
    ///
    /// # Arguments
    /// * `request` - The create request containing the record to be created.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the result of the operation.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn create(&self, request: CreateRequest, token: &str) -> Result<ResponseData, Error> {
        let mut client_inner = self.connect().await?;
        let request = authorize_request(request, token)?;

        let response = client_inner.create(request).await.handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.into_inner())
    }

    /// Deletes a record from the datastore with the provided request.
    ///
    /// # Arguments
    /// * `request` - The delete request containing the identifier of the record to be deleted.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the result of the operation.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn delete(&self, request: DeleteRequest, token: &str) -> Result<ResponseData, Error> {
        let mut client_inner = self.connect().await?;
        let request = authorize_request(request, token)?;

        let response = client_inner.delete(request).await.handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.into_inner())
    }

    /// Deletes multiple records from the datastore with the provided request.
    ///
    /// # Arguments
    /// * `request` - The batch delete request containing the identifiers of the records to be deleted.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the result of the operation.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn batch_delete(
        &self,
        request: BatchDeleteRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let mut client_inner = self.connect().await?;
        let request = authorize_request(request, token)?;

        let response = client_inner
            .batch_delete(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.into_inner())
    }

    /// Updates a record in the datastore with the provided request.
    ///
    /// # Arguments
    /// * `request` - The update request containing the record's updated data.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the result of the operation.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn update(&self, request: UpdateRequest, token: &str) -> Result<ResponseData, Error> {
        let mut client_inner = self.connect().await?;
        let request = authorize_request(request, token)?;

        let response = client_inner.update(request).await.handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.into_inner())
    }

    /// Updates multiple records in the datastore with the provided request.
    ///
    /// # Arguments
    /// * `request` - The batch update request containing the updated data for multiple records.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the result of the operation.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn batch_update(
        &self,
        request: BatchUpdateRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let mut client_inner = self.connect().await?;
        let request = authorize_request(request, token)?;

        let response = client_inner
            .batch_update(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.into_inner())
    }

    /// Retrieves records from the datastore based on the specified filter.
    ///
    /// # Arguments
    /// * `request` - The request containing the filter criteria.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the records that match the filter.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn get_by_filter(
        &self,
        request: GetByFilterRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let mut client_inner = self.connect().await?;
        let request = authorize_request(request, token)?;

        let response = client_inner
            .get_by_filter(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.into_inner())
    }

    /// Performs aggregation on records in the datastore based on the provided request.
    ///
    /// # Arguments
    /// * `request` - The aggregation request specifying the criteria for aggregation.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the result of the aggregation.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn aggregate(
        &self,
        request: AggregateRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let mut client_inner = self.connect().await?;
        let request = authorize_request(request, token)?;

        let response = client_inner
            .aggregate(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.into_inner())
    }

    /// Retrieves a record from the datastore by its identifier.
    ///
    /// # Arguments
    /// * `request` - The request containing the identifier of the record to be retrieved.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the requested record.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn get_by_id(
        &self,
        request: GetByIdRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let mut client_inner = self.connect().await?;
        let request = authorize_request(request, token)?;

        let response = client_inner
            .get_by_id(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.into_inner())
    }
}
