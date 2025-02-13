use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Alias {
    pub r#type: String,
    pub name: String,
    pub value: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub disabled: bool,
    pub r#type: String,
    pub policy: String,
    pub protocol: String,
    pub source_port: String,
    pub source_addr: String,
    pub destination_port: String,
    pub destination_addr: String,
    pub description: String,
    pub interface: String,
    pub order: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub device: String,
    pub address: String,
}

pub struct Configuration {
    pub rules: Vec<Rule>,
    pub aliases: Vec<Alias>,
    pub interfaces: Vec<NetworkInterface>,
    pub raw_content: String,
    pub hostname: String,
}
