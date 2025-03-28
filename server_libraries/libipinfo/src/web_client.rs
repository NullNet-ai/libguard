use nullnet_liberror::{Error, ErrorHandler, Location, location};
use reqwest::{Client, ClientBuilder};

pub(crate) fn new_web_client() -> Result<Client, Error> {
    ClientBuilder::new()
        .user_agent("Nullnet")
        // .timeout(Duration::from_secs(300))
        .build()
        .handle_err(location!())
}
