use maxminddb::Reader;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Default)]
pub enum MmdbReader {
    #[default]
    Empty,
    Reader(Reader<Vec<u8>>),
}

impl MmdbReader {
    pub fn lookup<'de, T: Deserialize<'de>>(&'de self, ip: IpAddr) -> Option<T> {
        match self {
            MmdbReader::Reader(reader) => reader.lookup(ip).unwrap(),
            MmdbReader::Empty => None,
        }
    }
}
