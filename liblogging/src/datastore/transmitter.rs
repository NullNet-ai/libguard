use crate::datastore::credentials::DatastoreCredentials;
use crate::datastore::entry::DatastoreEntry;
use crate::datastore::wrapper::DatastoreWrapper;
use tokio::sync::mpsc::Receiver;

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

    pub(crate) async fn transmit(mut self, mut receiver: Receiver<DatastoreEntry>) {
        loop {
            if receiver.recv_many(&mut self.unsent_entries, 10_000).await == 0 {
                // channel closed
                return;
            }

            // loop until datastore returns error
            loop {
                let insert_ok = match self.unsent_entries.as_slice() {
                    // channel closed
                    [] => return,
                    // received single log entry
                    [e] => self.datastore.logs_insert_single(e.clone()).await.is_ok(),
                    // received multiple log entries, or buffer accumulated multiple entries due to errors
                    _ => self
                        .datastore
                        .logs_insert_batch(self.unsent_entries.clone())
                        .await
                        .is_ok(),
                };

                if insert_ok {
                    // println!("Inserted {} log entries", self.unsent_entries.len());
                    self.unsent_entries.clear();
                    break;
                }

                // println!("Insertion failed");
                // wait 10 seconds before retrying
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
        }
    }
}
