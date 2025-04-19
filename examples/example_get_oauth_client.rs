use peertube_api_sdk::apis::configuration::Configuration;
use peertube_api_sdk::apis::session_api::SessionApi;

#[tokio::main]
async fn main() {
    let session_api = SessionApi::new(Configuration {
        base_path: "https://peertube.orderi.co".to_string(),
        ..Default::default()
    });

    let response = session_api
        .get_o_auth_client()
        .await;
    match response {
        Ok(response) => {
            println!("{:#?}", response.client_id);
            println!("{:#?}", response.client_secret);
        }
        Err(err) => println!("{:#?}", err),
    }
}