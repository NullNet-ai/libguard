use crate::mmdb::mmdb_reader::MmdbReader;
use crate::web_client::new_web_client;
use maxminddb::Reader;
use reqwest::Client;
use std::io::Read;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub(crate) async fn refresh_mmdb_data(
    mmdb_reader: Arc<RwLock<MmdbReader>>,
    url: &str,
    refresh_days: u64,
) {
    let client = new_web_client();

    loop {
        fetch_mmdb(&mmdb_reader, url, &client).await
        // .unwrap_or_default()
        ;
        tokio::time::sleep(Duration::from_secs(refresh_days * 60 * 60 * 24)).await;
    }
}

async fn fetch_mmdb(mmdb_reader: &Arc<RwLock<MmdbReader>>, url: &str, client: &Client) {
    // log::info!("Fetching IP info MMDB from remote...");

    let zipped_bytes = client.get(url).send().await.unwrap().bytes().await.unwrap();

    let mmdb: Vec<u8> = flate2::read::GzDecoder::new(&zipped_bytes[..])
        .bytes()
        .flatten()
        .collect();

    *mmdb_reader.write().unwrap() = MmdbReader::Reader(
        Reader::from_source(mmdb).unwrap(), // .handle_err(location!())?
    );

    // log::info!("IP info MMDB updated successfully");

    // Ok(())
}
