mod client;
mod config;
mod experimental;
mod response_data;
mod utils;

#[rustfmt::skip]
#[allow(clippy::pedantic)]
mod datastore;

#[rustfmt::skip]
#[allow(clippy::pedantic)]
mod store;

pub use client::DatastoreClient;
pub use config::DatastoreConfig;
pub use datastore::*;
pub use response_data::ResponseData;

pub use experimental::ExperimentalDatastoreClient;
pub use store::{
    CreateConnectionsRequest, CreateConnectionsResponse, DeleteConnectionsRequest,
    DeleteConnectionsResponse, GetConnectionsRequest, GetConnectionsResponse,
    UpdateConnectionsRequest, UpdateConnectionsResponse,
};
