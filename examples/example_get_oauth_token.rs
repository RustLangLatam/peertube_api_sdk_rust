use peertube_api_sdk::apis::configuration::Configuration;
use peertube_api_sdk::apis::session_api::SessionApi;

#[tokio::main]
async fn main() {
    let session_api = SessionApi::new(Configuration {
        base_path: "https://peertube.orderi.co".to_string(),
        ..Default::default()
    });

    let client_id = "5elw5exk0l91h6e3hhff772fnxurv9y4";
    let client_secret = "u4IzSaVgYk7uPxyQzN0F9YESCcgt6UlJ";
    let grant_type = "password";
    let username = "root";
    let password = "1ncC1hrpB9KD@peer";

    let response = session_api
        .get_o_auth_token(
            Some(client_id),
            Some(client_secret),
            Some(grant_type),
            Some(username),
            Some(password),
            None,
        )
        .await;
    match response {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(err) => println!("{:#?}", err),
    }
}