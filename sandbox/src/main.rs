use nullnet_libdatastore::{DatastoreClient, DatastoreConfig, LoginBody, LoginData, LoginRequest};

#[tokio::main]
async fn main() {
    let config = DatastoreConfig::new(String::from("192.168.2.19"), 6000, false);

    let client = DatastoreClient::new(config);

    let login_request = tonic::Request::new(LoginRequest {
        body: Some(LoginBody {
            data: Some(LoginData {
                account_id: String::from("device_Hello@gmail.com"),
                account_secret: String::from("12341234"),
            }),
        }),
    });

    let login_response = client.login(login_request).await.unwrap();

    println!("Token: {login_response:?}");
}
