mod models;
mod pfsense;
mod utils;

pub use models::{Alias, Configuration, Rule};
pub use nullnet_libconfmon::{Platform, Snapshot, FileData};
use pfsense::PfSenseParser;

/// Represents possible errors that can occur while parsing firewall configurations.
pub enum FireparseError {
    UnsupportedPlatform(String),
    ParserError(String),
}

/// A generic parser for firewall configuration files.
///
/// This parser determines the correct parsing logic based on the specified platform.
pub struct Parser {}

impl Parser {
    /// Parses a firewall configuration snapshot based on the specified platform.
    ///
    /// # Arguments
    /// * `platform` - The firewall platform (e.g., `Platform::PfSense` or `Platform::OPNsense`).
    /// * `snapshot` - A `Snapshot` representing the firewall configuration state.
    ///
    /// # Returns
    /// * `Ok(Configuration)` - If parsing is successful, returns a `Configuration` struct.
    /// * `Err(FireparseError)` - If the platform is unsupported or the snapshot is invalid.
    ///
    /// # Supported Platforms
    /// - `Platform::PfSense`: Uses `PfSenseParser` to process pfSense configurations.
    /// - `Platform::OPNsense`: Currently not implemented (`todo!()` placeholder).
    pub fn parse(platfom: Platform, snapshot: Snapshot) -> Result<Configuration, FireparseError> {
        match platfom {
            Platform::PfSense => PfSenseParser::parse(snapshot),
            Platform::OPNsense => todo!(),
        }
    }
}
