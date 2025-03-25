use crate::mmdb::mmdb_reader::MmdbReader;
use crate::web_client::new_web_client;
use maxminddb::Reader;
use nullnet_liberror::{Error, ErrorHandler, Location, location};
use reqwest::Client;
use std::fmt::Write;
use std::io::Read;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub(crate) async fn refresh_mmdb_data(
    mmdb_reader: Arc<RwLock<MmdbReader>>,
    url: &str,
    refresh_days: u64,
) {
    let client = new_web_client().unwrap_or_default();

    loop {
        let url = format_url_with_date(url).unwrap_or(url.to_string());
        fetch_mmdb(&mmdb_reader, &url, &client)
            .await
            .unwrap_or_default();
        tokio::time::sleep(Duration::from_secs(refresh_days * 60 * 60 * 24)).await;
    }
}

async fn fetch_mmdb(
    mmdb_reader: &Arc<RwLock<MmdbReader>>,
    url: &str,
    client: &Client,
) -> Result<(), Error> {
    log::info!("Fetching IP info MMDB from remote...");

    let zipped_bytes = client
        .get(url)
        .send()
        .await
        .handle_err(location!())?
        .bytes()
        .await
        .handle_err(location!())?;

    let mmdb: Vec<u8> = flate2::read::GzDecoder::new(&zipped_bytes[..])
        .bytes()
        .flatten()
        .collect();

    *mmdb_reader.write().handle_err(location!())? =
        MmdbReader::Reader(Reader::from_source(mmdb).handle_err(location!())?);

    log::info!("IP info MMDB updated successfully");

    Ok(())
}

// <`https://docs.rs/chrono/0.4.39/chrono/format/strftime/index.html`>
fn format_url_with_date(url: &str) -> Option<String> {
    let start = url.find('{')?;
    let end = url.find('}')?;
    if start >= end {
        return None;
    }

    let date_format = url.get(start + 1..end).unwrap_or_default();
    let mut date = String::new();
    write!(date, "{}", chrono::Utc::now().format(date_format)).ok()?;
    Some(url.replace(&url[start..=end], &date))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_url_with_date() {
        let url = "https://download.db-ip.com/free/dbip-city-lite-{%Y-%m}.mmdb.gz";
        let formatted_url = format_url_with_date(url);
        let year_month = chrono::Utc::now().format("%Y-%m").to_string();
        assert_eq!(
            formatted_url.unwrap(),
            format!("https://download.db-ip.com/free/dbip-city-lite-{year_month}.mmdb.gz")
        );
    }

    #[test]
    fn test_format_url_with_date_wrong() {
        let url = "https://download.db-ip.com/free/dbip-city-lite-{xyz}.mmdb.gz";
        let formatted_url = format_url_with_date(url);
        assert_eq!(
            formatted_url.unwrap(),
            "https://download.db-ip.com/free/dbip-city-lite-xyz.mmdb.gz".to_string()
        );
    }

    #[test]
    fn test_format_url_without_date() {
        let url = "https://download.db-ip.com/free/dbip-city-lite-{.mmdb.gz";
        let _ = format_url_with_date(url).is_none();
        let url = "https://download.db-ip.com/free/dbip-city-lite-}.mmdb.gz";
        let _ = format_url_with_date(url).is_none();
        let url = "https://download.db-ip.com/free/dbip-city-lite.mmdb.gz";
        let _ = format_url_with_date(url).is_none();
        let url = "https://download.db-ip.com/free/dbip-city-lite-}ciao{.mmdb.gz";
        let _ = format_url_with_date(url).is_none();
    }
}
