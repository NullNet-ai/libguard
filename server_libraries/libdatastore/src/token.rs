use crate::{Error, ErrorKind};
use base64::Engine as _;
use serde::Deserialize;

/// Represents a device associated with an account.
/// Contains detailed information about the device, including identifiers, timestamps, and location data.
#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: String,
    pub categories: Vec<String>,
    pub code: String,
    pub tombstone: u32,
    pub status: String,
    pub version: u32,
    pub created_date: String,
    pub created_time: String,
    pub updated_date: String,
    pub updated_time: String,
    pub organization_id: String,
    pub created_by: String,
    pub updated_by: String,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    pub timestamp: Option<String>,
    pub tags: Vec<String>,
    pub model: String,
    pub country: String,
    pub city: String,
    pub state: String,
    pub instance_name: String,
    pub is_connection_established: bool,
}

/// Represents an organization associated with an account.
/// Contains metadata about the organization, including its identifiers and hierarchy.
#[derive(Debug, Deserialize)]
pub struct Organization {
    pub id: String,
    pub categories: Vec<String>,
    pub code: Option<String>,
    pub tombstone: u32,
    pub status: String,
    pub version: u32,
    pub created_date: String,
    pub created_time: String,
    pub updated_date: String,
    pub updated_time: String,
    pub organization_id: String,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    pub timestamp: Option<String>,
    pub tags: Vec<String>,
    pub parent_organization_id: Option<String>,
    pub name: String,
}

/// Represents an account containing a device and organization.
/// Acts as a container for the relationships between devices and organizations.
#[derive(Debug, Deserialize)]
pub struct Account {
    pub device: Device,
    pub organization: Organization,
    pub organization_id: String,
    pub account_id: String,
}

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
    pub fn from_jwt(jwt: &str) -> Result<Self, Error> {
        let parts: Vec<&str> = jwt.split('.').collect();

        if parts.len() != 3 {
            return Err(Error {
                kind: ErrorKind::ErrorBadToken,
                message: String::from("Malformed JWT"),
            });
        }

        let decoded_payload = base64::engine::general_purpose::STANDARD
            .decode(parts[1])
            .map_err(|e| Error {
                kind: ErrorKind::ErrorBadToken,
                message: e.to_string(),
            })?;

        let token: Token = serde_json::from_slice(&decoded_payload).map_err(|e| Error {
            kind: ErrorKind::ErrorBadToken,
            message: e.to_string(),
        })?;

        Ok(token)
    }
}
