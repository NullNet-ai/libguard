use crate::datastore::entry::DatastoreEntry;
use crate::datastore::wrapper::DatastoreWrapper;
use std::sync::mpsc::Receiver;

struct DatastoreTransmitter {
    datastore: DatastoreWrapper,
    token: String,
    unsent_entries: Vec<DatastoreEntry>,
}

impl DatastoreTransmitter {
    fn new() -> Self {
        let datastore = DatastoreWrapper::new();
        let token = "".to_string();
        Self {
            datastore,
            token,
            unsent_entries: Vec::new(),
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

pub(crate) async fn datastore_transmitter(receiver: Receiver<DatastoreEntry>) {
    let mut transmitter = DatastoreTransmitter::new();
    while let Ok(e) = receiver.recv() {
        if let Err(_) = transmitter
            .datastore
            .logs_insert_single(&transmitter.token, e.clone())
            .await
        {
            // log::error!("Could not log to Datastore: {err}");
            transmitter.unsent_entries.push(e);
        } else {
            transmitter.flush().await;
        }
    }
}
