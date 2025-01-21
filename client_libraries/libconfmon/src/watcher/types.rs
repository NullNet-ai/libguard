/// Represents the data of a single file.
///
/// This struct encapsulates the filename and its content as a byte vector.
pub struct FileData {
    pub filename: String,
    pub content: Vec<u8>,
}

/// A snapshot of files and their contents.
///
/// This type alias defines a snapshot as a vector of `FileData`,
/// where each `FileData` represents a single file's data.
pub type Snapshot = Vec<FileData>;
