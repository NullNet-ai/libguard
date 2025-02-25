mod client;
mod config;
mod response_data;
mod utils;

#[rustfmt::skip]
#[allow(clippy::pedantic)]
mod datastore;

pub use client::DatastoreClient;
pub use config::DatastoreConfig;
pub use datastore::*;
pub use response_data::ResponseData;
