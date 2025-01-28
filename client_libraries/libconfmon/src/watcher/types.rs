use std::path::PathBuf;

/// Represents the data of a file, including its name and content.
///
/// # Fields
/// - `filename`: The name of the file.
/// - `content`: The binary content of the file, stored as a vector of bytes.
#[derive(Debug)]
pub struct FileData {
    pub filename: String,
    pub content: Vec<u8>,
}

/// Contains metadata about a file, including its path and modification time.
///
/// # Fields
/// - `path`: The filesystem path to the file.
/// - `mtime`: The modification time of the file, represented as the number of milliseconds
///            since the UNIX epoch.
#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub mtime: u128,
}

/// A snapshot representing a collection of file data.
///
/// This is used to store the current state of multiple files, where each file
/// is represented by its `FileData`.
pub type Snapshot = Vec<FileData>;
