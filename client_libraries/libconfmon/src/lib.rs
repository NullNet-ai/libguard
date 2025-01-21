use platform::Platform;
use std::{future::Future, path::PathBuf};
use watcher::{types::Snapshot, watcher::Watcher};

mod platform;
mod watcher;

/// Represents the different kinds of errors that can occur during configuration monitoring.
#[derive(Debug)]
pub enum ErrorKind {
    ErrorInitializingWatcher,
    ErrorWatchingFile,
    ErrorReadingFile,
    ErrorUnsupportedPlatform,
}

/// A structured error type for `libconfmon`.
///
/// # Fields
/// - `kind`: The specific type of error.
/// - `message`: A detailed message explaining the error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

/// Creates a new `Watcher` instance based on the provided platform and callback function.
///
/// # Parameters
/// - `platform`: A `String` representing the platform type.
/// - `callback`: An asynchronous callback function to handle events, which takes a `Snapshot` as input.
///
/// # Returns
/// - `Ok(Watcher<F, Fut>)`: A watcher instance configured for the specified platform and callback.
/// - `Err(Error)`: If the platform is unsupported or any other error occurs during initialization.
///
/// # Type Parameters
/// - `F`: The type of the callback function. It must be a function or closure that takes a `Snapshot` and returns a `Future`.
/// - `Fut`: The type of the `Future` returned by the callback, which must resolve to `()`.
///
/// # Errors
/// - Returns an `Error` with `ErrorKind::ErrorUnsupportedPlatform` if the provided platform string is invalid.
///
/// # Examples
/// ```
/// use crate::{make_watcher, Snapshot};
///
/// let watcher = make_watcher("pfsense".to_string(), async move |snapshot| {
///     println!("File change detected: {:?}", snapshot);
/// }).await;
///
/// assert!(watcher.is_ok());
/// ```
pub async fn make_watcher<F, Fut>(platform: String, callback: F) -> Result<Watcher<F, Fut>, Error>
where
    F: Fn(Snapshot) -> Fut,
    Fut: Future<Output = ()>,
{
    let pval = Platform::from_string(platform)?;
    let files = get_files_to_monitor(pval);

    Ok(Watcher::new(files, callback))
}

/// Determines the files to monitor based on the specified platform.
///
/// # Parameters
/// - `platform`: The `Platform` enum representing the target platform.
///
/// # Returns
/// - A `Vec<PathBuf>` containing the paths to files that should be monitored.
fn get_files_to_monitor(platform: Platform) -> Vec<PathBuf> {
    match platform {
        Platform::PfSense | Platform::OPNsense => vec![PathBuf::from("/conf/config.xml")],
    }
}
