use crate::datastore::entry::DatastoreEntry;
use crate::datastore::transmitter::DatastoreTransmitter;
use crate::DatastoreCredentials;
use std::sync::mpsc::Sender;

#[derive(Default)]
pub(crate) struct DatastoreLogger {
    logger: Option<Sender<DatastoreEntry>>,
}

impl DatastoreLogger {
    pub(crate) fn new(datastore_credentials: Option<DatastoreCredentials>) -> Self {
        let Some(credentials) = datastore_credentials else {
            return Self::default();
        };

        let (sender, receiver) = std::sync::mpsc::channel();

        tokio::spawn(async move {
            let transmitter = DatastoreTransmitter::new(credentials);
            transmitter.transmit(receiver).await;
        });

        Self {
            logger: Some(sender),
        }
    }

    // not needed with the current libdatastore implementation
    // pub(crate) fn reconnect(&self, err: &Error) {
    //     let Some(logger) = self.logger.clone() else {
    //         return;
    //     };
    //     *self.is_reconnecting.lock().unwrap() = true;
    //     log::error!("Could not log to datastore: {err}");
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
    //                 log::error!("Could not reconnect to datastore: {e}");
    //             }
    //         }
    //     });
    // }
}

impl log::Log for DatastoreLogger {
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
                    .expect("Could not send log entry to transmitter");
            }
        }
    }

    fn flush(&self) {}
}
