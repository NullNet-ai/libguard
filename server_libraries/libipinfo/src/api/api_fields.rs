use serde_json::Value;

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

impl ApiFields {
    pub(crate) fn extract_country(&self, json: &Value) -> Option<String> {
        extract_json_field(json, self.country)
    }

    pub(crate) fn extract_asn(&self, json: &Value) -> Option<String> {
        extract_json_field(json, self.asn)
    }

    pub(crate) fn extract_org(&self, json: &Value) -> Option<String> {
        extract_json_field(json, self.org)
    }

    pub(crate) fn extract_continent_code(&self, json: &Value) -> Option<String> {
        extract_json_field(json, self.continent_code)
    }

    pub(crate) fn extract_city(&self, json: &Value) -> Option<String> {
        extract_json_field(json, self.city)
    }

    pub(crate) fn extract_region(&self, json: &Value) -> Option<String> {
        extract_json_field(json, self.region)
    }

    pub(crate) fn extract_postal(&self, json: &Value) -> Option<String> {
        extract_json_field(json, self.postal)
    }

    pub(crate) fn extract_timezone(&self, json: &Value) -> Option<String> {
        extract_json_field(json, self.timezone)
    }
}

fn extract_json_field(json: &Value, field: Option<&str>) -> Option<String> {
    if let Some(name) = field {
        if let Some(value) = json.pointer(name).map(|v| v.as_str()) {
            return value.map(std::string::ToString::to_string);
        }
    }
    None
}
