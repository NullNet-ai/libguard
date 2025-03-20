mod ip_collection;

use crate::bogon::ip_collection::IpCollection;
use once_cell::sync::Lazy;
use std::net::IpAddr;

pub(crate) fn is_bogon(ip: IpAddr) -> Option<&'static str> {
    for bogon in BOGONS.iter() {
        if bogon.range.contains(&ip) {
            return Some(bogon.description);
        }
    }
    None
}

struct Bogon {
    range: IpCollection,
    description: &'static str,
}

// all bogons

static BOGONS: Lazy<Vec<&'static Bogon>> = Lazy::new(|| {
    vec![
        &THIS_NETWORK,
        &PRIVATE_USE,
        &CARRIER_GRADE,
        &LOOPBACK,
        &LINK_LOCAL,
        &IETF_PROTOCOL,
        &TEST_NET_1,
        &NETWORK_INTERCONNECT,
        &TEST_NET_2,
        &TEST_NET_3,
        &MULTICAST,
        &FUTURE_USE,
        &NODE_SCOPE_UNSPECIFIED,
        &NODE_SCOPE_LOOPBACK,
        &IPV4_MAPPED,
        &IPV4_COMPATIBLE,
        &REMOTELY_TRIGGERED,
        &ORCHID,
        &DOCUMENTATION_PREFIX,
        &ULA,
        &LINK_LOCAL_UNICAST,
        &SITE_LOCAL_UNICAST,
    ]
});

// IPv4 bogons

static THIS_NETWORK: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("0.0.0.0-0.255.255.255").unwrap(),
    description: "\"this\" network",
});

static PRIVATE_USE: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new(
        "10.0.0.0-10.255.255.255, 172.16.0.0-172.31.255.255, 192.168.0.0-192.168.255.255",
    )
    .unwrap(),
    description: "private-use",
});

static CARRIER_GRADE: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("100.64.0.0-100.127.255.255").unwrap(),
    description: "carrier-grade NAT",
});

static LOOPBACK: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("127.0.0.0-127.255.255.255").unwrap(),
    description: "loopback",
});

static LINK_LOCAL: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("169.254.0.0-169.254.255.255").unwrap(),
    description: "link local",
});

static IETF_PROTOCOL: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("192.0.0.0-192.0.0.255").unwrap(),
    description: "IETF protocol assignments",
});

static TEST_NET_1: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("192.0.2.0-192.0.2.255").unwrap(),
    description: "TEST-NET-1",
});

static NETWORK_INTERCONNECT: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("198.18.0.0-198.19.255.255").unwrap(),
    description: "network interconnect device benchmark testing",
});

static TEST_NET_2: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("198.51.100.0-198.51.100.255").unwrap(),
    description: "TEST-NET-2",
});

static TEST_NET_3: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("203.0.113.0-203.0.113.255").unwrap(),
    description: "TEST-NET-3",
});

static MULTICAST: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("224.0.0.0-239.255.255.255").unwrap(),
    description: "multicast",
});

static FUTURE_USE: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("240.0.0.0-255.255.255.255").unwrap(),
    description: "future use",
});

// IPv6 bogons

static NODE_SCOPE_UNSPECIFIED: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("::").unwrap(),
    description: "node-scope unicast unspecified",
});

static NODE_SCOPE_LOOPBACK: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("::1").unwrap(),
    description: "node-scope unicast loopback",
});

static IPV4_MAPPED: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("::ffff:0.0.0.0-::ffff:255.255.255.255").unwrap(),
    description: "IPv4-mapped",
});

static IPV4_COMPATIBLE: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("::-::255.255.255.255").unwrap(),
    description: "IPv4-compatible",
});

static REMOTELY_TRIGGERED: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("100::-100::ffff:ffff:ffff:ffff").unwrap(),
    description: "remotely triggered black hole",
});

static ORCHID: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("2001:10::-2001:1f:ffff:ffff:ffff:ffff:ffff:ffff").unwrap(),
    description: "ORCHID",
});

static DOCUMENTATION_PREFIX: Lazy<Bogon> = Lazy::new(|| {
    Bogon {
        range: IpCollection::new("2001:db8::-2001:db8:ffff:ffff:ffff:ffff:ffff:ffff, 3fff::-3fff:fff:ffff:ffff:ffff:ffff:ffff:ffff")
            .unwrap(),
        description: "documentation prefix",
    }
});

static ULA: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("fc00::-fdff:ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap(),
    description: "ULA",
});

static LINK_LOCAL_UNICAST: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("fe80::-febf:ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap(),
    description: "link-local unicast",
});

static SITE_LOCAL_UNICAST: Lazy<Bogon> = Lazy::new(|| Bogon {
    range: IpCollection::new("fec0::-feff:ffff:ffff:ffff:ffff:ffff:ffff:ffff").unwrap(),
    description: "site-local unicast",
});

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_is_bogon_no() {
        // loop {
        //     std::thread::sleep(std::time::Duration::from_secs(1));
        //     let instant = std::time::Instant::now();
        assert_eq!(is_bogon(IpAddr::from_str("8.8.8.8").unwrap()), None);
        assert_eq!(
            is_bogon(IpAddr::from_str("2001:4860:4860::8888").unwrap()),
            None
        );
        //     let elapsed = instant.elapsed();
        //     println!("test_is_bogon_no: {elapsed:?}");
        // }
    }

    #[test]
    fn test_is_bogon_this_network() {
        assert_eq!(
            is_bogon(IpAddr::from_str("0.1.2.3").unwrap()),
            Some("\"this\" network")
        );
    }

    #[test]
    fn test_is_bogon_private_use_networks() {
        assert_eq!(
            is_bogon(IpAddr::from_str("10.1.2.3").unwrap()),
            Some("private-use")
        );
        assert_eq!(
            is_bogon(IpAddr::from_str("172.22.2.3").unwrap()),
            Some("private-use")
        );
        assert_eq!(
            is_bogon(IpAddr::from_str("192.168.255.3").unwrap()),
            Some("private-use")
        );
    }

    #[test]
    fn test_is_bogon_carrier_grade() {
        assert_eq!(
            is_bogon(IpAddr::from_str("100.99.2.1").unwrap()),
            Some("carrier-grade NAT")
        );
    }

    #[test]
    fn test_is_bogon_loopback() {
        assert_eq!(
            is_bogon(IpAddr::from_str("127.99.2.1").unwrap()),
            Some("loopback")
        );
    }

    #[test]
    fn test_is_bogon_link_local() {
        assert_eq!(
            is_bogon(IpAddr::from_str("169.254.0.0").unwrap()),
            Some("link local")
        );
    }

    #[test]
    fn test_is_bogon_ietf() {
        assert_eq!(
            is_bogon(IpAddr::from_str("192.0.0.255").unwrap()),
            Some("IETF protocol assignments")
        );
    }

    #[test]
    fn test_is_bogon_test_net_1() {
        assert_eq!(
            is_bogon(IpAddr::from_str("192.0.2.128").unwrap()),
            Some("TEST-NET-1")
        );
    }

    #[test]
    fn test_is_bogon_network_interconnect() {
        assert_eq!(
            is_bogon(IpAddr::from_str("198.18.2.128").unwrap()),
            Some("network interconnect device benchmark testing")
        );
    }

    #[test]
    fn test_is_bogon_test_net_2() {
        assert_eq!(
            is_bogon(IpAddr::from_str("198.51.100.128").unwrap()),
            Some("TEST-NET-2")
        );
    }

    #[test]
    fn test_is_bogon_test_net_3() {
        assert_eq!(
            is_bogon(IpAddr::from_str("203.0.113.128").unwrap()),
            Some("TEST-NET-3")
        );
    }

    #[test]
    fn test_is_bogon_multicast() {
        assert_eq!(
            is_bogon(IpAddr::from_str("224.12.13.255").unwrap()),
            Some("multicast")
        );
    }

    #[test]
    fn test_is_bogon_future_use() {
        assert_eq!(
            is_bogon(IpAddr::from_str("240.0.0.0").unwrap()),
            Some("future use")
        );
    }

    #[test]
    fn test_node_scope_unspecified() {
        assert_eq!(
            is_bogon(IpAddr::from_str("::").unwrap()),
            Some("node-scope unicast unspecified")
        );
    }

    #[test]
    fn test_node_scope_loopback() {
        assert_eq!(
            is_bogon(IpAddr::from_str("::1").unwrap()),
            Some("node-scope unicast loopback")
        );
    }

    #[test]
    fn test_ipv4_mapped() {
        assert_eq!(
            is_bogon(IpAddr::from_str("::ffff:8.8.8.8").unwrap()),
            Some("IPv4-mapped")
        );
    }

    #[test]
    fn test_ipv4_compatible() {
        assert_eq!(
            is_bogon(IpAddr::from_str("::8.8.8.8").unwrap()),
            Some("IPv4-compatible")
        );
    }

    #[test]
    fn test_remotely_triggered() {
        assert_eq!(
            is_bogon(IpAddr::from_str("100::beef").unwrap()),
            Some("remotely triggered black hole")
        );
    }

    #[test]
    fn test_orchid() {
        assert_eq!(
            is_bogon(IpAddr::from_str("2001:10::feed").unwrap()),
            Some("ORCHID")
        );
    }

    #[test]
    fn test_documentation_prefix() {
        assert_eq!(
            is_bogon(IpAddr::from_str("2001:db8::fe90").unwrap()),
            Some("documentation prefix")
        );
        assert_eq!(
            is_bogon(IpAddr::from_str("3fff::").unwrap()),
            Some("documentation prefix")
        );
    }

    #[test]
    fn test_ula() {
        assert_eq!(is_bogon(IpAddr::from_str("fdff::").unwrap()), Some("ULA"));
    }

    #[test]
    fn test_link_local_unicast() {
        assert_eq!(
            is_bogon(IpAddr::from_str("feaf::").unwrap()),
            Some("link-local unicast")
        );
    }

    #[test]
    fn test_site_local_unicast() {
        assert_eq!(
            is_bogon(IpAddr::from_str("feea::1").unwrap()),
            Some("site-local unicast")
        );
    }
}
