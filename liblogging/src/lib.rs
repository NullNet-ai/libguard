mod error;

pub use error::{Error, ErrorHandler, Location};
use log::LevelFilter;
use syslog::Facility;

pub struct Logger {}

pub enum SyslogServer {
    Local,
    Remote(String),
}

impl Logger {
    pub fn init(syslog_server: SyslogServer) {
        match syslog_server {
            SyslogServer::Local => syslog::init_unix(Facility::LOG_USER, LevelFilter::Info),
            SyslogServer::Remote(server) => {
                syslog::init_tcp(server, String::new(), Facility::LOG_USER, LevelFilter::Info)
            }
        }
        .expect("Failed to initialize logger");
    }

    // this is private: errors are logged only via handle_err(location!())
    fn error<S: AsRef<str>>(message: S) {
        let m = message.as_ref();

        log::error!("{m}");

        #[cfg(debug_assertions)]
        println!("[ERROR] {m}");
    }

    pub fn warn<S: AsRef<str>>(message: S) {
        let m = message.as_ref();

        log::warn!("{m}");

        #[cfg(debug_assertions)]
        println!("[WARN] {m}");
    }

    pub fn info<S: AsRef<str>>(message: S) {
        let m = message.as_ref();

        log::info!("{m}");

        #[cfg(debug_assertions)]
        println!("[INFO] {m}");
    }

    pub fn debug<S: AsRef<str>>(message: S) {
        let m = message.as_ref();

        log::debug!("{m}");

        #[cfg(debug_assertions)]
        println!("[DEBUG] {m}");
    }

    pub fn trace<S: AsRef<str>>(message: S) {
        let m = message.as_ref();

        log::trace!("{m}");

        #[cfg(debug_assertions)]
        println!("[TRACE] {m}");
    }
}
