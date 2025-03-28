use std::net::IpAddr;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub(super) struct IpCollection {
    ips: Vec<IpAddr>,
    ranges: Vec<RangeInclusive<IpAddr>>,
}

impl IpCollection {
    const SEPARATOR: char = ',';
    const RANGE_SEPARATOR: char = '-';

    pub(super) fn new(str: &str) -> Option<Self> {
        let str = str.replace(' ', "");

        let mut ips = Vec::new();
        let mut ranges = Vec::new();

        let objects: Vec<&str> = str.split(Self::SEPARATOR).collect();
        for object in objects {
            if object.contains(Self::RANGE_SEPARATOR) {
                // IP range
                let mut subparts = object.split(Self::RANGE_SEPARATOR);
                let (lower_str, upper_str) =
                    (subparts.next().unwrap_or(""), subparts.next().unwrap_or(""));
                let lower_ip_res = IpAddr::from_str(lower_str);
                let upper_ip_res = IpAddr::from_str(upper_str);
                if lower_ip_res.is_ok() && upper_ip_res.is_ok() {
                    let lower_ip = lower_ip_res.ok()?;
                    let upper_ip = upper_ip_res.ok()?;
                    let range = RangeInclusive::new(lower_ip, upper_ip);
                    if range.is_empty() || lower_ip.is_ipv4() != upper_ip.is_ipv4() {
                        return None;
                    }
                    ranges.push(range);
                } else {
                    return None;
                }
            } else {
                // individual IP
                if let Ok(ip) = IpAddr::from_str(object) {
                    ips.push(ip);
                } else {
                    return None;
                }
            }
        }

        Some(Self { ips, ranges })
    }

    pub(super) fn contains(&self, ip: &IpAddr) -> bool {
        for range in &self.ranges {
            if range.contains(ip) {
                return true;
            }
        }
        self.ips.contains(ip)
    }
}

#[cfg(test)]
mod tests {
    use crate::bogon::ip_collection::IpCollection;
    use std::net::IpAddr;
    use std::ops::RangeInclusive;
    use std::str::FromStr;

    #[test]
    fn test_new_collections_1() {
        assert_eq!(
            IpCollection::new("1.1.1.1,2.2.2.2").unwrap(),
            IpCollection {
                ips: vec![
                    IpAddr::from_str("1.1.1.1").unwrap(),
                    IpAddr::from_str("2.2.2.2").unwrap()
                ],
                ranges: vec![]
            }
        );

        assert_eq!(
            IpCollection::new("1.1.1.1, 2.2.2.2, 3.3.3.3 - 5.5.5.5, 10.0.0.1-10.0.0.255,9.9.9.9",)
                .unwrap(),
            IpCollection {
                ips: vec![
                    IpAddr::from_str("1.1.1.1").unwrap(),
                    IpAddr::from_str("2.2.2.2").unwrap(),
                    IpAddr::from_str("9.9.9.9").unwrap()
                ],
                ranges: vec![
                    RangeInclusive::new(
                        IpAddr::from_str("3.3.3.3").unwrap(),
                        IpAddr::from_str("5.5.5.5").unwrap()
                    ),
                    RangeInclusive::new(
                        IpAddr::from_str("10.0.0.1").unwrap(),
                        IpAddr::from_str("10.0.0.255").unwrap()
                    )
                ]
            }
        );

        assert_eq!(
            IpCollection::new("  aaaa::ffff,bbbb::1-cccc::2").unwrap(),
            IpCollection {
                ips: vec![IpAddr::from_str("aaaa::ffff").unwrap(),],
                ranges: vec![RangeInclusive::new(
                    IpAddr::from_str("bbbb::1").unwrap(),
                    IpAddr::from_str("cccc::2").unwrap()
                )]
            }
        );
    }

    #[test]
    fn test_new_collections_2() {
        assert_eq!(
            IpCollection::new("1.1.1.1,2.2.2.2, 8.8.8.8   ").unwrap(),
            IpCollection {
                ips: vec![
                    IpAddr::from_str("1.1.1.1").unwrap(),
                    IpAddr::from_str("2.2.2.2").unwrap(),
                    IpAddr::from_str("8.8.8.8").unwrap()
                ],
                ranges: vec![]
            }
        );

        assert_eq!(
            IpCollection::new("  1.1.1.1 -1.1.1.1").unwrap(),
            IpCollection {
                ips: vec![],
                ranges: vec![RangeInclusive::new(
                    IpAddr::from_str("1.1.1.1").unwrap(),
                    IpAddr::from_str("1.1.1.1").unwrap()
                ),]
            }
        );

        assert_eq!(
            IpCollection::new("1.1.1.1,2.2.2.2,3.3.3.3-5.5.5.5,10.0.0.1-10.0.0.255,9.9.9.9",)
                .unwrap(),
            IpCollection {
                ips: vec![
                    IpAddr::from_str("1.1.1.1").unwrap(),
                    IpAddr::from_str("2.2.2.2").unwrap(),
                    IpAddr::from_str("9.9.9.9").unwrap()
                ],
                ranges: vec![
                    RangeInclusive::new(
                        IpAddr::from_str("3.3.3.3").unwrap(),
                        IpAddr::from_str("5.5.5.5").unwrap()
                    ),
                    RangeInclusive::new(
                        IpAddr::from_str("10.0.0.1").unwrap(),
                        IpAddr::from_str("10.0.0.255").unwrap()
                    )
                ]
            }
        );

        assert_eq!(
            IpCollection::new("aaaa::ffff,bbbb::1-cccc::2,ff::dd").unwrap(),
            IpCollection {
                ips: vec![
                    IpAddr::from_str("aaaa::ffff").unwrap(),
                    IpAddr::from_str("ff::dd").unwrap()
                ],
                ranges: vec![RangeInclusive::new(
                    IpAddr::from_str("bbbb::1").unwrap(),
                    IpAddr::from_str("cccc::2").unwrap()
                )]
            }
        );
    }

    #[test]
    fn test_new_collections_invalid() {
        assert_eq!(IpCollection::new(""), None);

        assert_eq!(IpCollection::new("  "), None);

        assert_eq!(
            IpCollection::new("1.1.1.1,2.2.2.2,3.3.3.3-5.5.5.5,10.0.0.1-10.0.0.255,9.9.9"),
            None
        );

        assert_eq!(
            IpCollection::new("1.1.1.1,2.2.2.2,3.3.3.3-5.5.5.5,10.0.0.1:10.0.0.255,9.9.9.9"),
            None
        );

        assert_eq!(IpCollection::new("1.1.1.1-aa::ff"), None);

        assert_eq!(IpCollection::new("aa::ff-1.1.1.1"), None);

        assert_eq!(IpCollection::new("aa::ff-aa::ee"), None);

        assert_eq!(IpCollection::new("1.1.1.1-1.1.0.1"), None);
    }

    #[test]
    fn test_ip_collection_contains() {
        let collection =
            IpCollection::new("1.1.1.1,2.2.2.2,3.3.3.3-5.5.5.5,10.0.0.1-10.0.0.255,9.9.9.9")
                .unwrap();
        assert!(collection.contains(&IpAddr::from_str("1.1.1.1").unwrap()));
        assert!(collection.contains(&IpAddr::from_str("2.2.2.2").unwrap()));
        assert!(collection.contains(&IpAddr::from_str("3.3.3.3").unwrap()));
        assert!(collection.contains(&IpAddr::from_str("4.0.0.0").unwrap()));
        assert!(collection.contains(&IpAddr::from_str("5.5.5.5").unwrap()));
        assert!(collection.contains(&IpAddr::from_str("9.9.9.9").unwrap()));
        assert!(collection.contains(&IpAddr::from_str("10.0.0.1").unwrap()));
        assert!(collection.contains(&IpAddr::from_str("10.0.0.128").unwrap()));
        assert!(collection.contains(&IpAddr::from_str("10.0.0.255").unwrap()));
        assert!(!collection.contains(&IpAddr::from_str("10.0.0.0").unwrap()));
        assert!(!collection.contains(&IpAddr::from_str("2.2.2.1").unwrap()));
        assert!(!collection.contains(&IpAddr::from_str("9.9.9.10").unwrap()));
        assert!(!collection.contains(&IpAddr::from_str("3.3.3.2").unwrap()));

        let collection_2 = IpCollection::new("1.1.1.0-1.1.9.0").unwrap();
        assert!(!collection_2.contains(&IpAddr::from_str("1.1.100.5").unwrap()));
        assert!(collection_2.contains(&IpAddr::from_str("1.1.3.255").unwrap()));

        // check that ipv4 range doesn't contain ipv6
        let collection_3 = IpCollection::new("0.0.0.0-255.255.255.255").unwrap();
        assert!(!collection_3.contains(&IpAddr::from_str("::").unwrap()));
        assert!(!collection_3.contains(&IpAddr::from_str("1111::2222").unwrap()));
    }

    #[test]
    fn test_ip_collection_contains_ipv6() {
        let collection =
            IpCollection::new( "2001:db8:1234:0000:0000:0000:0000:0000-2001:db8:1234:ffff:ffff:ffff:ffff:ffff,daa::aad,caa::aac").unwrap();
        assert!(
            collection
                .contains(&IpAddr::from_str("2001:db8:1234:0000:0000:0000:0000:0000").unwrap())
        );
        assert!(
            collection
                .contains(&IpAddr::from_str("2001:db8:1234:ffff:ffff:ffff:ffff:ffff").unwrap())
        );
        assert!(
            collection
                .contains(&IpAddr::from_str("2001:db8:1234:ffff:ffff:ffff:ffff:eeee").unwrap())
        );
        assert!(
            collection
                .contains(&IpAddr::from_str("2001:db8:1234:aaaa:ffff:ffff:ffff:eeee").unwrap())
        );
        assert!(collection.contains(&IpAddr::from_str("daa::aad").unwrap()));
        assert!(collection.contains(&IpAddr::from_str("caa::aac").unwrap()));
        assert!(
            !collection
                .contains(&IpAddr::from_str("2000:db8:1234:0000:0000:0000:0000:0000").unwrap())
        );
        assert!(
            !collection
                .contains(&IpAddr::from_str("2001:db8:1235:ffff:ffff:ffff:ffff:ffff").unwrap())
        );
        assert!(
            !collection
                .contains(&IpAddr::from_str("2001:eb8:1234:ffff:ffff:ffff:ffff:eeee").unwrap())
        );
        assert!(!collection.contains(&IpAddr::from_str("da::aad").unwrap()));
        assert!(!collection.contains(&IpAddr::from_str("caa::aab").unwrap()));

        let collection_2 = IpCollection::new("aa::bb-aa:1::00").unwrap();
        assert!(!collection_2.contains(&IpAddr::from_str("aa:11::0").unwrap()));
        assert!(collection_2.contains(&IpAddr::from_str("aa::bc").unwrap()));
        assert!(collection_2.contains(&IpAddr::from_str("aa::bbcc").unwrap()));
        assert!(collection_2.contains(&IpAddr::from_str("00aa:0001::00").unwrap()));

        // check that ipv6 range doesn't contain ipv4
        let collection_3 = IpCollection::new("0000::0000-ffff::8888").unwrap();
        assert!(!collection_3.contains(&IpAddr::from_str("192.168.1.1").unwrap()));
        assert!(!collection_3.contains(&IpAddr::from_str("0.0.0.0").unwrap()));
    }
}
