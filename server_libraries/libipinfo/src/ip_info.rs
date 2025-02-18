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
                .and_then(|city| city.country.as_ref().and_then(|country| country.iso_code))
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
                .and_then(|city| city.continent.as_ref().and_then(|continent| continent.code))
                .map(std::string::ToString::to_string),
            city: city
                .as_ref()
                .and_then(|city| {
                    city.city
                        .as_ref()
                        .and_then(|city| city.names.as_ref().and_then(|names| names.get("en")))
                })
                .map(std::string::ToString::to_string),
            region: city
                .as_ref()
                .and_then(|city| {
                    city.subdivisions
                        .as_ref()
                        .and_then(|subdivisions| subdivisions.first())
                })
                .and_then(|subdivision| {
                    subdivision.names.as_ref().and_then(|names| names.get("en"))
                })
                .map(std::string::ToString::to_string),
            postal: city
                .as_ref()
                .and_then(|city| city.postal.as_ref().and_then(|postal| postal.code))
                .map(std::string::ToString::to_string),
            timezone: city
                .as_ref()
                .and_then(|city| {
                    city.location
                        .as_ref()
                        .and_then(|location| location.time_zone)
                })
                .map(std::string::ToString::to_string),
        }
    }
}
