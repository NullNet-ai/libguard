use crate::api::api_config::ApiConfig;
use crate::handle_data::{periodically_refresh_data};
pub use crate::ip_info::IpInfo;
use crate::mmdb_reader::MmdbReader;
use reqwest::{Client, ClientBuilder};
use std::sync::{Arc, RwLock};
use std::time::Duration;

mod constants;
mod handle_data;
mod ip_info;
mod mmdb_reader;
mod api;

pub struct IpInfoHandler {
    web_client: Client,
    api_config: ApiConfig,
    mmdb_reader: Arc<RwLock<MmdbReader>>,
}

impl IpInfoHandler {
    pub fn new(api_config: ApiConfig, mmdb_key: String) -> Self {
        let web_client = ClientBuilder::new()
            .user_agent("nullnet")
            .timeout(Duration::from_secs(300))
            .build()
            // .handle_err(location!())
            .unwrap_or_default();
        let mmdb_reader = Arc::new(RwLock::new(MmdbReader::default()));
        let mmdb_reader_2 = mmdb_reader.clone();

        // log::info!("Opened blacklist SQLite database at {BLACKLIST_PATH}");

        tokio::spawn(async move {
            periodically_refresh_data(mmdb_reader_2, &mmdb_key).await;
        });

        Self {
            api_config,
            web_client,
            mmdb_reader,
        }
    }

    pub async fn lookup(&self, ip: &str) -> Result<IpInfo, ()> {
        let ip_info =
            IpInfo::lookup_from_api(&self.api_config, &self.web_client, ip)
                .await
                .or_else(|_| {
                    // log::warn!("Failed to look up IP info from API, trying with MMDB...");
                    IpInfo::lookup_from_mmdb(&self.mmdb_reader, ip)
                });

        ip_info
    }
}
