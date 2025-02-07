use reqwest::{Client, ClientBuilder};

pub(crate) fn new_web_client() -> Client {
    ClientBuilder::new()
        .user_agent("Nullnet")
        // .timeout(Duration::from_secs(300))
        .build()
        // .handle_err(location!())
        .unwrap_or_default()
}
