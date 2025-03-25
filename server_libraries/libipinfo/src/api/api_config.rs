use crate::IpInfo;
use crate::api::api_fields::ApiFields;
use nullnet_liberror::{Error, ErrorHandler, Location, location};
use reqwest::Client;
use serde_json::Value;

pub(crate) struct ApiConfig {
    url: String,
    fields: ApiFields,
}

impl ApiConfig {
    pub(crate) fn new(url: &str, api_key: &str, fields: ApiFields) -> Self {
        let url = url.replace("{api_key}", api_key);
        Self { url, fields }
    }

    fn get_url(&self, ip: &str) -> String {
        self.url.replace("{ip}", ip)
    }

    fn get_field_names(&self) -> &ApiFields {
        &self.fields
    }

    pub(crate) async fn lookup_ip(&self, client: &Client, ip: &str) -> Result<IpInfo, Error> {
        let json: Value = client
            .get(self.get_url(ip))
            .send()
            .await
            .handle_err(location!())?
            .json()
            .await
            .handle_err(location!())?;

        let names = self.get_field_names();

        Ok(IpInfo {
            country: names.extract_country(&json),
            asn: names.extract_asn(&json),
            org: names.extract_org(&json),
            continent_code: names.extract_continent_code(&json),
            city: names.extract_city(&json),
            region: names.extract_region(&json),
            postal: names.extract_postal(&json),
            timezone: names.extract_timezone(&json),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::IpInfo;
    use crate::api::api_config::ApiConfig;
    use crate::api::api_fields::ApiFields;
    use crate::web_client::new_web_client;

    #[tokio::test]
    async fn test_lookup_from_api() {
        let api_provider = ApiConfig::new(
            "https://ipapi.co/{ip}/json",
            "",
            ApiFields {
                country: Some("/country"),
                asn: Some("/asn"),
                org: Some("/org"),
                continent_code: Some("/continent_code"),
                city: Some("/city"),
                region: Some("/region"),
                postal: Some("/postal"),
                timezone: Some("/timezone"),
            },
        );

        let ip_info = api_provider
            .lookup_ip(&new_web_client().unwrap(), "8.8.8.8")
            .await
            .unwrap();
        assert_eq!(
            ip_info,
            IpInfo {
                country: Some("US".to_string()),
                asn: Some("AS15169".to_string()),
                org: Some("GOOGLE".to_string()),
                continent_code: Some("NA".to_string()),
                city: Some("Mountain View".to_string()),
                region: Some("California".to_string()),
                postal: Some("94043".to_string()),
                timezone: Some("America/Los_Angeles".to_string())
            }
        );
    }
}
