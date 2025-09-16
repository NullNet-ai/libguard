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
pub mod store;
mod builders;

pub use client::DatastoreClient;
pub use config::DatastoreConfig;
pub use datastore::*;
pub use response_data::ResponseData;

pub use experimental::ExperimentalDatastoreClient;
