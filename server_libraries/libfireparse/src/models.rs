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
    pub r#type: String,
    pub policy: String,
    pub protocol: String,
    pub source_port: String,
    pub source_addr: String,
    pub destination_port: String,
    pub destination_addr: String,
    pub description: String,
}

pub struct Configuration {
    pub rules: Vec<Rule>,
    pub aliases: Vec<Alias>,
    pub raw_data: String,
}
