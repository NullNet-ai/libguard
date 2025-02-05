pub struct ApiConfig {
    url: String,
    fields: ApiFields,
}

impl ApiConfig {
    pub fn new(url: String, api_key: String, fields: ApiFields) -> Self {
        let url = url.replace("{api_key}", &api_key);
        Self { url, fields }
    }

    pub(crate) fn get_url(&self, ip: &str) -> String {
        self.url
            .replace("{ip}", ip)
    }

    pub(crate) fn get_field_names(&self) -> &ApiFields {
        &self.fields
    }
}

pub struct ApiFields {
    pub country: Option<&'static str>,
    pub asn: Option<&'static str>,
    pub org: Option<&'static str>,
    pub continent_code: Option<&'static str>,
    pub city: Option<&'static str>,
    pub region: Option<&'static str>,
    pub postal: Option<&'static str>,
    pub timezone: Option<&'static str>,
}
