use postgres::{Client, Config, NoTls};
use std::env;
use std::sync::{Arc, Mutex};

pub(crate) struct PostgresLogger {
    logger: Option<Arc<Mutex<Client>>>,
    is_reconnecting: Arc<Mutex<bool>>,
}

impl PostgresLogger {
    pub(crate) fn new(postgres_endpoint: bool) -> Self {
        if postgres_endpoint {
            let mut logger = CONFIG
                .connect(NoTls)
                .expect("could not connect to postgres");

            // create postgres table if it doesn't exist
            let query = format!(
                "CREATE TABLE IF NOT EXISTS {} (
                id SERIAL PRIMARY KEY,
                timestamp TEXT NOT NULL,
                level TEXT NOT NULL,
                message TEXT NOT NULL
            )",
                POSTGRES_TABLE_NAME.as_str()
            );
            logger
                .execute(query.as_str(), &[])
                .expect("could not create logs table in postgres");

            Self {
                logger: Some(Arc::new(Mutex::new(logger))),
                is_reconnecting: Arc::new(Mutex::new(false)),
            }
        } else {
            Self {
                logger: None,
                is_reconnecting: Arc::new(Mutex::new(false)),
            }
        }
    }

    pub(crate) fn reconnect(&self) {
        let Some(logger) = self.logger.clone() else {
            return;
        };
        *self.is_reconnecting.lock().unwrap() = true;
        let is_reconnecting = self.is_reconnecting.clone();
        std::thread::spawn(move || loop {
            match CONFIG.connect(NoTls) {
                Ok(client) => {
                    *logger.lock().unwrap() = client;
                    *is_reconnecting.lock().unwrap() = false;
                    return;
                }
                Err(e) => {
                    log::error!("Could not reconnect to postgres: {e}");
                    std::thread::sleep(std::time::Duration::from_secs(10));
                }
            }
        });
    }
}

impl log::Log for PostgresLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.logger
            .as_ref()
            .is_some_and(|_| metadata.level() <= log::max_level())
    }

    fn log(&self, record: &log::Record) {
        if let Some(logger) = self.logger.as_ref() {
            if self.enabled(record.metadata()) {
                let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true);
                let level = record.level().as_str();
                let message = record.args().to_string();
                // send query to postgres
                let query = format!(
                    "INSERT INTO {} (timestamp, level, message) VALUES ($1, $2, $3)",
                    POSTGRES_TABLE_NAME.as_str()
                );
                if logger
                    .lock()
                    .unwrap()
                    .execute(query.as_str(), &[&now, &level, &message])
                    .is_err_and(|_| !*self.is_reconnecting.lock().unwrap())
                {
                    log::error!("Could not log to postgres, trying to reconnect...");
                    self.reconnect();
                };
            }
        }
    }

    fn flush(&self) {}
}

// -------------------------------------------------------------------------------------------------

static CONFIG: once_cell::sync::Lazy<Config> = once_cell::sync::Lazy::new(|| {
    let mut config = Config::new();
    config
        .user(POSTGRES_USER.as_str())
        .password(POSTGRES_PASSWORD.as_str())
        .dbname(POSTGRES_DB_NAME.as_str())
        .host(POSTGRES_HOST.as_str())
        .port(*POSTGRES_PORT);
    config
});

static POSTGRES_USER: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| env::var("POSTGRES_USER").unwrap_or(String::from("postgres")));

static POSTGRES_PASSWORD: once_cell::sync::Lazy<String> = once_cell::sync::Lazy::new(|| {
    env::var("POSTGRES_PASSWORD").unwrap_or(String::from("postgres"))
});

static POSTGRES_DB_NAME: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| env::var("POSTGRES_DB_NAME").unwrap_or(String::from("postgres")));

static POSTGRES_HOST: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| env::var("POSTGRES_HOST").unwrap_or(String::from("localhost")));

static POSTGRES_PORT: once_cell::sync::Lazy<u16> = once_cell::sync::Lazy::new(|| {
    env::var("POSTGRES_PORT")
        .unwrap_or(String::from("5432"))
        .parse::<u16>()
        .unwrap_or(5432)
});

static POSTGRES_TABLE_NAME: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| env::var("POSTGRES_TABLE_NAME").unwrap_or(String::from("logs")));
