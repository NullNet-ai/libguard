pub use crate::ip_info::IpInfo;
use crate::ip_info_provider::IpInfoProvider;
use crate::mmdb::mmdb_config::MmdbConfig;
use crate::web_client::new_web_client;
use reqwest::Client;

mod api;
mod ip_info;
mod ip_info_provider;
mod mmdb;
mod web_client;

pub struct IpInfoHandler {
    web_client: Client,
    providers: Vec<IpInfoProvider>,
    fallback: MmdbConfig,
}

impl IpInfoHandler {
    pub fn new(providers: Vec<IpInfoProvider>) -> Self {
        let web_client = new_web_client();

        let year_month = chrono::Utc::now().format("%Y-%m").to_string();
        let fallback = MmdbConfig::new(
            &format!("https://download.db-ip.com/free/dbip-city-lite-{year_month}.mmdb.gz"),
            &format!("https://download.db-ip.com/free/dbip-asn-lite-{year_month}.mmdb.gz"),
            "",
            31,
        );

        Self {
            web_client,
            providers,
            fallback,
        }
    }

    pub async fn lookup(&self, ip: &str) -> Result<IpInfo, ()> {
        for provider in &self.providers {
            let ip_info = provider.lookup_ip(&self.web_client, ip).await;
            if ip_info.is_ok() {
                return ip_info;
            }
        }

        self.fallback.lookup_ip(ip)?;

        Err(())
    }
}
