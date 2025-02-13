pub use detector::{Detector, State};
pub use error::{Error, ErrorKind};
pub use interface_snapshot::InterfaceSnapshot;
pub use platform::Platform;
pub use watcher::{
    r#impl::{Watcher, WatcherHandler},
    types::Snapshot,
};

mod detector;
mod error;
mod interface_snapshot;
mod platform;
mod watcher;

/// Creates and initializes a new `Watcher` to monitor file changes on a specified platform.
///
/// # Parameters
/// - `platform`: A string representing the target platform for the watcher (e.g., `"pfsense"` or `"opnsense"`).
/// - `poll_interval`: The polling interval in milliseconds to check for file changes.
/// - `handler`: A user-defined function or closure that gets executed when a change is detected.
///   This function must implement the `WatcherHandler` trait.
///
/// # Returns
/// - `Ok(Watcher<T>)`: A successfully initialized `Watcher` instance configured for the given platform.
/// - `Err(Error)`: Returns an error if initialization fails.
///
/// # Errors
/// - Returns `ErrorKind::ErrorUnsupportedPlatform` if the specified platform is not recognized.
/// - Returns `ErrorKind::ErrorInitializingWatcher` if the watcher fails to initialize.
pub async fn make_watcher<T>(
    platform: &str,
    poll_interval: u64,
    handler: T,
) -> Result<Watcher<T>, Error>
where
    T: WatcherHandler,
{
    let pval = Platform::from_string(platform)?;
    let retval = Watcher::new(pval, poll_interval, handler).await?;

    Ok(retval)
}
