use peertube_api_sdk::apis::configuration::Configuration;
use peertube_api_sdk::apis::video_api::VideoApi;
use peertube_api_sdk::models::ApiV1VideosOwnershipIdAcceptPostIdParameter;

#[tokio::main]
async fn main() {
    let video_api = VideoApi::new(Configuration {
        base_path: "https://peertube.orderi.co".to_string(),
        ..Default::default()
    });

    let id = ApiV1VideosOwnershipIdAcceptPostIdParameter::Id(653);
    let response = video_api.get_video(id, None).await;
    match response {
        Ok(response) => println!("{:#?}", response.state),
        Err(err) => println!("{:#?}", err),
    }
}