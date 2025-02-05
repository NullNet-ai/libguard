use crate::api::api_config::ApiConfig;
use crate::mmdb_reader::MmdbReader;
use maxminddb::MaxMindDBError;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;
use std::sync::{Arc, RwLock};

#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct IpInfo {
    pub country: Option<String>,
    pub asn: Option<String>,
    #[serde(alias = "as_name")]
    pub org: Option<String>,
    #[serde(alias = "continent")]
    pub continent_code: Option<String>,
    #[serde(default)]
    pub city: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    pub postal: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
}

impl IpInfo {
    pub(crate) async fn lookup_from_api(
        config: &ApiConfig,
        client: &Client,
        ip: &str,
    ) -> Result<IpInfo, ()>
// Result<IpInfo, Error>
    {
        let json: Value = client
            .get(config.get_url(ip))
            .send()
            .await
            .unwrap()
            // .handle_err(location!())?
            .json()
            .await
            .map_err(|_| ())?;
        // .handle_err(location!())

        println!("{:?}", json);

        let names = config.get_field_names();

        Ok(IpInfo {
            country: extract_json_field(&json, names.country),
            asn: extract_json_field(&json, names.asn),
            org: extract_json_field(&json, names.org),
            continent_code: extract_json_field(&json, names.continent_code),
            city: extract_json_field(&json, names.city),
            region: extract_json_field(&json, names.region),
            postal: extract_json_field(&json, names.postal),
            timezone: extract_json_field(&json, names.timezone),
        })
    }

    pub(crate) fn lookup_from_mmdb(
        mmdb_reader: &Arc<RwLock<MmdbReader>>,
        ip: &str,
    ) -> Result<IpInfo, ()>
// Result<IpInfo, Error>
    {
        let ip_info_res = mmdb_reader
            .read()
            .unwrap()
            // .handle_err(location!())?
            .lookup::<IpInfo>(
                ip.parse().unwrap(), // .handle_err(location!())?
            );

        if let Err(MaxMindDBError::AddressNotFoundError(_)) = ip_info_res {
            return Ok(IpInfo::default());
        }

        ip_info_res.map_err(|_| ())
        // .handle_err(location!())
    }
}

fn extract_json_field(json: &Value, field: Option<&str>) -> Option<String> {
    if let Some(name) = field {
        if let Some(value) = json.pointer(name).map(|v| v.as_str()) {
            return value.map(|v| v.to_string());
        }
    }
    None
}
