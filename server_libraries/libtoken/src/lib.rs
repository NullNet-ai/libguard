mod models;

use base64::Engine as _;
use serde::Deserialize;

pub use models::{Account, Device, Organization};

/// Represents a decoded JWT payload containing account information and metadata.
/// Includes issue and expiration times for the token.
#[derive(Debug, Deserialize)]
pub struct Token {
    pub account: Account,
    pub iat: u64,
    pub exp: u64,
}

impl Token {
    /// Decodes a JWT and parses its payload into a `Token` struct.
    ///
    /// # Arguments
    /// * `jwt` - A JWT string consisting of three parts separated by periods (`.`).
    ///
    /// # Returns
    /// * `Ok(Token)` if the token is successfully decoded and parsed.
    /// * `Err(Error)` if the token is malformed, Base64 decoding fails, or payload deserialization fails.
    #[allow(clippy::missing_errors_doc)]
    pub fn from_jwt(jwt: &str) -> Result<Self, String> {
        let parts: Vec<&str> = jwt.split('.').collect();

        if parts.len() != 3 {
            return Err(String::from("Malformed JWT"));
        }

        let decoded_payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(parts[1])
            .map_err(|e| e.to_string())?;

        let token: Token = serde_json::from_slice(&decoded_payload).map_err(|e| e.to_string())?;

        Ok(token)
    }
}
