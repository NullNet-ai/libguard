use std::fmt::{Display, Formatter, Result};

/// Represents the different kinds of errors that can occur during configuration monitoring.
#[derive(Debug)]
pub enum ErrorKind {
    ErrorInitializingWatcher,
    ErrorWatchingFile,
    ErrorReadingFile,
    ErrorHandlingSnapshot,
    ErrorUnsupportedPlatform,
}

impl Display for ErrorKind {
    /// Formats the `ErrorKind` for display.
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ErrorKind::ErrorInitializingWatcher => write!(f, "ErrorInitializingWatcher"),
            ErrorKind::ErrorWatchingFile => write!(f, "ErrorWatchingFile"),
            ErrorKind::ErrorReadingFile => write!(f, "ErrorReadingFile"),
            ErrorKind::ErrorHandlingSnapshot => write!(f, "ErrorHandlingSnapshot"),
            ErrorKind::ErrorUnsupportedPlatform => write!(f, "ErrorUnsupportedPlatform"),
        }
    }
}

/// A structured error type for `libconfmon`.
///
/// # Fields
/// - `kind`: The specific type of error.
/// - `message`: A detailed message explaining the error.
#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
}

impl Display for Error {
    /// Formats the `Error` for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.kind, self.message)
    }
}
