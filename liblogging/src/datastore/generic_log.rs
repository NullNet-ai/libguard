#[derive(Clone)]
pub(crate) struct GenericLog {
    pub(crate) timestamp: String,
    pub(crate) level: String,
    pub(crate) message: String,
}

impl From<GenericLog> for nullnet_libappguard::appguard::Log {
    fn from(val: GenericLog) -> nullnet_libappguard::appguard::Log {
        nullnet_libappguard::appguard::Log {
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
