use crate::{Response, ResponseData};
use nullnet_liberror::{Error, ErrorHandler, Location, location};
use std::str::FromStr;
use tonic::{Request, metadata::MetadataValue};

/// Adds an authorization token to the request metadata.
///
/// # Arguments
/// * `request` - The request object to be authorized.
/// * `token` - The authorization token as a string.
///
/// # Returns
/// * `Ok(Request<T>)` - The request with the authorization token included.
/// * `Err(Error)` - If the token cannot be parsed into a valid `MetadataValue`.
pub(crate) fn authorize_request<T>(request: T, token: &str) -> Result<Request<T>, Error> {
    let mut request = Request::new(request);

    let value = MetadataValue::from_str(token).handle_err(location!())?;

    request.metadata_mut().insert("authorization", value);

    Ok(request)
}

/// Validates a `Response` and converts it into a `ResponseData` if successful.
///
/// # Arguments
/// * `response` - The response object to validate and convert.
///
/// # Returns
/// * `Ok(ResponseData)` - If the response indicates success.
/// * `Err(Error)` - If the response indicates failure, an error is returned with details.
pub(crate) fn validate_response_and_convert_to_reponse_data(
    response: &Response,
) -> Result<ResponseData, Error> {
    if !response.success {
        return Err(format!(
            "Request failed. Status '{}'. Message '{}'. Error '{}'",
            response.status_code, response.message, response.error
        ))
        .handle_err(location!());
    }

    Ok(ResponseData {
        count: response.count,
        data: response.data.clone(),
        encoding: response.encoding.clone(),
    })
}
