use crate::datastore::store_service_client::StoreServiceClient;
#[allow(clippy::wildcard_imports)]
use crate::datastore::*;
use crate::utils::{authorize_request, validate_response_and_convert_to_reponse_data};
use crate::{DatastoreConfig, ResponseData};
use nullnet_liberror::{Error, ErrorHandler, Location, location};
use tonic::transport::Channel;

/// A client for interacting with the datastore service.
#[derive(Debug, Clone)]
pub struct DatastoreClient {
    /// Datastore gRPC endpoint
    client: StoreServiceClient<Channel>,
}

impl DatastoreClient {
    /// Creates a new instance of `DatastoreClient`.
    ///
    /// # Arguments
    /// * `config` - The configuration settings for connecting to the datastore.
    #[allow(clippy::missing_errors_doc)]
    pub async fn new(config: DatastoreConfig) -> Result<Self, Error> {
        let channel = config.connect().await?;
        let client = StoreServiceClient::new(channel).max_decoding_message_size(50 * 1024 * 1024);
        Ok(Self { client })
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
    pub async fn login(&mut self, request: LoginRequest) -> Result<LoginResponse, Error> {
        let response = self.client.login(request).await.handle_err(location!())?;

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
        &mut self,
        request: BatchCreateRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self
            .client
            .batch_create(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
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
    pub async fn create(
        &mut self,
        request: CreateRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self.client.create(request).await.handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
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
    pub async fn delete(
        &mut self,
        request: DeleteRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self.client.delete(request).await.handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
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
        &mut self,
        request: BatchDeleteRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self
            .client
            .batch_delete(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
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
    pub async fn update(
        &mut self,
        request: UpdateRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self.client.update(request).await.handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
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
        &mut self,
        request: BatchUpdateRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self
            .client
            .batch_update(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
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
        &mut self,
        request: GetByFilterRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self
            .client
            .get_by_filter(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
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
        &mut self,
        request: AggregateRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self
            .client
            .aggregate(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
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
        &mut self,
        request: GetByIdRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self
            .client
            .get_by_id(request)
            .await
            .handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
    }

    /// Updates (if already present) or creates (if not) a record in the datastore.
    ///
    /// # Arguments
    /// * `request` - The request containing the record to be updated or created (based on a list of conflict columns).
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(ResponseData)` - The response data containing the result of the operation.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn upsert(
        &mut self,
        request: UpsertRequest,
        token: &str,
    ) -> Result<ResponseData, Error> {
        let request = authorize_request(request, token)?;

        let response = self.client.upsert(request).await.handle_err(location!())?;

        validate_response_and_convert_to_reponse_data(response.get_ref())
    }

    /// Registeres a new account
    ///
    /// # Arguments
    /// * `request` - The request containing the account info.
    /// * `token` - The authorization token to authorize the request.
    ///
    /// # Returns
    /// * `Ok(RegisterResponse)` - The response data containing the result of the operation.
    /// * `Err(Error)` - If the operation fails or if an error occurs during the process.
    #[allow(clippy::missing_errors_doc)]
    pub async fn register(
        &mut self,
        request: RegisterRequest,
        token: &str,
    ) -> Result<RegisterResponse, Error> {
        let request = authorize_request(request, token)?;

        let response = self
            .client
            .register(request)
            .await
            .handle_err(location!())?;

        return Ok(response.into_inner());
    }
}
