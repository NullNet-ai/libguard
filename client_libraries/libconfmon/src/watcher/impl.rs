use super::{
    types::{FileData, FileInfo, Snapshot},
    utils::{get_mtime, make_error_mapper},
};
use crate::{Error, ErrorKind};
use std::{future::Future, path::PathBuf, time::Duration};

/// A file watcher that monitors changes in a list of files and triggers a callback when changes are detected.
///
/// # Generics
/// - `F`: A closure or function that takes a `Snapshot` and produces a future.
/// - `Fut`: The type of the future returned by the callback.
///
/// # Fields
/// - `files`: A list of `FileInfo` containing metadata about the monitored files.
/// - `callback`: A closure or function to execute when changes are detected.
/// - `poll_interval`: The interval (in milliseconds) at which files are polled for changes.
pub struct Watcher<F, Fut>
where
    F: Fn(Snapshot) -> Fut,
    Fut: Future<Output = ()>,
{
    files: Vec<FileInfo>,
    callback: F,
    poll_interval: u64,
}

impl<F, Fut> Watcher<F, Fut>
where
    F: Fn(Snapshot) -> Fut,
    Fut: Future<Output = ()>,
{
    /// Creates a new `Watcher` instance.
    ///
    /// # Parameters
    /// - `paths`: A list of paths to the files to monitor.
    /// - `poll_interval`: The interval (in milliseconds) at which files are polled for changes.
    /// - `callback`: A closure or function to execute when changes are detected.
    ///
    /// # Returns
    /// - `Ok(Self)`: A new instance of `Watcher` if all files are successfully initialized.
    /// - `Err(Error)`: An error if any file cannot be initialized (e.g., failed to read metadata).
    ///
    /// # Errors
    /// - Returns an error with `ErrorKind::ErrorInitializingWatcher` if a file's metadata cannot be read.
    pub async fn new(paths: Vec<PathBuf>, poll_interval: u64, callback: F) -> Result<Self, Error> {
        let mut files = Vec::new();

        for path in paths {
            let mtime = get_mtime(&path)
                .await
                .map_err(make_error_mapper(ErrorKind::ErrorInitializingWatcher))?;

            files.push(FileInfo { mtime, path });
        }

        Ok(Self {
            files,
            poll_interval,
            callback,
        })
    }

    /// Starts monitoring the files for changes.
    ///
    /// # Behavior
    /// - Periodically checks the modification time of each file.
    /// - If any file has been modified since the last check, triggers the `callback` with a `Snapshot`.
    /// - Continues indefinitely until the task is canceled.
    ///
    /// # Returns
    /// - `Ok(())`: This function only exits if the task is canceled or an error occurs.
    /// - `Err(Error)`: An error if reading a file's metadata or content fails.
    ///
    /// # Errors
    /// - Returns an error with `ErrorKind::ErrorReadingFile` if file metadata or content cannot be read.
    pub async fn watch(&mut self) -> Result<(), Error> {
        loop {
            let mut should_upload = false;
            for file in &mut self.files {
                let current = get_mtime(&file.path).await?;

                if current > file.mtime {
                    file.mtime = current;
                    should_upload = true;
                }
            }

            if should_upload {
                let snapshot = self.snapshot().await?;
                (self.callback)(snapshot).await;
            }

            tokio::time::sleep(Duration::from_millis(self.poll_interval)).await
        }
    }

    /// Generates a snapshot of the current state of the monitored files.
    ///
    /// # Returns
    /// - `Ok(Snapshot)`: A snapshot containing the data of all monitored files.
    /// - `Err(Error)`: An error if any file cannot be read.
    ///
    /// # Errors
    /// - Returns an error with `ErrorKind::ErrorReadingFile` if a file cannot be read.
    pub async fn snapshot(&self) -> Result<Snapshot, Error> {
        let mut snapshot = Snapshot::new();

        for file in &self.files {
            let content = tokio::fs::read(&file.path)
                .await
                .map_err(make_error_mapper(ErrorKind::ErrorReadingFile))?;

            let filename = file
                .path
                .file_name()
                .unwrap_or(file.path.as_os_str())
                .to_string_lossy()
                .into_owned();

            snapshot.push(FileData { filename, content });
        }

        Ok(snapshot)
    }
}
