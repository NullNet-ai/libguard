use postgres::{Client, Config, NoTls};
use std::sync::Mutex;

/// A Postgres endpoints
pub struct PostgresEndpoint {
    user: String,
    password: String,
    db_name: String,
    table_name: String,
    host: String,
    port: u16,
}

impl PostgresEndpoint {
    /// Creates a new Postgres endpoint
    pub fn new(
        host: String,
        port: u16,
        user: String,
        password: String,
        db_name: String,
        table_name: String,
    ) -> Self {
        Self {
            user,
            password,
            db_name,
            table_name,
            host,
            port,
        }
    }
}

pub(crate) struct PostgresLogger {
    logger: Option<Mutex<Client>>,
    table_name: String,
}

impl PostgresLogger {
    pub(crate) fn new(postgres_endpoint: Option<PostgresEndpoint>) -> Self {
        if let Some(postgres_endpoint) = postgres_endpoint {
            let mut config = Config::new();
            config
                .user(&postgres_endpoint.user)
                .password(&postgres_endpoint.password)
                .dbname(&postgres_endpoint.db_name)
                .host(&postgres_endpoint.host)
                .port(postgres_endpoint.port);
            // .hostaddr(IpAddr::from);
            let mut logger = config
                .connect(NoTls)
                .expect("could not connect to postgres");

            // create postgres table if it doesn't exist
            let query = format!(
                "CREATE TABLE IF NOT EXISTS {} (
                id SERIAL PRIMARY KEY,
                timestamp TIMESTAMP NOT NULL,
                level TEXT NOT NULL,
                message TEXT NOT NULL
            )",
                postgres_endpoint.table_name
            );
            logger
                .execute(query.as_str(), &[])
                .expect("could not create logs table in postgres");

            Self {
                logger: Some(Mutex::new(logger)),
                table_name: postgres_endpoint.table_name,
            }
        } else {
            Self {
                logger: None,
                table_name: String::new(),
            }
        }
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
                let mut logger = logger.lock().unwrap();
                let query = format!(
                    "INSERT INTO {} (timestamp, level, message) VALUES ($1, $2, $3)",
                    self.table_name
                );
                logger
                    .execute(query.as_str(), &[&now, &level, &message])
                    .expect("could not insert log into postgres");
            }
        }
    }

    fn flush(&self) {}
}
