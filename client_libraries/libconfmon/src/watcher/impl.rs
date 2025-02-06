use super::{
    types::{FileData, FileInfo, Snapshot},
    utils::{get_mtime, make_error_mapper},
};
use crate::{Detector, Error, ErrorKind, Platform, State};
use std::{path::PathBuf, time::Duration};

/// A simple file watcher that monitors changes in a list of files and triggers appropriate handlers.
#[allow(async_fn_in_trait)]
pub trait WatcherHandler {
    /// Defines how the snapshot should be uploaded or processed.
    ///
    /// # Parameters
    /// - `snapshot`: A snapshot of the monitored files, containing file metadata and content.
    ///
    /// # Returns
    /// - `Ok(())` if processing is successful.
    /// - `Err(Error)` if an error occurs while processing the snapshot.
    async fn on_snapshot(&self, snapshot: Snapshot, state: State) -> Result<(), Error>;

    /// Handles errors that occur during file monitoring.
    ///
    /// # Parameters
    /// - `error`: The error encountered during monitoring.
    async fn on_error(&self, error: Error);
}

/// A file watcher that monitors specified files for changes and notifies a handler when updates occur.
pub struct Watcher<H: WatcherHandler> {
    /// List of monitored files and their metadata.
    files: Vec<FileInfo>,
    /// Polling interval (in milliseconds) for checking file modifications.
    poll_interval: u64,
    /// Handler for processing snapshots and handling errors.
    handler: H,
    /// Target platform
    platform: Platform,
}

impl<H: WatcherHandler> Watcher<H> {
    /// Creates a new `Watcher` instance that monitors configuration files for changes.
    ///
    /// # Parameters
    /// - `platform`: The target platform for which the configuration state should be monitored.
    /// - `poll_interval`: Time interval (in milliseconds) to check for file changes.
    /// - `handler`: An instance implementing `WatcherHandler` for handling snapshots and errors.
    ///
    /// # Returns
    /// - `Ok(Self)`: A properly initialized `Watcher` instance.
    /// - `Err(Error)`: If any file metadata cannot be retrieved.
    pub async fn new(platform: Platform, poll_interval: u64, handler: H) -> Result<Self, Error> {
        let mut files = Vec::new();

        for path in get_files_to_monitor(platform) {
            let mtime = get_mtime(&path)
                .await
                .map_err(make_error_mapper(ErrorKind::ErrorInitializingWatcher))?;

            files.push(FileInfo { path, mtime });
        }

        Ok(Self {
            files,
            poll_interval,
            handler,
            platform,
        })
    }

    /// Starts monitoring the files for changes.
    ///
    /// This function continuously checks the monitored files for modifications.
    /// When a change is detected, it triggers the `on_snapshot` method of the handler.
    ///
    /// # Returns
    /// - `Ok(())` if the monitoring process runs smoothly.
    /// - `Err(Error)` if an unrecoverable error occurs.
    pub async fn watch(&mut self) {
        loop {
            let mut should_upload = false;
            for file in &mut self.files {
                match get_mtime(&file.path).await {
                    Ok(current) => {
                        if current > file.mtime {
                            file.mtime = current;
                            should_upload = true;
                        }
                    }
                    Err(err) => {
                        self.handler.on_error(err).await;
                    }
                }
            }

            if should_upload {
                match self.snapshot().await {
                    Ok(snapshot) => {
                        let state = Detector::check(self.platform).await;
                        if let Err(err) = self.handler.on_snapshot(snapshot, state).await {
                            self.handler.on_error(err).await;
                        }
                    }
                    Err(err) => {
                        self.handler.on_error(err).await;
                    }
                }
            }

            tokio::time::sleep(Duration::from_millis(self.poll_interval)).await;
        }
    }

    /// Generates a snapshot of the current state of the monitored files.
    ///
    /// # Returns
    /// - `Ok(Snapshot)`: A snapshot containing the contents and metadata of monitored files.
    /// - `Err(Error)`: If a file cannot be read.
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

/// Returns a list of files that should be monitored based on the given platform.
///
/// # Parameters
/// - `platform`: The target platform for which files need to be monitored.
///
/// # Returns
/// - `Vec<PathBuf>`: A vector containing paths to the configuration files that need monitoring.
fn get_files_to_monitor(platform: Platform) -> Vec<PathBuf> {
    match platform {
        Platform::PfSense | Platform::OPNsense => vec![PathBuf::from("/conf/config.xml")],
    }
}
