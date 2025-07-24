mod models;
mod opnsense;
mod pfsense;
mod utils;

pub use models::*;
pub use nullnet_libconfmon::{FileData, Platform, Snapshot};
use pfsense::PfSenseParser;

use crate::opnsense::OpnSenseParser;

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
    /// - `Platform::OPNsense`:  Uses `OpnSenseParser` to process OPNsense configurations.
    pub fn parse(platfom: Platform, snapshot: Snapshot) -> Result<Configuration, FireparseError> {
        match platfom {
            Platform::PfSense => PfSenseParser::parse(snapshot),
            Platform::OPNsense => OpnSenseParser::parse(snapshot),
        }
    }
}
