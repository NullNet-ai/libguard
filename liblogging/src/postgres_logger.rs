use postgres::{Client, Config, Error, NoTls};
use serde::Serialize;
use std::env;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub(crate) struct PostgresLogger {
    logger: Option<Arc<Mutex<Client>>>,
    is_reconnecting: Arc<Mutex<bool>>,
    unsent_entries: Arc<Mutex<Vec<PostgresEntry>>>,
}

impl PostgresLogger {
    pub(crate) fn new(postgres_endpoint: bool) -> Self {
        if !postgres_endpoint {
            return Self::default();
        }

        let mut logger = CONFIG
            .connect(NoTls)
            .expect("could not connect to postgres");

        // create postgres table if it doesn't exist
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {} (
                id SERIAL PRIMARY KEY,
                timestamp TIMESTAMPTZ NOT NULL,
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
            ..Self::default()
        }
    }

    pub(crate) fn reconnect(&self, err: &Error) {
        let Some(logger) = self.logger.clone() else {
            return;
        };
        *self.is_reconnecting.lock().unwrap() = true;
        log::error!("Could not log to postgres: {err}");
        let is_reconnecting = self.is_reconnecting.clone();
        std::thread::spawn(move || loop {
            std::thread::sleep(std::time::Duration::from_secs(10));
            match CONFIG.connect(NoTls) {
                Ok(client) => {
                    *logger.lock().unwrap() = client;
                    *is_reconnecting.lock().unwrap() = false;
                    return;
                }
                Err(e) => {
                    log::error!("Could not reconnect to postgres: {e}");
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
                let e = PostgresEntry::new(record);
                // send query to postgres
                let query = format!(
                    "INSERT INTO {} (timestamp, level, message) VALUES ($1, $2, $3)",
                    POSTGRES_TABLE_NAME.as_str()
                );
                let result = logger
                    .lock()
                    .unwrap()
                    .execute(query.as_str(), &[&e.timestamp, &e.level, &e.message]);
                if let Err(err) = result {
                    self.unsent_entries.lock().unwrap().push(e);
                    if !*self.is_reconnecting.lock().unwrap() {
                        self.reconnect(&err);
                    }
                } else {
                    self.flush();
                };
            }
        }
    }

    fn flush(&self) {
        if let Some(logger) = self.logger.as_ref() {
            let query = format!(
                "INSERT INTO {} (timestamp, level, message) VALUES ($1, $2, $3)",
                POSTGRES_TABLE_NAME.as_str()
            );
            let Ok(stmt) = logger.lock().unwrap().prepare(query.as_str()) else {
                return;
            };
            // use a batch insert instead of this
            let mut unsent_entries = self.unsent_entries.lock().unwrap();
            for e in unsent_entries.iter() {
                if logger
                    .lock()
                    .unwrap()
                    .execute(&stmt, &[&e.timestamp, &e.level, &e.message])
                    .is_err()
                {
                    return;
                }
            }
            unsent_entries.clear();
        }
    }
}

#[derive(Serialize)]
pub(crate) struct PostgresEntry {
    timestamp: String,
    level: String,
    message: String,
}

impl PostgresEntry {
    fn new(record: &log::Record) -> Self {
        let timestamp = chrono::Utc::now().to_rfc3339();
        let level = record.level().to_string();
        let message = record.args().to_string();
        Self {
            timestamp,
            level,
            message,
        }
    }
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
