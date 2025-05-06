use crate::{
    DatastoreConfig,
    store::{self, store_service_client::StoreServiceClient},
};
use nullnet_liberror::{Error, ErrorHandler, Location, location};
use tonic::transport::Channel;

#[derive(Debug, Clone)]
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
        request: store::CreateConnectionsRequest,
    ) -> Result<store::CreateConnectionsResponse, Error> {
        let response = self
            .client
            .create_connections(request)
            .await
            .handle_err(location!())?;

        Ok(response.into_inner())
    }

    pub async fn get_connections(
        &mut self,
        request: store::GetConnectionsRequest,
    ) -> Result<store::GetConnectionsResponse, Error> {
        let response = self
            .client
            .get_connections(request)
            .await
            .handle_err(location!())?;

        Ok(response.into_inner())
    }

    pub async fn update_connections(
        &mut self,
        request: store::UpdateConnectionsRequest,
    ) -> Result<store::UpdateConnectionsResponse, Error> {
        let response = self
            .client
            .update_connections(request)
            .await
            .handle_err(location!())?;

        Ok(response.into_inner())
    }

    pub async fn delete_connections(
        &mut self,
        request: store::DeleteConnectionsRequest,
    ) -> Result<store::DeleteConnectionsResponse, Error> {
        let response = self
            .client
            .delete_connections(request)
            .await
            .handle_err(location!())?;

        Ok(response.into_inner())
    }
}
