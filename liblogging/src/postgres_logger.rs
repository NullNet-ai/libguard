use crate::datastore::DatastoreWrapper;
use futures::executor::block_on;
use serde::Serialize;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub(crate) struct PostgresLogger {
    logger: Option<DatastoreWrapper>,
    // is_reconnecting: Arc<Mutex<bool>>,
    unsent_entries: Arc<Mutex<Vec<PostgresEntry>>>,
    token: String,
}

impl PostgresLogger {
    pub(crate) fn new(postgres_endpoint: bool) -> Self {
        if !postgres_endpoint {
            return Self::default();
        }

        Self {
            logger: Some(DatastoreWrapper::new()),
            ..Self::default()
        }
    }

    // not needed with the current libdatastore implementation
    // pub(crate) fn reconnect(&self, err: &Error) {
    //     let Some(logger) = self.logger.clone() else {
    //         return;
    //     };
    //     *self.is_reconnecting.lock().unwrap() = true;
    //     log::error!("Could not log to postgres: {err}");
    //     let is_reconnecting = self.is_reconnecting.clone();
    //     std::thread::spawn(move || loop {
    //         std::thread::sleep(std::time::Duration::from_secs(10));
    //         match CONFIG.connect(NoTls) {
    //             Ok(client) => {
    //                 logger = client;
    //                 *is_reconnecting.lock().unwrap() = false;
    //                 return;
    //             }
    //             Err(e) => {
    //                 log::error!("Could not reconnect to postgres: {e}");
    //             }
    //         }
    //     });
    // }
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
                // send single log entry to datastore
                if let Err(_) = block_on(logger.logs_insert_single(&self.token, e.clone())) {
                    // log::error!("Could not log to Datastore: {err}");
                    self.unsent_entries.lock().unwrap().push(e);
                } else {
                    self.flush();
                }
            }
        }
    }

    fn flush(&self) {
        if let Some(logger) = self.logger.as_ref() {
            let mut unsent_entries = self.unsent_entries.lock().unwrap();
            if unsent_entries.is_empty() {
                return;
            }
            // send log entries batch to datastore
            if let Err(_) = block_on(logger.logs_insert_batch(&self.token, unsent_entries.clone()))
            {
                // log::error!("Could not log to Datastore: {err}");
            } else {
                unsent_entries.clear();
            }
        }
    }
}

#[derive(Serialize, Clone)]
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
