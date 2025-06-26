use crate::utils::empty_object_or_null_is_none;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(default)]
    pub is_root_account: bool,
    #[serde(default, deserialize_with = "empty_object_or_null_is_none")]
    pub profile: Option<Profile>,
    #[serde(default, deserialize_with = "empty_object_or_null_is_none")]
    pub contact: Option<Contact>,
    #[serde(default, deserialize_with = "empty_object_or_null_is_none")]
    pub device: Option<Device>,
    pub organization: Organization,
    pub id: String,
    pub account_id: String,
    pub organization_id: String,
    pub account_organization_id: Option<String>,
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
