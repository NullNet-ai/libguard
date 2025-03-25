use super::serde_ext::*;
use bincode::{Error, deserialize, serialize};
use get_if_addrs::{IfAddr, get_if_addrs};
use pnet::datalink;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;

/// Represents a snapshot of a network interface
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct InterfaceSnapshot {
    pub name: String,
    pub is_up: bool,
    pub is_loopback: bool,
    pub is_multicast: bool,
    pub is_broadcast: bool,
    pub mac_address: Option<String>,
    pub interface_index: Option<u32>,
    #[serde(with = "serde_ipaddr_vec")]
    pub ip_addresses: Vec<IpAddr>,
    #[serde(with = "serde_ipaddr_option")]
    pub subnet_mask: Option<IpAddr>,
    #[serde(with = "serde_ipaddr_option")]
    pub gateway: Option<IpAddr>,
}

impl InterfaceSnapshot {
    /// Serializes a vector of `InterfaceSnapshot` objects into a binary format (`Vec<u8>`).
    ///
    /// # Arguments
    /// - `snapshot`: A reference to a vector of `InterfaceSnapshot` instances.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: Serialized binary data.
    /// - `Err(Error)`: If serialization fails.
    pub fn serialize_snapshot(snapshot: &Vec<InterfaceSnapshot>) -> Result<Vec<u8>, Error> {
        serialize(snapshot)
    }

    /// Deserializes binary data (`Vec<u8>`) back into a vector of `InterfaceSnapshot` objects.
    ///
    /// # Arguments
    /// - `data`: A byte slice containing serialized `InterfaceSnapshot` data.
    ///
    /// # Returns
    /// - `Ok(Vec<InterfaceSnapshot>)`: The deserialized vector of network interface snapshots.
    /// - `Err(Error)`: If deserialization fails.
    pub fn deserialize_snapshot(data: &[u8]) -> Result<Vec<InterfaceSnapshot>, Error> {
        deserialize(data)
    }

    /// Captures the current state of all network interfaces available on the system.
    ///
    /// - Retrieves interface names, statuses, MAC addresses, and other properties using `pnet::datalink`.
    /// - Fetches assigned IP addresses and subnet masks using `get_if_addrs`.
    ///
    /// # Returns
    /// - A `Vec<InterfaceSnapshot>` containing details of all detected network interfaces.
    pub fn take_all() -> Vec<InterfaceSnapshot> {
        let interfaces = datalink::interfaces();
        let mut iface_map: HashMap<String, InterfaceSnapshot> = HashMap::new();

        for iface in interfaces {
            iface_map.insert(
                iface.name.clone(),
                InterfaceSnapshot {
                    name: iface.name.clone(),
                    is_up: iface.is_up(),
                    is_loopback: iface.is_loopback(),
                    is_multicast: iface.is_multicast(),
                    is_broadcast: iface.is_broadcast(),
                    mac_address: iface.mac.as_ref().map(|mac| mac.to_string()),
                    interface_index: Some(iface.index),
                    ip_addresses: Vec::new(),
                    subnet_mask: None,
                    gateway: None,
                },
            );
        }

        if let Ok(if_addrs) = get_if_addrs() {
            for iface in if_addrs {
                if let Some(entry) = iface_map.get_mut(&iface.name) {
                    match iface.addr {
                        IfAddr::V4(ipv4) => {
                            entry.ip_addresses.push(IpAddr::V4(ipv4.ip));
                            entry.subnet_mask = Some(IpAddr::V4(ipv4.netmask));
                        }
                        IfAddr::V6(ipv6) => {
                            entry.ip_addresses.push(IpAddr::V6(ipv6.ip));
                        }
                    }
                }
            }
        }

        iface_map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::InterfaceSnapshot;
    use bincode;

    #[test]
    fn test_interface_snapshot_serialize_deserialize() {
        let snapshot = vec![InterfaceSnapshot {
            name: "eth0".to_string(),
            is_up: true,
            is_loopback: false,
            is_multicast: true,
            is_broadcast: true,
            mac_address: Some("00:1A:2B:3C:4D:5E".to_string()),
            interface_index: Some(1),
            ip_addresses: vec![
                "192.168.1.100".parse().unwrap(),
                "10.0.0.1".parse().unwrap(),
            ],
            subnet_mask: Some("255.255.255.0".parse().unwrap()),
            gateway: Some("192.168.1.1".parse().unwrap()),
        }];

        let serialized = InterfaceSnapshot::serialize_snapshot(&snapshot).unwrap();
        assert!(!serialized.is_empty(), "Serialization should produce data");

        let deserialized = InterfaceSnapshot::deserialize_snapshot(&serialized).unwrap();
        assert_eq!(
            deserialized, snapshot,
            "Deserialized snapshot should match original"
        );
    }

    #[test]
    fn test_empty_interface_snapshot_serialize_deserialize() {
        let snapshot: Vec<InterfaceSnapshot> = vec![];

        let serialized = InterfaceSnapshot::serialize_snapshot(&snapshot).unwrap();
        assert!(!serialized.is_empty(), "Serialization should produce data");

        let deserialized = InterfaceSnapshot::deserialize_snapshot(&serialized).unwrap();
        assert!(
            deserialized.is_empty(),
            "Deserialized snapshot should be empty"
        );
    }

    #[test]
    fn test_take_all_interfaces() {
        let snapshot = InterfaceSnapshot::take_all();
        assert!(
            !snapshot.is_empty(),
            "At least one network interface should be detected"
        );
    }

    #[test]
    fn test_serde_ipaddr_option_fields() {
        let snapshot = InterfaceSnapshot {
            name: "eth0".to_string(),
            is_up: true,
            is_loopback: false,
            is_multicast: true,
            is_broadcast: true,
            mac_address: Some("00:1A:2B:3C:4D:5E".to_string()),
            interface_index: Some(1),
            ip_addresses: vec!["192.168.1.100".parse().unwrap()],
            subnet_mask: None,
            gateway: Some("192.168.1.1".parse().unwrap()),
        };

        let serialized = bincode::serialize(&snapshot).unwrap();
        let deserialized: InterfaceSnapshot = bincode::deserialize(&serialized).unwrap();

        assert_eq!(
            deserialized.subnet_mask, None,
            "Subnet mask should remain `None` after deserialization"
        );
        assert_eq!(
            deserialized.gateway,
            Some("192.168.1.1".parse().unwrap()),
            "Gateway should match after deserialization"
        );
    }

    /// Test serialization and deserialization of multiple IP addresses.
    #[test]
    fn test_serde_ipaddr_vec_fields() {
        let snapshot = InterfaceSnapshot {
            name: "eth0".to_string(),
            is_up: true,
            is_loopback: false,
            is_multicast: true,
            is_broadcast: true,
            mac_address: Some("00:1A:2B:3C:4D:5E".to_string()),
            interface_index: Some(1),
            ip_addresses: vec![
                "192.168.1.100".parse().unwrap(),
                "10.0.0.1".parse().unwrap(),
            ],
            subnet_mask: Some("255.255.255.0".parse().unwrap()),
            gateway: None,
        };

        let serialized = bincode::serialize(&snapshot).unwrap();
        let deserialized: InterfaceSnapshot = bincode::deserialize(&serialized).unwrap();

        assert_eq!(
            deserialized.ip_addresses.len(),
            2,
            "There should be exactly 2 IP addresses"
        );
        assert_eq!(
            deserialized.ip_addresses, snapshot.ip_addresses,
            "IP addresses should match after deserialization"
        );
    }
}
