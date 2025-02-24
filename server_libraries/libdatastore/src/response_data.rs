pub struct ResponseData {
    /// The number of records returned in the response.
    pub count: i32,
    /// A JSON-encoded array containing the records.
    pub data: String,
    /// The encoding format of the data.
    pub encoding: String,
}
