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
}

impl log::Log for DatastoreLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        self.logger.as_ref().is_some_and(|_| {
            metadata.level() <= log::max_level()
                && !metadata.target().starts_with("nullnet_libdatastore")
                && !metadata.target().starts_with("nullnet_libtoken")
        })
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
