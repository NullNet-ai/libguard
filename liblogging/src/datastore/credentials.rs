#[derive(Debug)]
/// Datastore credentials
pub struct DatastoreCredentials {
    pub(crate) app_id: String,
    pub(crate) app_secret: String,
}

impl DatastoreCredentials {
    /// Create new `DatastoreCredentials`
    pub fn new<S: Into<String>>(app_id: S, app_secret: S) -> Self {
        Self {
            app_id: app_id.into(),
            app_secret: app_secret.into(),
        }
    }
}
