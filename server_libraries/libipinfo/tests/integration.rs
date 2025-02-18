use nullnet_libipinfo::{ApiFields, IpInfo, IpInfoHandler, IpInfoProvider};

#[tokio::test]
async fn with_providers() {
    let handler = IpInfoHandler::new(vec![
        IpInfoProvider::new_api_provider(
            "https://ipapi.co/{ip}/json",
            "",
            ApiFields {
                country: Some("/country"),
                asn: Some("/asn"),
                org: Some("/org"),
                continent_code: Some("/continent_code"),
                city: Some("/city"),
                region: Some("/region"),
                postal: Some("/postal"),
                timezone: Some("/timezone"),
            },
        ),
        IpInfoProvider::new_mmdb_provider(
            "https://download.db-ip.com/free/dbip-city-lite-{%Y-%m}.mmdb.gz",
            "https://download.db-ip.com/free/dbip-asn-lite-{%Y-%m}.mmdb.gz",
            "",
            31,
        ),
    ])
    .unwrap();

    let res = handler.lookup("8.8.8.8").await.unwrap();

    assert_eq!(
        res,
        IpInfo {
            country: Some("US".to_string()),
            asn: Some("AS15169".to_string()),
            org: Some("GOOGLE".to_string()),
            continent_code: Some("NA".to_string()),
            city: Some("Mountain View".to_string()),
            region: Some("California".to_string()),
            postal: Some("94043".to_string()),
            timezone: Some("America/Los_Angeles".to_string())
        }
    );
}

// #[tokio::test]
// async fn without_providers() {
//     let handler = IpInfoHandler::new(vec![]);
//
//     // wait for handler to download fallback databases
//     tokio::time::sleep(std::time::Duration::from_secs(100)).await;
//
//     let res = handler.lookup("8.8.8.8").await.unwrap();
//
//     assert_eq!(
//         res,
//         IpInfo {
//             country: Some("US".to_string()),
//             asn: Some("15169".to_string()),
//             org: Some("Google LLC".to_string()),
//             continent_code: Some("NA".to_string()),
//             city: Some("Mountain View".to_string()),
//             region: Some("California".to_string()),
//             postal: None,
//             timezone: None
//         }
//     );
// }
