use chrono::Utc;
use log::Record;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub(crate) struct DatastoreEntry {
    timestamp: String,
    level: String,
    message : String,
}

impl DatastoreEntry {
    pub(crate) fn new(record: &Record) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let level = record.level().to_string();
        let message = record.args().to_string();
        Self {
            timestamp,
            level,
            message
        }
    }
}