use serde::Deserialize;

/// Represents a device associated with an account.
/// Contains detailed information about the device, including identifiers, timestamps, and location data.
#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: String,
    pub categories: Vec<String>,
    pub code: String,
    pub tombstone: u32,
    pub status: String,
    pub version: u32,
    pub created_date: Option<String>,
    pub created_time: Option<String>,
    pub updated_date: Option<String>,
    pub updated_time: Option<String>,
    pub organization_id: String,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    pub timestamp: Option<String>,
    pub tags: Vec<String>,
}

/// Represents an organization associated with an account.
/// Contains metadata about the organization, including its identifiers and hierarchy.
#[derive(Debug, Deserialize)]
pub struct Organization {
    pub id: String,
    pub categories: Vec<String>,
    pub code: Option<String>,
    pub tombstone: u32,
    pub status: String,
    pub version: u32,
    pub created_date: String,
    pub created_time: String,
    pub updated_date: String,
    pub updated_time: String,
    pub organization_id: String,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_by: Option<String>,
    pub requested_by: Option<String>,
    pub timestamp: Option<String>,
    pub tags: Vec<String>,
    pub parent_organization_id: Option<String>,
    pub name: String,
}

/// Represents an account containing a device and organization.
/// Acts as a container for the relationships between devices and organizations.
#[derive(Debug, Deserialize)]
pub struct Account {
    pub device: Device,
    pub organization: Organization,
    pub organization_id: String,
    pub account_id: String,
}
