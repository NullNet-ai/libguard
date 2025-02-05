use maxminddb::{MaxMindDBError, Reader};
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Default)]
pub enum MmdbReader {
    #[default]
    Empty,
    Reader(Reader<Vec<u8>>),
}

impl MmdbReader {
    pub fn lookup<'de, T: Default + Deserialize<'de>>(
        &'de self,
        ip: IpAddr,
    ) -> Result<T, MaxMindDBError> {
        match self {
            MmdbReader::Reader(reader) => reader.lookup(ip),
            MmdbReader::Empty => Ok(T::default()),
        }
    }
}
