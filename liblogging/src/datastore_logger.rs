use crate::datastore::auth::GrpcInterface;
use crate::datastore::transmitter::DatastoreTransmitter;
use crate::datastore::wrapper::GenericLog;
use chrono::Utc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

#[derive(Default)]
pub(crate) struct DatastoreLogger {
    logger: Option<Sender<GenericLog>>,
}

impl DatastoreLogger {
    pub(crate) fn new(grpc: Option<GrpcInterface>) -> Self {
        let Some(grpc) = grpc else {
            return Self::default();
        };

        let (sender, receiver) = mpsc::channel(10_000);

        tokio::spawn(async move {
            let transmitter = DatastoreTransmitter::new(grpc).await;
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
                let timestamp = Utc::now().to_rfc3339();
                let level = record.level().to_string();
                let message = record.args().to_string();
                let e = GenericLog {
                    timestamp,
                    level,
                    message,
                };
                // send log entry to transmitter
                let _ = logger.try_send(e);
            }
        }
    }

    fn flush(&self) {}
}
