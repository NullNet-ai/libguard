use nullnet_libconfmon::InterfaceSnapshot;
use roxmltree::Document;

use crate::models::NetworkInterface;

/// A parser for extracting network interfaces from a pfSense XML configuration.
pub struct PfSenseInterfacesParser {}

impl PfSenseInterfacesParser {
    /// Parses a given XML document and extracts network interface details.
    ///
    /// # Arguments
    ///
    /// * `document` - A reference to a `roxmltree::Document` containing pfSense configuration.
    ///
    /// # Returns
    ///
    /// A vector of `NetworkInterface` structs containing parsed interface details.
    pub fn parse(
        document: &Document,
        os_interfaces: Vec<InterfaceSnapshot>,
    ) -> Vec<NetworkInterface> {
        let mut interfaces = vec![];

        if let Some(interfaces_node) = document
            .descendants()
            .find(|e| e.has_tag_name("pfsense"))
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

                let mut address = interface
                    .children()
                    .find(|c| c.has_tag_name("ipaddr"))
                    .and_then(|c| c.text())
                    .unwrap_or("none")
                    .to_string();

                if let Some(data) = os_interfaces.iter().find(|iface| iface.name == device) {
                    // !!!!!!!!!!!!!!!!! @TODO !!!!!!!!!!!!!!!!!!!!!!!!
                    // !!!Save all addresses, not only the first one!!!
                    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                    match data.ip_addresses.first() {
                        Some(addr) => address = addr.to_string(),
                        None => {}
                    };
                }

                interfaces.push(NetworkInterface {
                    name,
                    device,
                    address,
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
        let xml = r#"<pfsense>
                        <interfaces>
                            <wan>
                                <if>igb0</if>
                                <ipaddr>192.168.1.1</ipaddr>
                            </wan>
                            <lan>
                                <if>igb1</if>
                                <ipaddr>192.168.1.2</ipaddr>
                            </lan>
                        </interfaces>
                    </pfsense>"#;
        let doc = Document::parse(xml).expect("Failed to parse XML");
        let interfaces = PfSenseInterfacesParser::parse(&doc, vec![]);

        assert_eq!(interfaces.len(), 2);
        assert_eq!(interfaces[0].name, "wan");
        assert_eq!(interfaces[0].device, "igb0");
        assert_eq!(interfaces[0].address, "192.168.1.1");
        assert_eq!(interfaces[1].name, "lan");
        assert_eq!(interfaces[1].device, "igb1");
        assert_eq!(interfaces[1].address, "192.168.1.2");
    }

    #[test]
    fn test_parse_missing_elements() {
        let xml = r#"<pfsense>
                        <interfaces>
                            <wan></wan>
                        </interfaces>
                    </pfsense>"#;
        let doc = Document::parse(xml).expect("Failed to parse XML");
        let interfaces = PfSenseInterfacesParser::parse(&doc, vec![]);

        assert_eq!(interfaces.len(), 1);
        assert_eq!(interfaces[0].name, "wan");
        assert_eq!(interfaces[0].device, "none");
        assert_eq!(interfaces[0].address, "none");
    }

    #[test]
    fn test_parse_empty_xml() {
        let xml = "<pfsense></pfsense>";
        let doc = Document::parse(xml).expect("Failed to parse XML");
        let interfaces = PfSenseInterfacesParser::parse(&doc, vec![]);

        assert_eq!(interfaces.len(), 0);
    }

    #[test]
    fn test_ifaces_data_override_xml_contents() {
        let xml = r#"
        <pfsense>
            <interfaces>
                <wan>
                    <if>igb0</if>
                    <ipaddr>192.168.1.1</ipaddr>
                </wan>
            </interfaces>
        </pfsense>"#;

        let doc = Document::parse(xml).expect("Failed to parse XML");

        let iface_data = InterfaceSnapshot {
            name: String::from("igb0"),
            is_up: true,
            is_loopback: true,
            is_multicast: true,
            is_broadcast: true,
            mac_address: None,
            interface_index: None,
            ip_addresses: vec!["8.8.8.8".parse().unwrap()],
            subnet_mask: None,
            gateway: None,
        };

        let interfaces = PfSenseInterfacesParser::parse(&doc, vec![iface_data]);

        assert_eq!(interfaces.len(), 1);
        assert_eq!(interfaces[0].name, "wan");
        assert_eq!(interfaces[0].device, "igb0");
        assert_eq!(interfaces[0].address, "8.8.8.8");
    }
}
