use crate::{Error, ErrorKind};

/// Represents the supported platforms.
#[derive(Clone, Copy, Debug)]
pub enum Platform {
    PfSense,
    OPNsense,
}

impl Platform {
    /// Converts a string representation into a `Platform` enum.
    ///
    /// # Parameters
    /// - `value`: A `String` containing the name of the platform.
    ///
    /// # Returns
    /// - `Ok(Platform)`: If the string matches a supported platform.
    /// - `Err(Error)`: If the string does not match any supported platform.
    ///
    /// # Errors
    /// Returns an `Error` with the kind `ErrorUnsupportedPlatform` if the input string is not recognized as a valid platform.
    pub fn from_string(value: String) -> Result<Platform, Error> {
        match value.as_str() {
            "pfsense" => Ok(Platform::PfSense),
            "opnsense" => Ok(Platform::OPNsense),
            _ => Err(Error {
                kind: ErrorKind::ErrorUnsupportedPlatform,
                message: format!("Unsupported platform: {}", value),
            }),
        }
    }
}
