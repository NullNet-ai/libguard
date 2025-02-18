use crate::api::api_config::ApiConfig;
use crate::api::api_fields::ApiFields;
use crate::mmdb::mmdb_config::MmdbConfig;
use crate::IpInfo;
use nullnet_liberror::Error;
use reqwest::Client;

/// An IP information provider.
pub struct IpInfoProvider {
    inner: IpInfoProviderInner,
}

impl IpInfoProvider {
    /// Returns a new API-based `IpInfoProvider`.
    ///
    /// # Arguments
    /// * `url` - The URL of the API endpoint.
    ///   The string "`{ip}`" will be replaced with the IP address, and "`{api_key}`" with the API key, if any.
    /// * `api_key` - The API key to use.
    /// * `fields` - The fields to request from the API.
    #[must_use]
    pub fn new_api_provider(url: &str, api_key: &str, fields: ApiFields) -> Self {
        Self {
            inner: IpInfoProviderInner::Api(ApiConfig::new(url, api_key, fields)),
        }
    }

    /// Returns a new MMDB-based `IpInfoProvider`.
    ///
    /// # Arguments
    ///
    /// * `location_url` - The URL of the location MMDB file.
    ///   The string "`{api_key}`" will be replaced with the API key, if any.
    /// * `mmdb_url` - The URL of the ASN MMDB file.
    ///   The string "`{api_key}`" will be replaced with the API key, if any.
    /// * `api_key` - The API key to use.
    /// * `refresh_days` - The number of days to wait before refreshing the MMDB files.
    #[must_use]
    pub fn new_mmdb_provider(
        location_url: &str,
        mmdb_url: &str,
        api_key: &str,
        refresh_days: u64,
    ) -> Self {
        Self {
            inner: IpInfoProviderInner::Mmdb(MmdbConfig::new(
                location_url,
                mmdb_url,
                api_key,
                refresh_days,
            )),
        }
    }

    pub(crate) async fn lookup_ip(&self, client: &Client, ip: &str) -> Result<IpInfo, Error> {
        match &self.inner {
            IpInfoProviderInner::Api(config) => config.lookup_ip(client, ip).await,
            IpInfoProviderInner::Mmdb(config) => config.lookup_ip(ip),
        }
    }
}

enum IpInfoProviderInner {
    Api(ApiConfig),
    Mmdb(MmdbConfig),
}
