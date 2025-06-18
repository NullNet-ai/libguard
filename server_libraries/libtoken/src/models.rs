use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub profile: Profile,
    pub contact: Option<Contact>,
    pub device: Option<Device>,
    pub organization: Organization,
    pub id: String,
    pub account_id: String,
    pub organization_id: String,
    pub account_organization_id: String,
    pub account_status: String,
    pub role_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub account_id: String,
    pub categories: Vec<String>,
    pub code: Option<String>,
    pub status: String,
    pub organization_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub account_id: String,
    pub code: String,
    pub categories: Vec<String>,
    pub status: String,
    pub organization_id: String,
    pub date_of_birth: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub code: String,
    pub categories: Vec<String>,
    pub status: String,
    pub organization_id: String,
    pub timestamp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub code: String,
    pub categories: Vec<String>,
    pub status: String,
    pub organization_id: String,
    pub parent_organization_id: Option<String>,
}
