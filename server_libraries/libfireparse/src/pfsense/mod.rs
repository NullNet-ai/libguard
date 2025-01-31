use crate::{utils, Configuration, FireparseError};
use aliases_parser::AliasesParser;
use roxmltree::Document;
use rules_parser::PfSenseRulesParser;

mod aliases_parser;
mod endpoint_parser;
mod rules_parser;

/// A parser for extracting configuration details from a pfSense XML configuration.
pub struct PfSenseParser {}

impl PfSenseParser {
    /// Parses a pfSense XML configuration and extracts aliases, rules, and raw data.
    ///
    /// # Arguments
    /// * `document` - A string slice (`&str`) containing the XML configuration data.
    ///
    /// # Returns
    /// * `Ok(Configuration)` - If parsing is successful, returns a `Configuration` struct containing:
    ///   - `raw_data`: Base64-encoded XML data.
    ///   - `aliases`: Parsed aliases from `<aliases>`.
    ///   - `rules`: Parsed firewall and NAT rules from `<filter>` and `<nat>`.
    /// * `Err(FireparseError)` - If parsing fails, returns a `FireparseError::ParserError`.
    pub fn parse(document: &str) -> Result<Configuration, FireparseError> {
        let xmltree =
            Document::parse(document).map_err(|e| FireparseError::ParserError(e.to_string()))?;

        Ok(Configuration {
            raw_content: utils::encode_base64(document.as_bytes()),
            aliases: AliasesParser::parse(&xmltree),
            rules: PfSenseRulesParser::parse(&xmltree),
        })
    }
}
