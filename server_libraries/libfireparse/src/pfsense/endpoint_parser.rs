use roxmltree::Node;

/// A parser for extracting endpoint information from `source` and `destination` nodes.
pub struct EndpoingParser {}

impl EndpoingParser {
    /// Parses a `source` or `destination` node to extract the address and port.
    ///
    /// # Arguments
    /// * `node` - An optional `Node` representing an endpoint.
    ///
    /// # Returns
    /// A tuple `(String, String)` where:
    /// - The first element is the address, extracted from the `<address>` or `<network>` tag, or `"*"` if missing.
    /// - The second element is the port, extracted from the `<port>` tag, or `"*"` if missing.
    ///
    /// If the `<any>` tag is present inside the node, it returns `("*", "*")`.
    /// If the node is `None`, it also returns `("*", "*")`.
    pub fn parse(node: Option<Node>) -> (String, String) {
        if node.is_none() {
            return (String::from("*"), String::from("*"));
        }

        let node_value = node.unwrap();

        if node_value
            .children()
            .find(|e| e.has_tag_name("any"))
            .is_some()
        {
            return (String::from("*"), String::from("*"));
        }

        (
            EndpoingParser::parse_addr(&node_value),
            EndpoingParser::parse_port(&node_value),
        )
    }

    /// Extracts the port from a `source` or `destination` node.
    ///
    /// # Arguments
    /// * `node` - A reference to an XML node.
    ///
    /// # Returns
    /// A `String` containing the port value extracted from the `<port>` tag.
    /// If the `<port>` tag is missing, it defaults to `"*"`.
    fn parse_port(node: &Node) -> String {
        node.children()
            .find(|e| e.has_tag_name("port"))
            .and_then(|e| e.text())
            .unwrap_or("*")
            .to_string()
    }

    /// Extracts the address from a `source` or `destination` node.
    ///
    /// # Arguments
    /// * `node` - A reference to an XML node.
    ///
    /// # Returns
    /// A `String` containing:
    /// - The value from the `<address>` tag, if present.
    /// - The value from the `<network>` tag, if `<address>` is missing.
    /// - `"*"` if neither tag is found.
    fn parse_addr(node: &Node) -> String {
        if let Some(address) = node
            .children()
            .find(|e| e.has_tag_name("address"))
            .and_then(|e| e.text())
        {
            return String::from(address);
        }

        if let Some(network) = node
            .children()
            .find(|e| e.has_tag_name("network"))
            .and_then(|e| e.text())
        {
            return String::from(network);
        }

        String::from("*")
    }
}

#[cfg(test)]
mod tests {
    use super::EndpoingParser;
    use roxmltree::Document;

    #[test]
    fn test_parse_destination_with_address_and_port() {
        let xml = r#"
        <root>
            <destination>
                <address>1.1.1.1</address>
                <port>8080</port>
            </destination>
        </root>
        "#;

        let doc = Document::parse(xml).expect("Failed to parse XML");
        let node = doc
            .descendants()
            .find(|n| n.has_tag_name("destination"))
            .expect("Tag not found");

        let (addr, port) = EndpoingParser::parse(Some(node));
        assert_eq!(addr, "1.1.1.1");
        assert_eq!(port, "8080");
    }

    #[test]
    fn test_parse_destination_with_address_and_port_range() {
        let xml = r#"
        <root>
            <destination>
                <address>1.1.1.1</address>
                <port>8080-8090</port>
            </destination>
        </root>
        "#;

        let doc = Document::parse(xml).expect("Failed to parse XML");
        let node = doc
            .descendants()
            .find(|n| n.has_tag_name("destination"))
            .expect("Tag not found");

        let (addr, port) = EndpoingParser::parse(Some(node));
        assert_eq!(addr, "1.1.1.1");
        assert_eq!(port, "8080-8090");
    }

    #[test]
    fn test_parse_destination_with_any() {
        let xml = r#"
        <root>
            <destination>
                <any></any>
            </destination>
        </root>
        "#;

        let doc = Document::parse(xml).expect("Failed to parse XML");
        let node = doc
            .descendants()
            .find(|n| n.has_tag_name("destination"))
            .expect("Tag not found");

        let (addr, port) = EndpoingParser::parse(Some(node));
        assert_eq!(addr, "*");
        assert_eq!(port, "*");
    }

    #[test]
    fn test_parse_source_with_network() {
        let xml = r#"
        <root>
            <source>
                <network>wanip</network>
            </source>
        </root>
        "#;

        let doc = Document::parse(xml).expect("Failed to parse XML");
        let node = doc
            .descendants()
            .find(|n| n.has_tag_name("source"))
            .expect("Tag not found");

        let (addr, port) = EndpoingParser::parse(Some(node));
        assert_eq!(addr, "wanip");
        assert_eq!(port, "*");
    }

    #[test]
    fn test_parse_destination_with_no_address_or_network() {
        let xml = r#"
        <root>
            <destination>
                <port>9000</port>
            </destination>
        </root>
        "#;

        let doc = Document::parse(xml).expect("Failed to parse XML");
        let node = doc
            .descendants()
            .find(|n| n.has_tag_name("destination"))
            .expect("Tag not found");

        let (addr, port) = EndpoingParser::parse(Some(node));
        assert_eq!(addr, "*");
        assert_eq!(port, "9000");
    }

    #[test]
    fn test_parse_destination_with_no_port() {
        let xml = r#"
        <root>
            <destination>
                <address>2.2.2.2</address>
            </destination>
        </root>
        "#;

        let doc = Document::parse(xml).expect("Failed to parse XML");
        let node = doc
            .descendants()
            .find(|n| n.has_tag_name("destination"))
            .expect("Tag not found");

        let (addr, port) = EndpoingParser::parse(Some(node));
        assert_eq!(addr, "2.2.2.2");
        assert_eq!(port, "*");
    }

    #[test]
    fn test_no_node_returns_default_results() {
        let (addr, port) = EndpoingParser::parse(None);
        assert_eq!(addr, "*");
        assert_eq!(port, "*");
    }
}
