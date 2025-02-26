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

// sample user token
// eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhY2NvdW50Ijp7ImNvbnRhY3QiOnsiaWQiOiIwOWIzN2IxMS02MWJlLTRmYjctODg0Ni0zNTY4NGVmZjExZDEiLCJjYXRlZ29yaWVzIjpbIkNvbnRhY3QiXSwiY29kZSI6bnVsbCwidG9tYnN0b25lIjowLCJzdGF0dXMiOiJBY3RpdmUiLCJwcmV2aW91c19zdGF0dXMiOm51bGwsInZlcnNpb24iOjEsImNyZWF0ZWRfZGF0ZSI6IjIwMjUvMDIvMjYiLCJjcmVhdGVkX3RpbWUiOiIxOjIxOjQwIHAubS4iLCJ1cGRhdGVkX2RhdGUiOiIyMDI1LzAyLzI2IiwidXBkYXRlZF90aW1lIjoiMToyMTo0MCBwLm0uIiwib3JnYW5pemF0aW9uX2lkIjoiZWUxYjlhNTAtNTFlYy00ZWNmLWJjYzItOGY5NTExZjlmZWI4IiwiY3JlYXRlZF9ieSI6bnVsbCwidXBkYXRlZF9ieSI6bnVsbCwiZGVsZXRlZF9ieSI6bnVsbCwicmVxdWVzdGVkX2J5IjpudWxsLCJ0aW1lc3RhbXAiOm51bGwsInRhZ3MiOltdLCJmaXJzdF9uYW1lIjoiU3VwZXIiLCJtaWRkbGVfbmFtZSI6bnVsbCwibGFzdF9uYW1lIjoiQWRtaW4iLCJkYXRlX29mX2JpcnRoIjpudWxsfSwib3JnYW5pemF0aW9uIjp7ImlkIjoiZWUxYjlhNTAtNTFlYy00ZWNmLWJjYzItOGY5NTExZjlmZWI4IiwiY2F0ZWdvcmllcyI6W10sImNvZGUiOm51bGwsInRvbWJzdG9uZSI6MCwic3RhdHVzIjoiQWN0aXZlIiwicHJldmlvdXNfc3RhdHVzIjpudWxsLCJ2ZXJzaW9uIjoxLCJjcmVhdGVkX2RhdGUiOiIyMDI1LzAyLzI2IiwiY3JlYXRlZF90aW1lIjoiMToyMTo0MCBwLm0uIiwidXBkYXRlZF9kYXRlIjoiMjAyNS8wMi8yNiIsInVwZGF0ZWRfdGltZSI6IjE6MjE6NDAgcC5tLiIsIm9yZ2FuaXphdGlvbl9pZCI6ImVlMWI5YTUwLTUxZWMtNGVjZi1iY2MyLThmOTUxMWY5ZmViOCIsImNyZWF0ZWRfYnkiOm51bGwsInVwZGF0ZWRfYnkiOm51bGwsImRlbGV0ZWRfYnkiOm51bGwsInJlcXVlc3RlZF9ieSI6bnVsbCwidGltZXN0YW1wIjpudWxsLCJ0YWdzIjpbXSwicGFyZW50X29yZ2FuaXphdGlvbl9pZCI6bnVsbCwibmFtZSI6Imdsb2JhbC1vcmdhbml6YXRpb24ifSwib3JnYW5pemF0aW9uX2lkIjoiZWUxYjlhNTAtNTFlYy00ZWNmLWJjYzItOGY5NTExZjlmZWI4IiwiYWNjb3VudF9pZCI6ImFkbWluQGRuYW1pY3JvLmNvbSIsIm9yZ2FuaXphdGlvbl9hY2NvdW50X2lkIjoiMDliMzdiMTEtNjFiZS00ZmI3LTg4NDYtMzU2ODRlZmYxMWQxIn0sImlhdCI6MTc0MDYwNDk2MywiZXhwIjoxNzQwNzc3NzYzfQ.-cd4jhqUufohzb_3KHUOhNi-4N2l04jv4vj2oYpUiEc
//
// sample device token
