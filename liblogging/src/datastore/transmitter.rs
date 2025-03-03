use crate::datastore::credentials::DatastoreConfig;
use crate::datastore::wrapper::ServerWrapper;
use libwallguard::Log;
use tokio::sync::mpsc::Receiver;

pub(crate) struct DatastoreTransmitter {
    server: ServerWrapper,
    unsent_entries: Vec<Log>,
}

impl DatastoreTransmitter {
    pub(crate) async fn new(datastore_credentials: DatastoreConfig) -> Self {
        let datastore = ServerWrapper::new(datastore_credentials).await;
        Self {
            server: datastore,
            unsent_entries: Vec::new(),
        }
    }

    pub(crate) async fn transmit(mut self, mut receiver: Receiver<Log>) {
        loop {
            if receiver.recv_many(&mut self.unsent_entries, 10_000).await == 0 {
                // channel closed
                return;
            }

            // loop until server returns error
            loop {
                let insert_ok = if self.unsent_entries.is_empty() {
                    // channel closed
                    return;
                } else {
                    self.server
                        .logs_insert(self.unsent_entries.clone())
                        .await
                        .is_ok()
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
