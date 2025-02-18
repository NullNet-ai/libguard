use nullnet_liberror::{location, Error, ErrorHandler, Location};
use reqwest::{Client, ClientBuilder};

pub(crate) fn new_web_client() -> Result<Client, Error> {
    ClientBuilder::new()
        .user_agent("Nullnet")
        // .timeout(Duration::from_secs(300))
        .build()
        .handle_err(location!())
}
