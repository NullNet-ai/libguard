// <https://www.atlassian.com/data/sql/how-to-start-a-postgresql-server-on-mac-os-x>

use postgres::{Client, Config, NoTls};
use std::env;
use std::sync::Mutex;

pub(crate) struct PostgresLogger {
    logger: Option<Mutex<Client>>,
    table_name: String,
}

impl PostgresLogger {
    pub(crate) fn new(postgres_endpoint: bool) -> Self {
        if postgres_endpoint {
            let user = env::var("POSTGRES_USER").unwrap_or(String::from("postgres"));
            let password = env::var("POSTGRES_PASSWORD").unwrap_or(String::from("postgres"));
            let db_name = env::var("POSTGRES_DB_NAME").unwrap_or(String::from("postgres"));
            let host = env::var("POSTGRES_HOST").unwrap_or(String::from("localhost"));
            let port = env::var("POSTGRES_PORT")
                .unwrap_or(String::from("5432"))
                .parse::<u16>()
                .unwrap_or(5432);
            let table_name = env::var("POSTGRES_TABLE_NAME").unwrap_or(String::from("logs"));
            let mut config = Config::new();
            config
                .user(user.as_str())
                .password(password.as_str())
                .dbname(db_name.as_str())
                .host(host.as_str())
                .port(port);
            // .hostaddr(IpAddr::from);
            let mut logger = config
                .connect(NoTls)
                .expect("could not connect to postgres");

            // delete table
            // let query = format!("DROP TABLE IF EXISTS {}", postgres_endpoint.table_name);
            // logger
            //     .execute(query.as_str(), &[])
            //     .expect("could not delete logs table in postgres");

            // create postgres table if it doesn't exist
            let query = format!(
                "CREATE TABLE IF NOT EXISTS {table_name} (
                id SERIAL PRIMARY KEY,
                timestamp TEXT NOT NULL,
                level TEXT NOT NULL,
                message TEXT NOT NULL
            )"
            );
            logger
                .execute(query.as_str(), &[])
                .expect("could not create logs table in postgres");

            Self {
                logger: Some(Mutex::new(logger)),
                table_name,
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
                let query = format!(
                    "INSERT INTO {} (timestamp, level, message) VALUES ($1, $2, $3)",
                    self.table_name
                );
                logger
                    .lock()
                    .unwrap()
                    .execute(query.as_str(), &[&now, &level, &message])
                    .expect("could not insert log into postgres");
            }
        }
    }

    fn flush(&self) {}
}
