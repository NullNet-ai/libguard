#[derive(Clone)]
pub(crate) struct GenericLog {
    pub(crate) timestamp: String,
    pub(crate) level: String,
    pub(crate) message: String,
}

impl From<GenericLog> for nullnet_libappguard::Log {
    fn from(val: GenericLog) -> nullnet_libappguard::Log {
        nullnet_libappguard::Log {
            timestamp: val.timestamp,
            level: val.level,
            message: val.message,
        }
    }
}

impl From<GenericLog> for nullnet_libwallguard::Log {
    fn from(val: GenericLog) -> nullnet_libwallguard::Log {
        nullnet_libwallguard::Log {
            timestamp: val.timestamp,
            level: val.level,
            message: val.message,
        }
    }
}
