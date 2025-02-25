use crate::datastore::entry::DatastoreEntry;
use std::sync::mpsc::Sender;
use crate::datastore::transmitter::datastore_transmitter;

#[derive(Default)]
pub(crate) struct PostgresLogger {
    logger: Option<Sender<DatastoreEntry>>,
    // is_reconnecting: Arc<Mutex<bool>>,
    // unsent_entries: Arc<Mutex<Vec<DatastoreEntry>>>,
    token: String,
}

impl PostgresLogger {
    pub(crate) fn new(postgres_endpoint: bool) -> Self {
        if !postgres_endpoint {
            return Self::default();
        }

        let (sender, receiver) = std::sync::mpsc::channel();

        tokio::spawn(async move {
            datastore_transmitter(receiver).await
        });

        Self {
            logger: Some(sender),
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
                let e = DatastoreEntry::new(record);
                // send log entry to transmitter
                logger
                    .send(e)
                    // .map_err(|e| log::error!("Could not send log entry to transmitter: {e}"))
                    .expect("Could not send log entry to transmitter");
            }
        }
    }

    fn flush(&self) {}
}
