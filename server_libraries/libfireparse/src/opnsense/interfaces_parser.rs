use nullnet_libconfmon::InterfaceSnapshot;
use roxmltree::Document;

use crate::{IpAddress, NetworkInterface};

pub struct OpnSenseInterfacesParser {}

impl OpnSenseInterfacesParser {
    pub fn parse(
        document: &Document,
        os_interfaces: Vec<InterfaceSnapshot>,
    ) -> Vec<NetworkInterface> {
        let mut interfaces = vec![];

        if let Some(interfaces_node) = document
            .descendants()
            .find(|e| e.has_tag_name("opnsense"))
            .and_then(|e| e.children().find(|ce| ce.has_tag_name("interfaces")))
        {
            for interface in interfaces_node.children().filter(|e| e.is_element()) {
                let name = interface.tag_name().name().to_string();

                let device = interface
                    .children()
                    .find(|c| c.has_tag_name("if"))
                    .and_then(|v| v.text())
                    .unwrap_or("none")
                    .to_string();

                let mut addresses = vec![];
                if let Some(data) = os_interfaces.iter().find(|iface| iface.name == device) {
                    addresses = data
                        .ip_addresses
                        .iter()
                        .map(|addr| IpAddress {
                            address: addr.to_string(),
                            version: if addr.is_ipv4() { 4 } else { 5 },
                        })
                        .collect();
                }

                if addresses.is_empty() {
                    if let Some(address_v4) = interface
                        .children()
                        .find(|c| c.has_tag_name("ipaddr"))
                        .and_then(|c| c.text())
                    {
                        addresses.push(IpAddress {
                            address: String::from(address_v4),
                            version: 4,
                        });
                    }

                    if let Some(address_v6) = interface
                        .children()
                        .find(|c| c.has_tag_name("ipaddrv6"))
                        .and_then(|c| c.text())
                    {
                        addresses.push(IpAddress {
                            address: String::from(address_v6),
                            version: 6,
                        });
                    }
                }

                interfaces.push(NetworkInterface {
                    name,
                    device,
                    addresses,
                });
            }
        }

        interfaces
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roxmltree::Document;

    #[test]
    fn test_parse_valid_xml() {
        let xml = r#"<opnsense>
                        <interfaces>
                            <wan>
                                <if>igb0</if>
                                <ipaddr>192.168.1.1</ipaddr>
                            </wan>
                            <lan>
                                <if>igb1</if>
                                <ipaddr>dhcp</ipaddr>
                            </lan>
                        </interfaces>
                    </opnsense>"#;
        let doc = Document::parse(xml).expect("Failed to parse XML");
        let interfaces = OpnSenseInterfacesParser::parse(&doc, vec![]);

        assert_eq!(interfaces.len(), 2);
        assert_eq!(interfaces[0].name, "wan");
        assert_eq!(interfaces[0].device, "igb0");
        assert_eq!(interfaces[0].addresses.len(), 1);
        assert_eq!(interfaces[0].addresses[0].address, "192.168.1.1");
        assert_eq!(interfaces[0].addresses[0].version, 4);

        assert_eq!(interfaces[1].name, "lan");
        assert_eq!(interfaces[1].device, "igb1");

        assert_eq!(interfaces[1].addresses.len(), 1);
        assert_eq!(interfaces[1].addresses[0].address, "dhcp");
        assert_eq!(interfaces[1].addresses[0].version, 4);
    }

    #[test]
    fn test_parse_missing_elements() {
        let xml = r#"<opnsense>
                        <interfaces>
                            <wan></wan>
                        </interfaces>
                    </opnsense>"#;
        let doc = Document::parse(xml).expect("Failed to parse XML");
        let interfaces = OpnSenseInterfacesParser::parse(&doc, vec![]);

        assert_eq!(interfaces.len(), 1);
        assert_eq!(interfaces[0].name, "wan");
        assert_eq!(interfaces[0].device, "none");
        assert_eq!(interfaces[0].addresses.len(), 0);
    }

    #[test]
    fn test_parse_empty_xml() {
        let xml = "<opnsense></opnsense>";
        let doc = Document::parse(xml).expect("Failed to parse XML");
        let interfaces = OpnSenseInterfacesParser::parse(&doc, vec![]);

        assert_eq!(interfaces.len(), 0);
    }

    #[test]
    fn test_ifaces_data_override_xml_contents() {
        let xml = r#"
        <opnsense>
            <interfaces>
                <wan>
                    <if>igb0</if>
                    <ipaddr>192.168.1.1</ipaddr>
                </wan>
            </interfaces>
        </opnsense>"#;

        let doc = Document::parse(xml).expect("Failed to parse XML");

        let iface_data = InterfaceSnapshot {
            name: String::from("igb0"),
            is_up: true,
            is_loopback: true,
            is_multicast: true,
            is_broadcast: true,
            mac_address: None,
            interface_index: None,
            ip_addresses: vec!["8.8.8.8".parse().unwrap(), "8.8.4.4".parse().unwrap()],
            subnet_mask: None,
            gateway: None,
        };

        let interfaces = OpnSenseInterfacesParser::parse(&doc, vec![iface_data]);

        assert_eq!(interfaces.len(), 1);
        assert_eq!(interfaces[0].name, "wan");
        assert_eq!(interfaces[0].device, "igb0");
        assert_eq!(interfaces[0].addresses.len(), 2);
        assert_eq!(interfaces[0].addresses[0].address, "8.8.8.8");
        assert_eq!(interfaces[0].addresses[0].version, 4);
        assert_eq!(interfaces[0].addresses[1].address, "8.8.4.4");
        assert_eq!(interfaces[0].addresses[1].version, 4);
    }
}
