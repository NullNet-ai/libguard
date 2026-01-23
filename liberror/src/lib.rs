use std::fmt::Display;

#[derive(Debug)]
/// General error type for Nullnet
pub struct Error {
    message: String,
}

impl Error {
    #[must_use]
    /// Returns the error message
    pub fn to_str(&self) -> &str {
        &self.message
    }
}

/// Trait for logging and handling errors in a unified way
pub trait ErrorHandler<T, E> {
    /// Handle the error and log its location
    #[allow(clippy::missing_errors_doc)]
    fn handle_err(self, loc: Location) -> Result<T, Error>;
}

impl<T, E: Display> ErrorHandler<T, E> for Result<T, E> {
    fn handle_err(self, location: Location) -> Result<T, Error> {
        self.map_err(|e| {
            // log::error!(target: location.module_path, "[{}:{}] {e}", location.file, location.line);
            println!("[ERROR] [{}:{}] {e}", location.file, location.line);
            Error {
                message: e.to_string(),
            }
        })
    }

    fn handle_err_no_print(self, location: Location) -> Result<T, Error> {
        self.map_err(|e| {
            Error {
                message: e.to_string(),
            }
        })
    }
}

/// Struct to store the location in the code (module path, file, and line)
pub struct Location {
    pub module_path: &'static str,
    pub file: &'static str,
    pub line: u32,
}

#[macro_export]
/// Macro to get the current location in the code (module path, file, and line)
macro_rules! location {
    () => {
        Location {
            module_path: module_path!(),
            file: file!(),
            line: line!(),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_handler() {
        let err_result: Result<usize, &str> = Err("test_error");
        let err_handled = err_result.handle_err(location!());
        assert_eq!(err_handled.unwrap_err().to_str(), "test_error");

        let ok_result: Result<usize, &str> = Ok(2);
        let ok_handled = ok_result.handle_err(location!());
        assert_eq!(ok_handled.unwrap(), 2);
    }
}
