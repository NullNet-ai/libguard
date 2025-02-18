/// Challenges:
/// 1) different providers have different database formats
/// 2) certain providers split IPv4 and IPv6 data into separate databases, others combine them
/// 3) certain providers split ASN and geolocation data into separate databases, others combine them
/// 4) certain providers have static download links, others vary by date
/// 5) certain providers compress their databases, others do not
///
/// What we currently support:
/// 1) MMDBs compatible with `MaxMind`'s specification version 2.0
/// 2) databases that combine IPv4 and IPv6 data
/// 3) databases that split ASN and geolocation data
/// 4) databases that have static download links
/// 5) databases that are gzip compressed
use crate::mmdb::mmdb_reader::MmdbReader;
use crate::mmdb::refresh_mmdb_data::refresh_mmdb_data;
use crate::IpInfo;
use maxminddb::geoip2::{Asn, City};
use nullnet_liberror::{location, Error, ErrorHandler, Location};
use std::sync::{Arc, RwLock};

pub(crate) struct MmdbConfig {
    location_reader: Arc<RwLock<MmdbReader>>,
    asn_reader: Arc<RwLock<MmdbReader>>,
}

impl MmdbConfig {
    pub(crate) fn new(location_url: &str, asn_url: &str, api_key: &str, refresh_days: u64) -> Self {
        let location_url = location_url.replace("{api_key}", api_key);
        let mmdb_url = asn_url.replace("{api_key}", api_key);

        let location_reader = Arc::new(RwLock::new(MmdbReader::default()));
        let location_reader_2 = location_reader.clone();
        tokio::spawn(async move {
            refresh_mmdb_data(location_reader_2, &location_url, refresh_days).await;
        });

        let asn_reader = Arc::new(RwLock::new(MmdbReader::default()));
        let asn_reader_2 = asn_reader.clone();
        tokio::spawn(async move {
            refresh_mmdb_data(asn_reader_2, &mmdb_url, refresh_days).await;
        });

        Self {
            location_reader,
            asn_reader,
        }
    }

    pub(crate) fn lookup_ip(&self, ip: &str) -> Result<IpInfo, Error> {
        let ip = ip.parse().handle_err(location!())?;

        let location_reader = self.location_reader.read().handle_err(location!())?;
        let location = location_reader.lookup::<City>(ip);

        let asn_reader = self.asn_reader.read().handle_err(location!())?;
        let asn = asn_reader.lookup::<Asn>(ip);

        Ok(IpInfo::from_mmdb(location.as_ref(), asn.as_ref()))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::mmdb::mmdb_config::MmdbConfig;
//     use crate::IpInfo;
//     use tokio::time::sleep;
//
//     #[tokio::test]
//     async fn test_lookup_from_mmdb() {
//         let mmdb_provider = MmdbConfig::new(
//             "https://download.db-ip.com/free/dbip-city-lite-{%Y-%m}.mmdb.gz",
//             "https://download.db-ip.com/free/dbip-asn-lite-{%Y-%m}.mmdb.gz",
//             "",
//             31,
//         );
//
//         sleep(std::time::Duration::from_secs(100)).await;
//         let ip_info = mmdb_provider.lookup_ip("8.8.8.8").unwrap();
//         assert_eq!(
//             ip_info,
//             IpInfo {
//                 country: Some("US".to_string()),
//                 asn: Some("15169".to_string()),
//                 org: Some("Google LLC".to_string()),
//                 continent_code: Some("NA".to_string()),
//                 city: Some("Mountain View".to_string()),
//                 region: Some("California".to_string()),
//                 postal: None,
//                 timezone: None
//             }
//         );
//     }
// }
