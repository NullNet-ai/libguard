use std::{path::PathBuf, time::SystemTime};
use tokio::fs;

use crate::{Error, ErrorKind};

/// Creates an error mapper for a specific `ErrorKind`.
///
/// # Parameters
/// - `kind`: The type of error to associate with the mapped error.
///
/// # Returns
/// A closure that takes an error implementing `Display` and maps it to an `Error`
/// with the specified `ErrorKind` and the error message.
pub fn make_error_mapper<E: std::fmt::Display + 'static>(
    kind: ErrorKind,
) -> impl FnOnce(E) -> Error {
    move |error| Error {
        kind,
        message: error.to_string(),
    }
}

/// Gets the modification time (mtime) of a file as a number of milliseconds since the UNIX epoch.
///
/// # Parameters
/// - `path`: A reference to a `PathBuf` representing the file path.
///
/// # Returns
/// - `Ok(u128)`: The modification time in milliseconds since the UNIX epoch if successful.
/// - `Err(Error)`: An error of type `Error` if retrieving the metadata or modified time fails.
///
/// # Errors
/// - Returns an error with `ErrorKind::ErrorReadingFile` if the file metadata or modified time
///   cannot be read or converted.
pub async fn get_mtime(path: &PathBuf) -> Result<u128, Error> {
    let value = fs::metadata(path)
        .await
        .map_err(make_error_mapper(ErrorKind::ErrorReadingFile))?
        .modified()
        .map_err(make_error_mapper(ErrorKind::ErrorReadingFile))?
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(make_error_mapper(ErrorKind::ErrorReadingFile))?;

    Ok(value.as_millis())
}
