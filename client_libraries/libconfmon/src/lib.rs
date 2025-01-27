use platform::Platform;
use std::fmt::Display;
use std::{future::Future, path::PathBuf};
pub use watcher::{r#impl::Watcher, types::Snapshot};

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

impl Display for Error {
    /// Formats the `Error` for display.
    ///
    /// # Format
    /// The output includes the `ErrorKind` and the detailed message:
    /// `[ErrorKind] message`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)
    }
}

/// Creates a new `Watcher` to monitor file changes.
///
/// # Parameters
/// - `platform`: The platform for which the watcher is being created (e.g., `"PfSense"` or `"OPNsense"`).
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
/// let watcher = make_watcher("PfSense".to_string(), 1000, |snapshot| async move {
///     println!("Changes detected in snapshot: {:?}", snapshot);
/// }).await?;
/// ```
pub async fn make_watcher<F, Fut>(
    platform: &str,
    poll_interval: u64,
    callback: F,
) -> Result<Watcher<F, Fut>, Error>
where
    F: Fn(Snapshot) -> Fut,
    Fut: Future<Output = ()>,
{
    let pval = Platform::from_str(platform)?;
    let files = get_files_to_monitor(pval);
    let retval = Watcher::new(files, poll_interval, callback).await?;

    Ok(retval)
}

fn get_files_to_monitor(platform: Platform) -> Vec<PathBuf> {
    match platform {
        Platform::PfSense | Platform::OPNsense => vec![PathBuf::from("/conf/config.xml")],
    }
}
