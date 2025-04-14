use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

pub mod abuses_api;
pub mod account_blocks_api;
pub mod accounts_api;
pub mod automatic_tags_api;
pub mod channels_sync_api;
pub mod config_api;
pub mod homepage_api;
pub mod instance_follows_api;
pub mod instance_redundancy_api;
pub mod job_api;
pub mod live_videos_api;
pub mod logs_api;
pub mod my_history_api;
pub mod my_notifications_api;
pub mod my_subscriptions_api;
pub mod my_user_api;
pub mod plugins_api;
pub mod register_api;
pub mod runner_jobs_api;
pub mod runner_registration_token_api;
pub mod runners_api;
pub mod search_api;
pub mod server_blocks_api;
pub mod session_api;
pub mod static_video_files_api;
pub mod stats_api;
pub mod user_exports_api;
pub mod user_imports_api;
pub mod users_api;
pub mod video_api;
pub mod video_blocks_api;
pub mod video_captions_api;
pub mod video_channels_api;
pub mod video_chapters_api;
pub mod video_comments_api;
pub mod video_download_api;
pub mod video_feeds_api;
pub mod video_files_api;
pub mod video_imports_api;
pub mod video_mirroring_api;
pub mod video_ownership_change_api;
pub mod video_passwords_api;
pub mod video_playlists_api;
pub mod video_rates_api;
pub mod video_stats_api;
pub mod video_transcoding_api;
pub mod video_upload_api;
pub mod videos_api;
pub mod watched_words_api;

pub mod configuration;
