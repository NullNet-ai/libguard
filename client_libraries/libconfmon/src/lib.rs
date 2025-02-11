pub use detector::{Detector, State};
pub use error::{Error, ErrorKind};
pub use platform::Platform;
pub use watcher::{
    r#impl::{Watcher, WatcherHandler},
    types::Snapshot,
};

mod detector;
mod error;
mod platform;
mod watcher;

/// Creates a new `Watcher` to monitor file changes.
///
/// # Parameters
/// - `platform`: The platform for which the watcher is being created (e.g., `"pfsense"` or `"opnsense"`).
/// - `poll_interval`: The interval (in milliseconds) at which files are polled for changes.
/// - `callback`: A closure or function to execute when changes are detected.
///
/// # Returns
/// - `Ok(Watcher<F, Fut>)`: A new instance of the `Watcher` if successfully initialized.
/// - `Err(Error)`: An error if initialization fails.
///
/// # Errors
/// - Returns an error with `ErrorKind::ErrorUnsupportedPlatform` if the platform is not supported.
/// - Returns an error with `ErrorKind::ErrorInitializingWatcher` if the watcher fails to initialize.
///
/// # Example
/// ```rust
/// use nullnet_libconfmon::{make_watcher, Error};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let watcher = make_watcher("pfsense", 1000, |snapshot| async move {
///         println!("Changes detected in snapshot: {:?}", snapshot);
///     }).await?;
///     Ok(())
/// }
/// ```
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
