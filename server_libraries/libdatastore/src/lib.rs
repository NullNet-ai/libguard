mod client;
mod config;

#[rustfmt::skip]
mod datastore;

pub use client::DatastoreClient;
pub use config::DatastoreConfig;
pub use datastore::{
    AdvanceFilter, AggregateBody, AggregateRequest, Aggregation, BatchCreateBody,
    BatchCreateRequest, CreateParams, CreateRequest, DeleteRequest, EntityFieldFrom, EntityFieldTo,
    FieldRelation, GetByFilterBody, GetByFilterRequest, GetByIdRequest, Join, LoginBody, LoginData,
    LoginRequest, LoginResponse, MultipleSort, Order, Params, Query, Response, UpdateRequest,
    Value,
};

/// Represents the different kinds of errors that can occur during configuration monitoring.
#[derive(Debug)]
pub enum ErrorKind {
    ErrorCouldNotConnectToDatastore,
    ErrorRequestFailed,
}

/// A structured error type for `libconfmon`.
///
/// # Fields
/// - `kind`: The specific type of error.
/// - `message`: A detailed message explaining the error.
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}
