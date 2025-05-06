use crate::{
    DatastoreConfig,
    store::{
        CreateConnectionsRequest, CreateConnectionsResponse, DeleteConnectionsRequest,
        DeleteConnectionsResponse, GetConnectionsRequest, GetConnectionsResponse,
        UpdateConnectionsRequest, UpdateConnectionsResponse,
        store_service_client::StoreServiceClient,
    },
};
use nullnet_liberror::{Error, ErrorHandler, Location, location};
use tonic::transport::Channel;

pub struct ExperimentalDatastoreClient {
    client: StoreServiceClient<Channel>,
}

impl ExperimentalDatastoreClient {
    #[allow(clippy::missing_errors_doc)]
    pub async fn new(config: DatastoreConfig) -> Result<Self, Error> {
        let channel = config.connect().await?;
        let client = StoreServiceClient::new(channel).max_decoding_message_size(50 * 1024 * 1024);
        Ok(Self { client })
    }

    pub async fn create_connections(
        &mut self,
        request: CreateConnectionsRequest,
    ) -> Result<CreateConnectionsResponse, Error> {
        let response = self
            .client
            .create_connections(request)
            .await
            .handle_err(location!())?;

        Ok(response.into_inner())
    }

    pub async fn get_connections(
        &mut self,
        request: GetConnectionsRequest,
    ) -> Result<GetConnectionsResponse, Error> {
        let response = self
            .client
            .get_connections(request)
            .await
            .handle_err(location!())?;

        Ok(response.into_inner())
    }

    pub async fn update_connections(
        &mut self,
        request: UpdateConnectionsRequest,
    ) -> Result<UpdateConnectionsResponse, Error> {
        let response = self
            .client
            .update_connections(request)
            .await
            .handle_err(location!())?;

        Ok(response.into_inner())
    }

    pub async fn delete_connections(
        &mut self,
        request: DeleteConnectionsRequest,
    ) -> Result<DeleteConnectionsResponse, Error> {
        let response = self
            .client
            .delete_connections(request)
            .await
            .handle_err(location!())?;

        Ok(response.into_inner())
    }
}
