use crate::store_service_client::StoreServiceClient;
use nullnet_liberror::{Error, ErrorHandler, Location, location};
use tonic::transport::{Channel, ClientTlsConfig};

/// Configuration structure for the datastore.
///
/// This struct encapsulates the configuration details required to connect to the datastore,
/// including the host address, port, and whether TLS is enabled.
#[derive(Debug, Clone)]
pub struct DatastoreConfig {
    /// Hostname or IP address of the datastore.
    pub host: String,
    /// Port number used to connect to the datastore.
    pub port: u16,
    /// Whether TLS is enabled for the datastore connection.
    pub tls: bool,
}

impl DatastoreConfig {
    /// Creates a new `DatastoreConfig` instance by reading values from environment variables.
    ///
    /// If the environment variables are not set or cannot be parsed, default values are used:
    /// - `DATASTORE_HOST` defaults to `"127.0.0.1"`.
    /// - `DATASTORE_PORT` defaults to `6000`.
    /// - `DATASTORE_TLS` defaults to `false`.
    ///
    /// # Returns
    /// A `DatastoreConfig` instance with values derived from the environment or defaults.
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            host: read_host_value_from_env(String::from("127.0.0.1")),
            port: read_port_value_from_env(6000),
            tls: real_tls_value_from_env(false),
        }
    }

    /// Creates a new `DatastoreConfig` instance with the provided values.
    ///
    /// # Arguments
    /// * `host` - A `String` representing the hostname or IP address of the datastore.
    /// * `port` - A `u16` representing the port number for connecting to the datastore.
    /// * `tls` - A `bool` indicating whether to use TLS for the connection (`true` for TLS, `false` otherwise).
    ///
    /// # Returns
    /// A `DatastoreConfig` instance initialized with the specified host, port, and TLS settings.
    #[must_use]
    pub fn new(host: String, port: u16, tls: bool) -> Self {
        Self { host, port, tls }
    }

    /// Establishes a connection to the datastore service.
    ///
    /// # Returns
    /// * `Ok(StoreServiceClient<Channel>)` - The client for interacting with the datastore service.
    /// * `Err(Error)` - If the connection fails.
    pub(crate) async fn connect(&self) -> Result<StoreServiceClient<Channel>, Error> {
        let protocol = if self.tls { "https" } else { "http" };
        let host = self.host.as_str();
        let port = self.port;

        let mut endpoint = Channel::from_shared(format!("{protocol}://{host}:{port}"))
            .handle_err(location!())?
            .connect_timeout(std::time::Duration::from_secs(10));

        if self.tls {
            endpoint = endpoint
                .tls_config(ClientTlsConfig::new().with_native_roots())
                .handle_err(location!())?;
        }

        let channel: Channel = endpoint.connect().await.handle_err(location!())?;

        Ok(StoreServiceClient::new(channel).max_decoding_message_size(50 * 1024 * 1024))
    }
}

/// Reads the `DATASTORE_HOST` environment variable.
///
/// If the variable is not set or an error occurs, the specified default value is returned.
///
/// # Parameters
/// - `default`: Default hostname to use if the environment variable is not set.
///
/// # Returns
/// The value of `DATASTORE_HOST` or the default value if the variable is not set.
fn read_host_value_from_env(default: String) -> String {
    std::env::var("DATASTORE_HOST").unwrap_or_else(|err| {
        log::warn!("Failed to read 'DATASTORE_HOST' env var: {err}. Using default value ...");
        default
    })
}

/// Reads the `DATASTORE_PORT` environment variable and parses it as a `u16`.
///
/// If the variable is not set, cannot be parsed, or an error occurs, the specified default value is used.
///
/// # Parameters
/// - `default`: Default port to use if the environment variable is not set or invalid.
///
/// # Returns
/// The value of `DATASTORE_PORT` parsed as a `u16` or the default value if the variable is not set or invalid.
fn read_port_value_from_env(default: u16) -> u16 {
    match std::env::var("DATASTORE_PORT") {
        Ok(value) => value.parse::<u16>().unwrap_or_else(|err| {
            log::warn!(
                "Failed to parse 'DATASTORE_PORT' ({value}) as u16: {err}. Using default value ..."
            );
            default
        }),
        Err(err) => {
            log::warn!("Failed to read 'DATASTORE_PORT' env var: {err}. Using default value ...");
            default
        }
    }
}

/// Reads the `DATASTORE_TLS` environment variable and interprets it as a boolean.
///
/// If the variable is set to `"true"` (case insensitive), it returns `true`. For any other value or
/// if the variable is not set, the specified default value is used.
///
/// # Parameters
/// - `default`: Default value to use if the environment variable is not set.
///
/// # Returns
/// `true` if the environment variable is set to `"true"`, otherwise the default value.
fn real_tls_value_from_env(default: bool) -> bool {
    match std::env::var("DATASTORE_TLS") {
        Ok(value) => value.to_lowercase() == "true",
        Err(err) => {
            log::warn!("Failed to read 'DATASTORE_TLS' env var: {err}. Using default value ...");
            default
        }
    }
}
