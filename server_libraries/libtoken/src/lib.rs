pub mod models;

use base64::Engine as _;
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

const EXPIRATION_MARGIN: u64 = 60 * 5;

/// Represents a decoded JWT payload containing account information and metadata.
/// Includes issue and expiration times for the token.
#[derive(Debug, Deserialize)]
pub struct Token {
    pub account: models::Account,
    pub signed_in_account: models::Account,
    pub iat: u64,
    pub exp: u64,
    #[serde(skip)]
    pub jwt: String,
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

        let mut token: Token =
            serde_json::from_slice(&decoded_payload).map_err(|e| e.to_string())?;
        token.jwt = jwt.to_string();

        Ok(token)
    }

    /// Checks if the token has expired.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        // consider the token expired if duration_since fails
        let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) else {
            return true;
        };
        self.exp <= (duration.as_secs() - EXPIRATION_MARGIN)
    }
}

#[cfg(test)]
mod tests {
    use crate::Token;

    #[test]
    fn test_device_issued_to_a_device() {
        let token = concat!(
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhY2NvdW50Ijp7InByb2ZpbGUiOnsiaWQiOiIwMUp",
            "ZMkM0NVQ3VjNLRVE1TjBKOU43SFpKRiIsImZpcnN0X25hbWUiOm51bGwsImxhc3RfbmFtZSI6bnVsbCw",
            "iZW1haWwiOiJzeXN0ZW1fZGV2aWNlIiwiYWNjb3VudF9pZCI6IjAxSlQxUjFCMlhNRERCN1dZM0pDODR",
            "EVjU1IiwiY2F0ZWdvcmllcyI6W10sImNvZGUiOm51bGwsInN0YXR1cyI6IkFjdGl2ZSIsIm9yZ2FuaXp",
            "hdGlvbl9pZCI6IjAxSlkyQzQ1TkdXMVJUSERHV1NFV0Y5TkhGIn0sImNvbnRhY3QiOm51bGwsImRldml",
            "jZSI6eyJpZCI6IjAxSlQxUjFCMlhNRERCN1dZM0pDODREVjU1IiwiY29kZSI6IkRWMDAwMDAxIiwiY2F",
            "0ZWdvcmllcyI6WyJEZXZpY2UiXSwic3RhdHVzIjoiRHJhZnQiLCJvcmdhbml6YXRpb25faWQiOiIwMUp",
            "CSEtYSFlTS1BQMjQ3SFpaV0hBM0pDVCIsInRpbWVzdGFtcCI6bnVsbH0sIm9yZ2FuaXphdGlvbiI6eyJ",
            "pZCI6IjAxSkJIS1hIWVNLUFAyNDdIWlpXSEEzSkNUIiwibmFtZSI6Imdsb2JhbC1vcmdhbml6YXRpb24",
            "iLCJjb2RlIjoiT1IwMDAwMDIiLCJjYXRlZ29yaWVzIjpbIlRlYW0iXSwic3RhdHVzIjoiQWN0aXZlIiw",
            "ib3JnYW5pemF0aW9uX2lkIjoiMDFKQkhLWEhZU0tQUDI0N0haWldIQTNKQ1QiLCJwYXJlbnRfb3JnYW5",
            "pemF0aW9uX2lkIjpudWxsfSwiaWQiOiIwMUpUMVIxQjJYTUREQjdXWTNKQzg0RFY1NSIsImFjY291bnR",
            "faWQiOiJzeXN0ZW1fZGV2aWNlIiwib3JnYW5pemF0aW9uX2lkIjoiMDFKQkhLWEhZU0tQUDI0N0haWld",
            "IQTNKQ1QiLCJhY2NvdW50X29yZ2FuaXphdGlvbl9pZCI6IjAxSlkyQzQ1WEo5MEtTU1laWFNNU0YyMDI",
            "4IiwiYWNjb3VudF9zdGF0dXMiOiJBY3RpdmUiLCJyb2xlX2lkIjpudWxsfSwic2lnbmVkX2luX2FjY29",
            "1bnQiOnsicHJvZmlsZSI6eyJpZCI6IjAxSlkyQzQ1VDdWM0tFUTVOMEo5TjdIWkpGIiwiZmlyc3RfbmF",
            "tZSI6bnVsbCwibGFzdF9uYW1lIjpudWxsLCJlbWFpbCI6InN5c3RlbV9kZXZpY2UiLCJhY2NvdW50X2l",
            "kIjoiMDFKVDFSMUIyWE1EREI3V1kzSkM4NERWNTUiLCJjYXRlZ29yaWVzIjpbXSwiY29kZSI6bnVsbCw",
            "ic3RhdHVzIjoiQWN0aXZlIiwib3JnYW5pemF0aW9uX2lkIjoiMDFKWTJDNDVOR1cxUlRIREdXU0VXRjl",
            "OSEYifSwiY29udGFjdCI6bnVsbCwiZGV2aWNlIjp7ImlkIjoiMDFKVDFSMUIyWE1EREI3V1kzSkM4NER",
            "WNTUiLCJjb2RlIjoiRFYwMDAwMDEiLCJjYXRlZ29yaWVzIjpbIkRldmljZSJdLCJzdGF0dXMiOiJEcmF",
            "mdCIsIm9yZ2FuaXphdGlvbl9pZCI6IjAxSkJIS1hIWVNLUFAyNDdIWlpXSEEzSkNUIiwidGltZXN0YW1",
            "wIjpudWxsfSwib3JnYW5pemF0aW9uIjp7ImlkIjoiMDFKQkhLWEhZU0tQUDI0N0haWldIQTNKQ1QiLCJ",
            "uYW1lIjoiZ2xvYmFsLW9yZ2FuaXphdGlvbiIsImNvZGUiOiJPUjAwMDAwMiIsImNhdGVnb3JpZXMiOls",
            "iVGVhbSJdLCJzdGF0dXMiOiJBY3RpdmUiLCJvcmdhbml6YXRpb25faWQiOiIwMUpCSEtYSFlTS1BQMjQ",
            "3SFpaV0hBM0pDVCIsInBhcmVudF9vcmdhbml6YXRpb25faWQiOm51bGx9LCJpZCI6IjAxSlQxUjFCMlh",
            "NRERCN1dZM0pDODREVjU1IiwiYWNjb3VudF9pZCI6InN5c3RlbV9kZXZpY2UiLCJvcmdhbml6YXRpb25",
            "faWQiOiIwMUpCSEtYSFlTS1BQMjQ3SFpaV0hBM0pDVCIsImFjY291bnRfb3JnYW5pemF0aW9uX2lkIjo",
            "iMDFKWTJDNDVYSjkwS1NTWVpYU01TRjIwMjgiLCJhY2NvdW50X3N0YXR1cyI6IkFjdGl2ZSIsInJvbGV",
            "faWQiOm51bGx9LCJpYXQiOjE3NTAyODc3NjIsImV4cCI6MTc1MDQ2MDU2Mn0.y6B0BNCuYQeiVsrqiCQ",
            "kkwdYCvwj5x2tqFEHRCSXkxY"
        );

        let token = Token::from_jwt(token);
        assert!(token.is_ok());

        let token = token.unwrap();
        assert!(token.account.device.is_some());
        assert!(token.account.contact.is_none());
    }

    #[test]
    fn test_device_issued_to_a_user() {
        let token = concat!(
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhY2NvdW50Ijp7InByb2ZpbGUiOnsiaWQiOiIwMUp",
            "ZMkM0NUFOWkMwNjJaQ0Y2QzU4MlJQRyIsImZpcnN0X25hbWUiOiJTdXBlciIsImxhc3RfbmFtZSI6IkF",
            "kbWluIiwiZW1haWwiOiJhZG1pbkBkbmFtaWNyby5jb20iLCJhY2NvdW50X2lkIjoiMDFKQ1NBRzc5S1E",
            "xV00wRjlCNDdRNzAwUDEiLCJjYXRlZ29yaWVzIjpbXSwiY29kZSI6bnVsbCwic3RhdHVzIjoiQWN0aXZ",
            "lIiwib3JnYW5pemF0aW9uX2lkIjoiMDFKWTJDNDU0UjZHTTM2R0gwQ1RDREcwQ0gifSwiY29udGFjdCI",
            "6eyJpZCI6IjAxSkNTQUc3OUtRMVdNMEY5QjQ3UTcwMFAxIiwiZmlyc3RfbmFtZSI6IlN1cGVyIiwibGF",
            "zdF9uYW1lIjoiQWRtaW4iLCJhY2NvdW50X2lkIjoiMDFKQ1NBRzc5S1ExV00wRjlCNDdRNzAwUDEiLCJ",
            "jb2RlIjoiQ08wMDAwMDEiLCJjYXRlZ29yaWVzIjpbIkNvbnRhY3QiXSwic3RhdHVzIjoiQWN0aXZlIiw",
            "ib3JnYW5pemF0aW9uX2lkIjoiMDFKQkhLWEhZU0tQUDI0N0haWldIQTNKQ1QiLCJkYXRlX29mX2JpcnR",
            "oIjpudWxsfSwiZGV2aWNlIjpudWxsLCJvcmdhbml6YXRpb24iOnsiaWQiOiIwMUpCSEtYSFlTS1BQMjQ",
            "3SFpaV0hBM0pDVCIsIm5hbWUiOiJnbG9iYWwtb3JnYW5pemF0aW9uIiwiY29kZSI6Ik9SMDAwMDAyIiw",
            "iY2F0ZWdvcmllcyI6WyJUZWFtIl0sInN0YXR1cyI6IkFjdGl2ZSIsIm9yZ2FuaXphdGlvbl9pZCI6IjA",
            "xSkJIS1hIWVNLUFAyNDdIWlpXSEEzSkNUIiwicGFyZW50X29yZ2FuaXphdGlvbl9pZCI6bnVsbH0sIml",
            "kIjoiMDFKQ1NBRzc5S1ExV00wRjlCNDdRNzAwUDEiLCJhY2NvdW50X2lkIjoiYWRtaW5AZG5hbWljcm8",
            "uY29tIiwib3JnYW5pemF0aW9uX2lkIjoiMDFKQkhLWEhZU0tQUDI0N0haWldIQTNKQ1QiLCJhY2NvdW5",
            "0X29yZ2FuaXphdGlvbl9pZCI6IjAxSlkyQzQ1SlRCMFEySDE5QjM0Q1FOOVZNIiwiYWNjb3VudF9zdGF",
            "0dXMiOiJBY3RpdmUiLCJyb2xlX2lkIjpudWxsfSwic2lnbmVkX2luX2FjY291bnQiOnsicHJvZmlsZSI",
            "6eyJpZCI6IjAxSlkyQzQ1QU5aQzA2MlpDRjZDNTgyUlBHIiwiZmlyc3RfbmFtZSI6IlN1cGVyIiwibGF",
            "zdF9uYW1lIjoiQWRtaW4iLCJlbWFpbCI6ImFkbWluQGRuYW1pY3JvLmNvbSIsImFjY291bnRfaWQiOiI",
            "wMUpDU0FHNzlLUTFXTTBGOUI0N1E3MDBQMSIsImNhdGVnb3JpZXMiOltdLCJjb2RlIjpudWxsLCJzdGF",
            "0dXMiOiJBY3RpdmUiLCJvcmdhbml6YXRpb25faWQiOiIwMUpZMkM0NTRSNkdNMzZHSDBDVENERzBDSCJ",
            "9LCJjb250YWN0Ijp7ImlkIjoiMDFKQ1NBRzc5S1ExV00wRjlCNDdRNzAwUDEiLCJmaXJzdF9uYW1lIjo",
            "iU3VwZXIiLCJsYXN0X25hbWUiOiJBZG1pbiIsImFjY291bnRfaWQiOiIwMUpDU0FHNzlLUTFXTTBGOUI",
            "0N1E3MDBQMSIsImNvZGUiOiJDTzAwMDAwMSIsImNhdGVnb3JpZXMiOlsiQ29udGFjdCJdLCJzdGF0dXM",
            "iOiJBY3RpdmUiLCJvcmdhbml6YXRpb25faWQiOiIwMUpCSEtYSFlTS1BQMjQ3SFpaV0hBM0pDVCIsImR",
            "hdGVfb2ZfYmlydGgiOm51bGx9LCJkZXZpY2UiOm51bGwsIm9yZ2FuaXphdGlvbiI6eyJpZCI6IjAxSkJ",
            "IS1hIWVNLUFAyNDdIWlpXSEEzSkNUIiwibmFtZSI6Imdsb2JhbC1vcmdhbml6YXRpb24iLCJjb2RlIjo",
            "iT1IwMDAwMDIiLCJjYXRlZ29yaWVzIjpbIlRlYW0iXSwic3RhdHVzIjoiQWN0aXZlIiwib3JnYW5pemF",
            "0aW9uX2lkIjoiMDFKQkhLWEhZU0tQUDI0N0haWldIQTNKQ1QiLCJwYXJlbnRfb3JnYW5pemF0aW9uX2l",
            "kIjpudWxsfSwiaWQiOiIwMUpDU0FHNzlLUTFXTTBGOUI0N1E3MDBQMSIsImFjY291bnRfaWQiOiJhZG1",
            "pbkBkbmFtaWNyby5jb20iLCJvcmdhbml6YXRpb25faWQiOiIwMUpCSEtYSFlTS1BQMjQ3SFpaV0hBM0p",
            "DVCIsImFjY291bnRfb3JnYW5pemF0aW9uX2lkIjoiMDFKWTJDNDVKVEIwUTJIMTlCMzRDUU45Vk0iLCJ",
            "hY2NvdW50X3N0YXR1cyI6IkFjdGl2ZSIsInJvbGVfaWQiOm51bGx9LCJpYXQiOjE3NTAyODY5MTIsImV",
            "4cCI6MTc1MDQ1OTcxMn0.vzG9BDQH_upGkzOavcIPdAfImN9E4KPH9La5Eo2jKIU"
        );

        let token = Token::from_jwt(token);
        assert!(token.is_ok());

        let token = token.unwrap();
        assert!(token.account.device.is_none());
        assert!(token.account.contact.is_some());
    }

    #[test]
    fn test_device_issued_to_root() {
        let token = concat!(
            "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJhY2NvdW50Ijp7InByb2ZpbGUiOnsiaWQiOiIwMUp",
            "NM0dUV0NIUjNDTTJOUDg1QzBRMktOMSIsImZpcnN0X25hbWUiOm51bGwsImxhc3RfbmFtZSI6bnVsbCw",
            "iZW1haWwiOiJyb290IiwiYWNjb3VudF9pZCI6IjAxSk0zR1RXQ0hSM0NNMk5QODVDMFEyS04xIiwiY2F",
            "0ZWdvcmllcyI6W10sImNvZGUiOm51bGwsInN0YXR1cyI6IkFjdGl2ZSIsIm9yZ2FuaXphdGlvbl9pZCI",
            "6IjAxSlNONFhBMkMzQTdSSE4zTU5aWkpHQlIzIn0sImNvbnRhY3QiOm51bGwsImRldmljZSI6bnVsbCw",
            "ib3JnYW5pemF0aW9uIjp7ImlkIjoiMDFKU040WEEyQzNBN1JITjNNTlpaSkdCUjMiLCJuYW1lIjoiUm9",
            "vdCBQZXJzb25hbCBPcmdhbml6YXRpb24iLCJjb2RlIjoiT1IwMDAwMDAiLCJjYXRlZ29yaWVzIjpbIlJ",
            "vb3QiLCJQZXJzb25hbCJdLCJzdGF0dXMiOiJBY3RpdmUiLCJvcmdhbml6YXRpb25faWQiOiIwMUpTTjR",
            "YQTJDM0E3UkhOM01OWlpKR0JSMyIsInBhcmVudF9vcmdhbml6YXRpb25faWQiOm51bGx9LCJpZCI6IjA",
            "xSk0zR1RXQ0hSM0NNMk5QODVDMFEyS04xIiwiYWNjb3VudF9pZCI6InJvb3QiLCJvcmdhbml6YXRpb25",
            "faWQiOiIwMUpTTjRYQTJDM0E3UkhOM01OWlpKR0JSMyIsImFjY291bnRfb3JnYW5pemF0aW9uX2lkIjo",
            "iMDFKTTNHVFdDSFIzQ00yTlA4NUMwUTJLTjEiLCJhY2NvdW50X3N0YXR1cyI6IkFjdGl2ZSIsInJvbGV",
            "faWQiOm51bGx9LCJzaWduZWRfaW5fYWNjb3VudCI6eyJwcm9maWxlIjp7ImlkIjoiMDFKTTNHVFdDSFI",
            "zQ00yTlA4NUMwUTJLTjEiLCJmaXJzdF9uYW1lIjpudWxsLCJsYXN0X25hbWUiOm51bGwsImVtYWlsIjo",
            "icm9vdCIsImFjY291bnRfaWQiOiIwMUpNM0dUV0NIUjNDTTJOUDg1QzBRMktOMSIsImNhdGVnb3JpZXM",
            "iOltdLCJjb2RlIjpudWxsLCJzdGF0dXMiOiJBY3RpdmUiLCJvcmdhbml6YXRpb25faWQiOiIwMUpTTjR",
            "YQTJDM0E3UkhOM01OWlpKR0JSMyJ9LCJjb250YWN0IjpudWxsLCJkZXZpY2UiOm51bGwsIm9yZ2FuaXp",
            "hdGlvbiI6eyJpZCI6IjAxSlNONFhBMkMzQTdSSE4zTU5aWkpHQlIzIiwibmFtZSI6IlJvb3QgUGVyc29",
            "uYWwgT3JnYW5pemF0aW9uIiwiY29kZSI6Ik9SMDAwMDAwIiwiY2F0ZWdvcmllcyI6WyJSb290IiwiUGV",
            "yc29uYWwiXSwic3RhdHVzIjoiQWN0aXZlIiwib3JnYW5pemF0aW9uX2lkIjoiMDFKU040WEEyQzNBN1J",
            "ITjNNTlpaSkdCUjMiLCJwYXJlbnRfb3JnYW5pemF0aW9uX2lkIjpudWxsfSwiaWQiOiIwMUpNM0dUV0N",
            "IUjNDTTJOUDg1QzBRMktOMSIsImFjY291bnRfaWQiOiJyb290Iiwib3JnYW5pemF0aW9uX2lkIjoiMDF",
            "KU040WEEyQzNBN1JITjNNTlpaSkdCUjMiLCJhY2NvdW50X29yZ2FuaXphdGlvbl9pZCI6IjAxSk0zR1R",
            "XQ0hSM0NNMk5QODVDMFEyS04xIiwiYWNjb3VudF9zdGF0dXMiOiJBY3RpdmUiLCJyb2xlX2lkIjpudWx",
            "sfSwiaWF0IjoxNzUwMjg3OTIwLCJleHAiOjE3NTA0NjA3MjB9.wHqECSTYAe6rDsZd3Pff66yklRmGw4",
            "olmJSYi502Q_M"
        );

        let token = Token::from_jwt(token);
        assert!(token.is_ok());

        let token = token.unwrap();
        assert!(token.account.device.is_none());
        assert!(token.account.contact.is_none());
    }
}
