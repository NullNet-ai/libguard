use crate::api::api_config::ApiConfig;
use crate::api::api_fields::ApiFields;
use crate::mmdb::mmdb_config::MmdbConfig;
use crate::IpInfo;
use reqwest::Client;

pub struct IpInfoProvider {
    inner: IpInfoProviderInner,
}

impl IpInfoProvider {
    pub fn new_api_provider(url: &str, api_key: &str, fields: ApiFields) -> Self {
        Self {
            inner: IpInfoProviderInner::Api(ApiConfig::new(url, api_key, fields)),
        }
    }

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

    pub(crate) async fn lookup_ip(&self, client: &Client, ip: &str) -> Result<IpInfo, ()> {
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
