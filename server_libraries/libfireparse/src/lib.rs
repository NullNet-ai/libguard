mod models;
mod pfsense;
mod utils;

pub use models::{Alias, Configuration, Rule};
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
    /// Parses a firewall configuration document based on the specified platform.
    ///
    /// # Arguments
    /// * `platform` - A string slice representing the firewall platform (e.g., `"pfsense"`).
    /// * `document` - A string slice containing the raw firewall configuration data.
    ///
    /// # Returns
    /// * `Ok(Configuration)` - If parsing is successful, returns a `Configuration` struct.
    /// * `Err(FireparseError)` - If the platform is unsupported or the document is invalid.
    ///
    /// # Supported Platforms
    /// - `"pfsense"`: Uses `PfSenseParser` to process pfSense XML configurations.
    pub fn parse(platform: &str, document: &str) -> Result<Configuration, FireparseError> {
        match platform.to_lowercase().as_str() {
            "pfsense" => PfSenseParser::parse(document),
            _ => {
                return Err(FireparseError::UnsupportedPlatform(format!(
                    "Platform {} is not supported",
                    platform
                )))
            }
        }
    }
}
