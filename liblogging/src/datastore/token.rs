use nullnet_libtoken::Token;
use std::time::{SystemTime, UNIX_EPOCH};

const EXPIRATION_MARGIN: u64 = 60 * 5;

#[derive(Debug)]
pub struct TokenWrapper {
    pub jwt: String,
    pub info: Token,
}

impl TokenWrapper {
    pub fn from_jwt(jwt: String) -> Result<Self, String> {
        let info = Token::from_jwt(&jwt)?;
        Ok(Self { jwt, info })
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.info.exp <= (now - EXPIRATION_MARGIN)
    }
}
