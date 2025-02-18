/// Serialization module for `Option<IpAddr>`.
/// Converts `Some(IpAddr)` into a `(u8, String)` tuple where:
/// - `1` indicates `Some`, followed by the IP address string.
/// - `0` indicates `None`, followed by an empty string.
pub mod serde_ipaddr_option {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::net::IpAddr;

    /// Serializes an `Option<IpAddr>` into a `(u8, String)` tuple.
    ///
    /// - `Some(IpAddr)` is stored as `(1, "192.168.1.1")`
    /// - `None` is stored as `(0, "")`
    pub fn serialize<S>(ip: &Option<IpAddr>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(ip) = ip {
            (1u8, ip.to_string()).serialize(serializer)
        } else {
            (0u8, "".to_string()).serialize(serializer)
        }
    }

    /// Deserializes an `Option<IpAddr>` from a `(u8, String)` tuple.
    ///
    /// - `(1, "192.168.1.1")` is deserialized to `Some(IpAddr)`.
    /// - `(0, "")` is deserialized to `None`.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<IpAddr>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (flag, ip_str): (u8, String) = Deserialize::deserialize(deserializer)?;
        match flag {
            1 => ip_str.parse().map(Some).map_err(serde::de::Error::custom),
            _ => Ok(None),
        }
    }
}

/// Serialization module for `Vec<IpAddr>`.
/// Converts `Vec<IpAddr>` into a single comma-separated string representation.
pub mod serde_ipaddr_vec {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::net::IpAddr;

    /// Serializes `Vec<IpAddr>` as a comma-separated string.
    ///
    /// - `vec!["192.168.1.1".parse().unwrap(), "10.0.0.1".parse().unwrap()]`
    ///   becomes `"192.168.1.1,10.0.0.1"`.
    #[allow(clippy::ptr_arg)]
    pub fn serialize<S>(ips: &Vec<IpAddr>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ip_strings: Vec<String> = ips.iter().map(|ip| ip.to_string()).collect();
        serializer.serialize_str(&ip_strings.join(","))
    }

    /// Deserializes `Vec<IpAddr>` from a comma-separated string.
    ///
    /// - `"192.168.1.1,10.0.0.1"` is deserialized into
    ///   `vec![IpAddr::V4("192.168.1.1".parse().unwrap()), IpAddr::V4("10.0.0.1".parse().unwrap())]`.
    ///
    /// - Empty string (`""`) results in an empty vector.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<IpAddr>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ip_string = String::deserialize(deserializer)?;
        let ips: Vec<IpAddr> = ip_string
            .split(',')
            .filter_map(|s| s.parse().ok())
            .collect();
        Ok(ips)
    }
}

#[cfg(test)]
mod tests {
    use super::{serde_ipaddr_option, serde_ipaddr_vec};
    use bincode;
    use serde::{Deserialize, Serialize};
    use std::net::IpAddr;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestOptionIpAddr {
        #[serde(with = "serde_ipaddr_option")]
        ip: Option<IpAddr>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestVecIpAddr {
        #[serde(with = "serde_ipaddr_vec")]
        ips: Vec<IpAddr>,
    }

    #[test]
    fn test_serde_ipaddr_option_serialize() {
        let data = TestOptionIpAddr {
            ip: Some("192.168.1.1".parse().unwrap()),
        };

        let serialized = bincode::serialize(&data).unwrap();
        let deserialized: TestOptionIpAddr = bincode::deserialize(&serialized).unwrap();

        assert_eq!(deserialized, data);
    }

    #[test]
    fn test_serde_ipaddr_option_serialize_none() {
        let data = TestOptionIpAddr { ip: None };
        let serialized = bincode::serialize(&data).unwrap();
        let deserialized: TestOptionIpAddr = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized, data);
    }

    #[test]
    fn test_serde_ipaddr_vec_serialize() {
        let data = TestVecIpAddr {
            ips: vec!["192.168.1.1".parse().unwrap(), "10.0.0.1".parse().unwrap()],
        };
        let serialized = bincode::serialize(&data).unwrap();
        let deserialized: TestVecIpAddr = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized, data);
    }

    #[test]
    fn test_serde_ipaddr_vec_serialize_empty() {
        let data = TestVecIpAddr { ips: vec![] };
        let serialized = bincode::serialize(&data).unwrap();
        let deserialized: TestVecIpAddr = bincode::deserialize(&serialized).unwrap();
        assert_eq!(deserialized, data);
    }
}
