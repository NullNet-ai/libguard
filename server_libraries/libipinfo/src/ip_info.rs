use maxminddb::geoip2::{Asn, City};

#[derive(Debug, PartialEq, Default)]
/// Struct to hold information about an IP address.
pub struct IpInfo {
    pub country: Option<String>,
    pub asn: Option<String>,
    pub org: Option<String>,
    pub continent_code: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal: Option<String>,
    pub timezone: Option<String>,
}

impl IpInfo {
    pub(crate) fn from_mmdb(city: Option<&City>, asn: Option<&Asn>) -> Self {
        Self {
            country: city
                .as_ref()
                .and_then(|city| city.country.iso_code)
                .map(std::string::ToString::to_string),
            asn: asn.as_ref().and_then(|asn| {
                asn.autonomous_system_number
                    .map(|asn_number| asn_number.to_string())
            }),
            org: asn.as_ref().and_then(|asn| {
                asn.autonomous_system_organization
                    .map(std::string::ToString::to_string)
            }),
            continent_code: city
                .as_ref()
                .and_then(|city| city.continent.code)
                .map(std::string::ToString::to_string),
            city: city
                .as_ref()
                .and_then(|city| city.city.names.english)
                .map(std::string::ToString::to_string),
            region: city
                .as_ref()
                .and_then(|city| city.subdivisions.first())
                .and_then(|subdivision| subdivision.names.english)
                .map(std::string::ToString::to_string),
            postal: city
                .as_ref()
                .and_then(|city| city.postal.code)
                .map(std::string::ToString::to_string),
            timezone: city
                .as_ref()
                .and_then(|city| city.location.time_zone)
                .map(std::string::ToString::to_string),
        }
    }
}
