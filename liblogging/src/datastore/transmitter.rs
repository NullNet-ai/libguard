use crate::datastore::credentials::DatastoreCredentials;
use crate::datastore::entry::DatastoreEntry;
use crate::datastore::wrapper::DatastoreWrapper;
use std::sync::mpsc::Receiver;

pub(crate) struct DatastoreTransmitter {
    datastore: DatastoreWrapper,
    unsent_entries: Vec<DatastoreEntry>,
}

impl DatastoreTransmitter {
    pub(crate) fn new(datastore_credentials: DatastoreCredentials) -> Self {
        let datastore = DatastoreWrapper::new(datastore_credentials);
        Self {
            datastore,
            unsent_entries: Vec::new(),
        }
    }

    pub(crate) async fn transmit(mut self, receiver: Receiver<DatastoreEntry>) {
        while let Ok(e) = receiver.recv() {
            if self.datastore.logs_insert_single(e.clone()).await.is_err() {
                self.unsent_entries.push(e);
            } else {
                self.flush().await;
            }
        }
    }

    async fn flush(&mut self) {
        if self.unsent_entries.is_empty() {
            return;
        }
        // send log entries batch to datastore
        if self
            .datastore
            .logs_insert_batch(self.unsent_entries.clone())
            .await
            .is_ok()
        {
            self.unsent_entries.clear();
        }
    }
}
