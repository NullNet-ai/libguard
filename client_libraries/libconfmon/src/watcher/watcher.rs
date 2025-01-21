use super::types::{FileData, Snapshot};
use crate::{Error, ErrorKind};

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher as _};
use std::{future::Future, path::PathBuf};
use tokio::{fs::read, sync::mpsc};

/// The `Watcher` struct monitors specified files for changes and invokes a callback when modifications are detected.
///
/// # Type Parameters
/// - `F`: The type of the callback function. It must be a function or closure that takes a `Snapshot` and returns a `Future`.
/// - `Fut`: The type of the `Future` returned by the callback. It must resolve to `()`.
///
/// # Fields
/// - `files`: A vector of file paths to be watched.
/// - `callback`: A callback function that is executed asynchronously when file changes are detected. The function receives a `Snapshot` as input.

pub struct Watcher<F, Fut>
where
    F: Fn(Snapshot) -> Fut,
    Fut: Future<Output = ()>,
{
    files: Vec<PathBuf>,
    callback: F,
}

impl<F, Fut> Watcher<F, Fut>
where
    F: Fn(Snapshot) -> Fut,
    Fut: Future<Output = ()>,
{
    /// Creates a new `Watcher` instance.
    ///
    /// # Parameters
    /// - `files`: A vector of file paths to monitor.
    /// - `callback`: A callback function that is executed asynchronously when changes are detected.
    ///
    /// # Returns
    /// A new `Watcher` instance.
    pub fn new(files: Vec<PathBuf>, callback: F) -> Self {
        Self { files, callback }
    }

    /// Starts watching the specified files for changes asynchronously.
    ///
    /// # Returns
    /// - `Ok(())`: If the watcher is initialized and starts successfully.
    /// - `Err(Error)`: If there is an issue initializing the watcher or watching files.
    pub async fn watch(&self) -> Result<(), Error> {
        let (tx, mut rx) = mpsc::unbounded_channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            Config::default(),
        )
        .map_err(|e| Error {
            kind: ErrorKind::ErrorInitializingWatcher,
            message: e.to_string(),
        })?;

        for file in &self.files {
            watcher
                .watch(file, RecursiveMode::NonRecursive)
                .map_err(|e| Error {
                    kind: ErrorKind::ErrorWatchingFile,
                    message: e.to_string(),
                })?;
        }

        while let Some(event) = rx.recv().await {
            self.handle_event(event).await;
        }

        Ok(())
    }

    /// Captures a snapshot of the current state of the monitored files.
    ///
    /// # Returns
    /// - `Ok(Snapshot)`: A snapshot containing file data.
    /// - `Err(Error)`: If reading any file fails.
    pub async fn snapshot(&self) -> Result<Snapshot, Error> {
        let mut snapshot = Snapshot::new();

        for file in &self.files {
            let content = read(file).await.map_err(|e| Error {
                kind: ErrorKind::ErrorReadingFile,
                message: e.to_string(),
            })?;

            let filename = file
                .file_name()
                .unwrap_or(file.as_os_str())
                .to_string_lossy()
                .into_owned();

            snapshot.push(FileData { filename, content });
        }

        Ok(snapshot)
    }

    /// Handles file modification events by triggering the callback with the most recent snapshot.
    ///
    /// # Parameters
    /// - `event`: The event representing a file modification.
    async fn handle_event(&self, event: Event) {
        if !event.kind.is_modify() {
            return;
        }

        match self.snapshot().await {
            Ok(snapshot) => (self.callback)(snapshot).await,
            Err(error) => eprintln!("Watcher: Failed to take snapshot: {:?}", error),
        }
    }
}
