use crate::constants::{IP_MMDB_LINK};
use crate::mmdb_reader::MmdbReader;
use maxminddb::Reader;
use reqwest::{Client, ClientBuilder};
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub(crate) async fn periodically_refresh_data(
    mmdb_reader: Arc<RwLock<MmdbReader>>,
    mmdb_key: &str,
) {
    if cfg!(test) {
        return;
    }

    let client = ClientBuilder::new()
        .user_agent("Nullnet")
        .timeout(Duration::from_secs(300))
        .build()
        // .handle_err(location!())
        .unwrap_or_default();

    loop {
        fetch_mmdb(&mmdb_reader, mmdb_key, &client).await
        // .unwrap_or_default()
        ;
        tokio::time::sleep(Duration::from_secs(60 * 60 * 24)).await; // 24 hours
    }
}

async fn fetch_mmdb(mmdb_reader: &Arc<RwLock<MmdbReader>>, mmdb_key: &str, client: &Client) {
    if mmdb_key.is_empty() {
        // log::warn!("IP info MMDB key not found (cannot download database)");
    } else {
        // log::info!("Fetching IP info MMDB from remote...");

        let link = format!("{IP_MMDB_LINK}{mmdb_key}");
        let mmdb = client
            .get(link)
            .send()
            .await
            .unwrap()
            // .handle_err(location!())?
            .bytes()
            .await
            .unwrap()
            // .handle_err(location!())?
            .to_vec();
        *mmdb_reader.write()
            .unwrap()
            // .handle_err(location!())?
            =
            MmdbReader::Reader(Reader::from_source(mmdb)
                                   .unwrap()
                               // .handle_err(location!())?
            );

        // log::info!("IP info MMDB updated successfully");
    }

    // Ok(())
}
