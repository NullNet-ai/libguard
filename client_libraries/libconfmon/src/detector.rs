use crate::Platform;
use std::ffi::OsStr;
use std::path::Path;
use tokio::fs;
use tokio::fs::ReadDir;

/// Represents the possible states of the system configuration.
#[derive(Debug, PartialEq)]
pub enum State {
    /// The configuration is in draft mode and has not been applied.
    Draft,
    /// The configuration has been applied to the system.
    Applied,
    /// The state of the configuration is undefined due to an error or missing information.
    Undefined,
}

/// A detector responsible for determining whether the system configuration is in draft mode or applied.
pub struct Detector {}

impl Detector {
    /// Checks the current configuration state based on the platform.
    ///
    /// # Parameters
    /// - `platform`: The target platform for which the configuration state is being checked.
    ///
    /// # Returns
    /// - `State::Draft` if the configuration is still in draft mode.
    /// - `State::Applied` if the configuration has been applied.
    /// - `State::Undefined` if the check fails or the state cannot be determined.
    pub async fn check(platform: Platform) -> State {
        match platform {
            Platform::PfSense => Detector::check_pfsense().await,
            Platform::OPNsense => todo!("Not implemented"),
        }
    }

    /// Checks the configuration state for the **PfSense** platform.
    ///
    /// # Returns
    /// - `State::Draft` if a file with a `.dirty` extension exists in `/var/run/`, indicating pending changes.
    /// - `State::Applied` if no such files are found.
    /// - `State::Undefined` if an error occurs while reading the directory.
    async fn check_pfsense() -> State {
        let mut entries: ReadDir = match fs::read_dir("/var/run/").await {
            Ok(entries) => entries,
            Err(_) => return State::Undefined,
        };

        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Some(ext) = Path::new(&entry.file_name())
                .extension()
                .and_then(OsStr::to_str)
            {
                if ext == "dirty" {
                    return State::Draft;
                }
            }
        }

        State::Applied
    }
}
