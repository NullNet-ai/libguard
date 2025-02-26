use crate::datastore::entry::DatastoreEntry;
use crate::datastore::wrapper::DatastoreWrapper;
use std::sync::mpsc::Receiver;

pub(crate) struct DatastoreTransmitter {
    datastore: DatastoreWrapper,
    token: String,
    unsent_entries: Vec<DatastoreEntry>,
}

impl DatastoreTransmitter {
    pub(crate) fn new() -> Self {
        let datastore = DatastoreWrapper::new();
        let token = "".to_string();
        Self {
            datastore,
            token,
            unsent_entries: Vec::new(),
        }
    }

    pub(crate) async fn transmit(mut self, receiver: Receiver<DatastoreEntry>) {
        while let Ok(e) = receiver.recv() {
            if let Err(_) = self
                .datastore
                .logs_insert_single(&self.token, e.clone())
                .await
            {
                // log::error!("Could not log to Datastore: {err}");
                self.unsent_entries.push(e);
            } else {
                self.flush().await;
            }
        }
    }

    async fn flush(&mut self) {
        let unsent_entries = &mut self.unsent_entries;
        if unsent_entries.is_empty() {
            return;
        }
        // send log entries batch to datastore
        if let Err(_) = self
            .datastore
            .logs_insert_batch(&self.token, unsent_entries.clone())
            .await
        {
            // log::error!("Could not log to Datastore: {err}");
        } else {
            unsent_entries.clear();
        }
    }
}
