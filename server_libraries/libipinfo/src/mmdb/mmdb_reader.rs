use maxminddb::Reader;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Default)]
pub(crate) enum MmdbReader {
    #[default]
    Empty,
    Reader(Reader<Vec<u8>>),
}

impl MmdbReader {
    pub(crate) fn lookup<'de, T: Deserialize<'de>>(&'de self, ip: IpAddr) -> Option<T> {
        match self {
            MmdbReader::Reader(reader) => reader.lookup(ip).and_then(|lr| lr.decode()).ok()?,
            MmdbReader::Empty => None,
        }
    }
}
