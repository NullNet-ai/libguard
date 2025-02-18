#![doc = include_str!("../README.md")]

pub use crate::api::api_fields::ApiFields;
pub use crate::ip_info::IpInfo;
pub use crate::ip_info_provider::IpInfoProvider;
use crate::mmdb::mmdb_config::MmdbConfig;
use crate::web_client::new_web_client;
use nullnet_liberror::Error;
use reqwest::Client;

mod api;
mod ip_info;
mod ip_info_provider;
mod mmdb;
mod web_client;

/// The main struct for handling IP information lookups.
pub struct IpInfoHandler {
    web_client: Client,
    providers: Vec<IpInfoProvider>,
    fallback: MmdbConfig,
}

impl IpInfoHandler {
    /// Returns a new `IpInfoHandler` with the given providers.
    ///
    /// Even if no providers are given, the handler will still use a fallback provider
    /// (free databases from [dp-ip.com](https://db-ip.com)).
    #[allow(clippy::missing_errors_doc)]
    pub fn new(providers: Vec<IpInfoProvider>) -> Result<Self, Error> {
        let web_client = new_web_client()?;

        let fallback = MmdbConfig::new(
            "https://download.db-ip.com/free/dbip-city-lite-{%Y-%m}.mmdb.gz",
            "https://download.db-ip.com/free/dbip-asn-lite-{%Y-%m}.mmdb.gz",
            "",
            31,
        );

        Ok(Self {
            web_client,
            providers,
            fallback,
        })
    }

    /// Looks up the IP information for the given IP address.
    #[allow(clippy::missing_errors_doc)]
    pub async fn lookup(&self, ip: &str) -> Result<IpInfo, Error> {
        for provider in &self.providers {
            let ip_info = provider.lookup_ip(&self.web_client, ip).await;
            if ip_info.is_ok() {
                return ip_info;
            }
        }
        self.fallback.lookup_ip(ip)
    }
}
