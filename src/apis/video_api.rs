/*
 * PeerTube
 *
 * The PeerTube API is built on HTTP(S) and is RESTful. You can use your favorite HTTP/REST library for your programming language to use PeerTube. The spec API is fully compatible with [openapi-generator](https://github.com/OpenAPITools/openapi-generator/wiki/API-client-generator-HOWTO) which generates a client SDK in the language of your choice - we generate some client SDKs automatically:  - [Python](https://framagit.org/framasoft/peertube/clients/python) - [Go](https://framagit.org/framasoft/peertube/clients/go) - [Kotlin](https://framagit.org/framasoft/peertube/clients/kotlin)  See the [REST API quick start](https://docs.joinpeertube.org/api/rest-getting-started) for a few examples of using the PeerTube API.  # Authentication  When you sign up for an account on a PeerTube instance, you are given the possibility to generate sessions on it, and authenticate there using an access token. Only __one access token can currently be used at a time__.  ## Roles  Accounts are given permissions based on their role. There are three roles on PeerTube: Administrator, Moderator, and User. See the [roles guide](https://docs.joinpeertube.org/admin/managing-users#roles) for a detail of their permissions.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call, completed by a [RFC7807-compliant](https://tools.ietf.org/html/rfc7807) response body.  ``` HTTP 1.1 404 Not Found Content-Type: application/problem+json; charset=utf-8  {   \"detail\": \"Video not found\",   \"docs\": \"https://docs.joinpeertube.org/api-rest-reference.html#operation/getVideo\",   \"status\": 404,   \"title\": \"Not Found\",   \"type\": \"about:blank\" } ```  We provide error `type` (following RFC7807) and `code` (internal PeerTube code) values for [a growing number of cases](https://github.com/Chocobozzz/PeerTube/blob/develop/packages/models/src/server/server-error-code.enum.ts), but it is still optional. Types are used to disambiguate errors that bear the same status code and are non-obvious:  ``` HTTP 1.1 403 Forbidden Content-Type: application/problem+json; charset=utf-8  {   \"detail\": \"Cannot get this video regarding follow constraints\",   \"docs\": \"https://docs.joinpeertube.org/api-rest-reference.html#operation/getVideo\",   \"status\": 403,   \"title\": \"Forbidden\",   \"type\": \"https://docs.joinpeertube.org/api-rest-reference.html#section/Errors/does_not_respect_follow_constraints\" } ```  Here a 403 error could otherwise mean that the video is private or blocklisted.  ### Validation errors  Each parameter is evaluated on its own against a set of rules before the route validator proceeds with potential testing involving parameter combinations. Errors coming from validation errors appear earlier and benefit from a more detailed error description:  ``` HTTP 1.1 400 Bad Request Content-Type: application/problem+json; charset=utf-8  {   \"detail\": \"Incorrect request parameters: id\",   \"docs\": \"https://docs.joinpeertube.org/api-rest-reference.html#operation/getVideo\",   \"instance\": \"/api/v1/videos/9c9de5e8-0a1e-484a-b099-e80766180\",   \"invalid-params\": {     \"id\": {       \"location\": \"params\",       \"msg\": \"Invalid value\",       \"param\": \"id\",       \"value\": \"9c9de5e8-0a1e-484a-b099-e80766180\"     }   },   \"status\": 400,   \"title\": \"Bad Request\",   \"type\": \"about:blank\" } ```  Where `id` is the name of the field concerned by the error, within the route definition. `invalid-params.<field>.location` can be either 'params', 'body', 'header', 'query' or 'cookies', and `invalid-params.<field>.value` reports the value that didn't pass validation whose `invalid-params.<field>.msg` is about.  ### Deprecated error fields  Some fields could be included with previous versions. They are still included but their use is deprecated: - `error`: superseded by `detail`  # Rate limits  We are rate-limiting all endpoints of PeerTube's API. Custom values can be set by administrators:  | Endpoint (prefix: `/api/v1`) | Calls         | Time frame   | |------------------------------|---------------|--------------| | `/_*`                         | 50            | 10 seconds   | | `POST /users/token`          | 15            | 5 minutes    | | `POST /users/register`       | 2<sup>*</sup> | 5 minutes    | | `POST /users/ask-send-verify-email` | 3      | 5 minutes    |  Depending on the endpoint, <sup>*</sup>failed requests are not taken into account. A service limit is announced by a `429 Too Many Requests` status code.  You can get details about the current state of your rate limit by reading the following headers:  | Header                  | Description                                                | |-------------------------|------------------------------------------------------------| | `X-RateLimit-Limit`     | Number of max requests allowed in the current time period  | | `X-RateLimit-Remaining` | Number of remaining requests in the current time period    | | `X-RateLimit-Reset`     | Timestamp of end of current time period as UNIX timestamp  | | `Retry-After`           | Seconds to delay after the first `429` is received         |  # CORS  This API features [Cross-Origin Resource Sharing (CORS)](https://fetch.spec.whatwg.org/), allowing cross-domain communication from the browser for some routes:  | Endpoint                    | |------------------------- ---| | `/api/_*`                    | | `/download/_*`               | | `/lazy-static/_*`            | | `/.well-known/webfinger`    |  In addition, all routes serving ActivityPub are CORS-enabled for all origins.
 *
 * The version of the OpenAPI document: 7.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for typed errors of method [`add_live_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddLive0Error {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_view`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddViewError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_videos_id_studio_edit_post_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1VideosIdStudioEditPost0Error {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_videos_id_watching_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1VideosIdWatchingPutError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`del_video`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DelVideoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_video_source_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteVideoSourceFileError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_account_videos_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAccountVideos0Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_categories`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCategoriesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_languages`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLanguagesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_licences`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLicencesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_live_id_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLiveId0Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_video`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVideoError {
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_video_channel_videos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVideoChannelVideosError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_video_desc`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVideoDescError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_video_privacy_policies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVideoPrivacyPoliciesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_video_source`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVideoSourceError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_videos`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVideosError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_video_storyboards`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVideoStoryboardsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`put_video`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutVideoError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_video_source_resumable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceVideoSourceResumableError {
    Status403(),
    Status404(),
    Status409(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_video_source_resumable_cancel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceVideoSourceResumableCancelError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_video_source_resumable_init`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceVideoSourceResumableInitError {
    Status413(),
    Status415(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`request_video_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestVideoTokenError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_videos_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchVideos0Error {
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_live_id_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLiveId0Error {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_legacy`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadLegacyError {
    Status403(),
    Status408(),
    Status413(),
    Status415(),
    Status422(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_resumable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadResumableError {
    Status403(),
    Status404(),
    Status409(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_resumable_cancel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadResumableCancelError {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_resumable_init`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadResumableInitError {
    Status413(),
    Status415(),
    UnknownValue(serde_json::Value),
}

pub struct VideoApi {
    configuration: Box<configuration::Configuration>,
}

impl VideoApi {
    pub fn new(configuration: configuration::Configuration) -> VideoApi {
        VideoApi {
            configuration: Box::new(configuration),
        }
    }

    pub async fn add_live_0(
        &self,
        channel_id: i32,
        name: &str,
        save_replay: Option<bool>,
        replay_settings: Option<models::LiveVideoReplaySettings>,
        permanent_live: Option<bool>,
        latency_mode: Option<models::LiveVideoLatencyMode>,
        thumbnailfile: Option<std::path::PathBuf>,
        previewfile: Option<std::path::PathBuf>,
        privacy: Option<models::VideoPrivacySet>,
        category: Option<i32>,
        licence: Option<i32>,
        language: Option<&str>,
        description: Option<&str>,
        support: Option<&str>,
        nsfw: Option<bool>,
        tags: Option<Vec<String>>,
        comments_enabled: Option<bool>,
        comments_policy: Option<models::VideoCommentsPolicySet>,
        download_enabled: Option<bool>,
    ) -> Result<models::VideoUploadResponse, Error<AddLive0Error>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/videos/live", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("channelId", channel_id.to_string());
        if let Some(local_var_param_value) = save_replay {
            local_var_form = local_var_form.text("saveReplay", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = replay_settings {
            local_var_form =
                local_var_form.text("replaySettings", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = permanent_live {
            local_var_form =
                local_var_form.text("permanentLive", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = latency_mode {
            local_var_form = local_var_form.text("latencyMode", local_var_param_value.to_string());
        }
        // TODO: support file upload for 'thumbnailfile' parameter
        // TODO: support file upload for 'previewfile' parameter
        if let Some(local_var_param_value) = privacy {
            local_var_form = local_var_form.text("privacy", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = category {
            local_var_form = local_var_form.text("category", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = licence {
            local_var_form = local_var_form.text("licence", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = language {
            local_var_form = local_var_form.text("language", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = description {
            local_var_form = local_var_form.text("description", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = support {
            local_var_form = local_var_form.text("support", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = nsfw {
            local_var_form = local_var_form.text("nsfw", local_var_param_value.to_string());
        }
        local_var_form = local_var_form.text("name", name.to_string());
        if let Some(local_var_param_value) = tags {
            local_var_form = local_var_form.text(
                "tags",
                local_var_param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            );
        }
        if let Some(local_var_param_value) = comments_enabled {
            local_var_form =
                local_var_form.text("commentsEnabled", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = comments_policy {
            local_var_form =
                local_var_form.text("commentsPolicy", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = download_enabled {
            local_var_form =
                local_var_form.text("downloadEnabled", local_var_param_value.to_string());
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<AddLive0Error> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Call this endpoint regularly (every 5-10 seconds for example) to notify the server the user is watching the video. After a while, PeerTube will increase video's viewers counter. If the user is authenticated, PeerTube will also store the current player time.
    pub async fn add_view(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
        user_viewing_video: models::UserViewingVideo,
    ) -> Result<(), Error<AddViewError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/views",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.json(&user_viewing_video);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<AddViewError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Create a task to edit a video  (cut, add intro/outro etc)
    pub async fn api_v1_videos_id_studio_edit_post_0(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
    ) -> Result<(), Error<ApiV1VideosIdStudioEditPost0Error>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/studio/edit",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<ApiV1VideosIdStudioEditPost0Error> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This endpoint has been deprecated. Use `/videos/{id}/views` instead
    pub async fn api_v1_videos_id_watching_put(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
        user_viewing_video: models::UserViewingVideo,
    ) -> Result<(), Error<ApiV1VideosIdWatchingPutError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/watching",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&user_viewing_video);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<ApiV1VideosIdWatchingPutError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn del_video(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
    ) -> Result<(), Error<DelVideoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DelVideoError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn delete_video_source_file(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
    ) -> Result<(), Error<DeleteVideoSourceFileError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/source/file",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteVideoSourceFileError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_account_videos_0(
        &self,
        name: &str,
        category_one_of: Option<models::GetAccountVideosCategoryOneOfParameter>,
        is_live: Option<bool>,
        tags_one_of: Option<models::GetAccountVideosTagsOneOfParameter>,
        tags_all_of: Option<models::GetAccountVideosTagsAllOfParameter>,
        licence_one_of: Option<models::GetAccountVideosLicenceOneOfParameter>,
        language_one_of: Option<models::GetAccountVideosLanguageOneOfParameter>,
        auto_tag_one_of: Option<models::GetAccountVideosTagsAllOfParameter>,
        nsfw: Option<&str>,
        is_local: Option<bool>,
        include: Option<i32>,
        privacy_one_of: Option<models::VideoPrivacySet>,
        has_hls_files: Option<bool>,
        has_web_video_files: Option<bool>,
        skip_count: Option<&str>,
        start: Option<i32>,
        count: Option<i32>,
        sort: Option<&str>,
        exclude_already_watched: Option<bool>,
        search: Option<&str>,
    ) -> Result<models::VideoListResponse, Error<GetAccountVideos0Error>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/accounts/{name}/videos",
            local_var_configuration.base_path,
            name = crate::apis::urlencode(name)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = category_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("categoryOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = is_live {
            local_var_req_builder =
                local_var_req_builder.query(&[("isLive", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = tags_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("tagsOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = tags_all_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("tagsAllOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = licence_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("licenceOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = language_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("languageOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = auto_tag_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("autoTagOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = nsfw {
            local_var_req_builder =
                local_var_req_builder.query(&[("nsfw", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = is_local {
            local_var_req_builder =
                local_var_req_builder.query(&[("isLocal", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = include {
            local_var_req_builder =
                local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = privacy_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("privacyOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = has_hls_files {
            local_var_req_builder =
                local_var_req_builder.query(&[("hasHLSFiles", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = has_web_video_files {
            local_var_req_builder =
                local_var_req_builder.query(&[("hasWebVideoFiles", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = skip_count {
            local_var_req_builder =
                local_var_req_builder.query(&[("skipCount", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = start {
            local_var_req_builder =
                local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = count {
            local_var_req_builder =
                local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = sort {
            local_var_req_builder =
                local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = exclude_already_watched {
            local_var_req_builder = local_var_req_builder
                .query(&[("excludeAlreadyWatched", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = search {
            local_var_req_builder =
                local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetAccountVideos0Error> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_categories(&self) -> Result<Vec<String>, Error<GetCategoriesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/categories",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetCategoriesError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_languages(&self) -> Result<Vec<String>, Error<GetLanguagesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/languages",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetLanguagesError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_licences(&self) -> Result<Vec<String>, Error<GetLicencesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/licences",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetLicencesError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_live_id_0(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
    ) -> Result<models::LiveVideoResponse, Error<GetLiveId0Error>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/live/{id}",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetLiveId0Error> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_video(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
        x_peertube_video_password: Option<&str>,
    ) -> Result<models::VideoDetails, Error<GetVideoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = x_peertube_video_password {
            local_var_req_builder = local_var_req_builder.header(
                "x-peertube-video-password",
                local_var_param_value.to_string(),
            );
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetVideoError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_video_channel_videos(
        &self,
        channel_handle: &str,
        category_one_of: Option<models::GetAccountVideosCategoryOneOfParameter>,
        is_live: Option<bool>,
        tags_one_of: Option<models::GetAccountVideosTagsOneOfParameter>,
        tags_all_of: Option<models::GetAccountVideosTagsAllOfParameter>,
        licence_one_of: Option<models::GetAccountVideosLicenceOneOfParameter>,
        language_one_of: Option<models::GetAccountVideosLanguageOneOfParameter>,
        auto_tag_one_of: Option<models::GetAccountVideosTagsAllOfParameter>,
        nsfw: Option<&str>,
        is_local: Option<bool>,
        include: Option<i32>,
        privacy_one_of: Option<models::VideoPrivacySet>,
        has_hls_files: Option<bool>,
        has_web_video_files: Option<bool>,
        skip_count: Option<&str>,
        start: Option<i32>,
        count: Option<i32>,
        sort: Option<&str>,
        exclude_already_watched: Option<bool>,
        search: Option<&str>,
    ) -> Result<models::VideoListResponse, Error<GetVideoChannelVideosError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/video-channels/{channelHandle}/videos",
            local_var_configuration.base_path,
            channelHandle = crate::apis::urlencode(channel_handle)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = category_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("categoryOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = is_live {
            local_var_req_builder =
                local_var_req_builder.query(&[("isLive", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = tags_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("tagsOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = tags_all_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("tagsAllOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = licence_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("licenceOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = language_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("languageOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = auto_tag_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("autoTagOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = nsfw {
            local_var_req_builder =
                local_var_req_builder.query(&[("nsfw", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = is_local {
            local_var_req_builder =
                local_var_req_builder.query(&[("isLocal", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = include {
            local_var_req_builder =
                local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = privacy_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("privacyOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = has_hls_files {
            local_var_req_builder =
                local_var_req_builder.query(&[("hasHLSFiles", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = has_web_video_files {
            local_var_req_builder =
                local_var_req_builder.query(&[("hasWebVideoFiles", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = skip_count {
            local_var_req_builder =
                local_var_req_builder.query(&[("skipCount", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = start {
            local_var_req_builder =
                local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = count {
            local_var_req_builder =
                local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = sort {
            local_var_req_builder =
                local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = exclude_already_watched {
            local_var_req_builder = local_var_req_builder
                .query(&[("excludeAlreadyWatched", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = search {
            local_var_req_builder =
                local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetVideoChannelVideosError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_video_desc(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
        x_peertube_video_password: Option<&str>,
    ) -> Result<String, Error<GetVideoDescError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/description",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = x_peertube_video_password {
            local_var_req_builder = local_var_req_builder.header(
                "x-peertube-video-password",
                local_var_param_value.to_string(),
            );
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetVideoDescError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_video_privacy_policies(
        &self,
    ) -> Result<Vec<String>, Error<GetVideoPrivacyPoliciesError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/privacies",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetVideoPrivacyPoliciesError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_video_source(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
    ) -> Result<models::VideoSource, Error<GetVideoSourceError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/source",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetVideoSourceError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn get_videos(
        &self,
        category_one_of: Option<models::GetAccountVideosCategoryOneOfParameter>,
        is_live: Option<bool>,
        tags_one_of: Option<models::GetAccountVideosTagsOneOfParameter>,
        tags_all_of: Option<models::GetAccountVideosTagsAllOfParameter>,
        licence_one_of: Option<models::GetAccountVideosLicenceOneOfParameter>,
        language_one_of: Option<models::GetAccountVideosLanguageOneOfParameter>,
        auto_tag_one_of: Option<models::GetAccountVideosTagsAllOfParameter>,
        nsfw: Option<&str>,
        is_local: Option<bool>,
        include: Option<i32>,
        privacy_one_of: Option<models::VideoPrivacySet>,
        has_hls_files: Option<bool>,
        has_web_video_files: Option<bool>,
        skip_count: Option<&str>,
        start: Option<i32>,
        count: Option<i32>,
        sort: Option<&str>,
        exclude_already_watched: Option<bool>,
        search: Option<&str>,
    ) -> Result<models::VideoListResponse, Error<GetVideosError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/api/v1/videos", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = category_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("categoryOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = is_live {
            local_var_req_builder =
                local_var_req_builder.query(&[("isLive", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = tags_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("tagsOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = tags_all_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("tagsAllOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = licence_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("licenceOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = language_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("languageOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = auto_tag_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("autoTagOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = nsfw {
            local_var_req_builder =
                local_var_req_builder.query(&[("nsfw", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = is_local {
            local_var_req_builder =
                local_var_req_builder.query(&[("isLocal", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = include {
            local_var_req_builder =
                local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = privacy_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("privacyOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = has_hls_files {
            local_var_req_builder =
                local_var_req_builder.query(&[("hasHLSFiles", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = has_web_video_files {
            local_var_req_builder =
                local_var_req_builder.query(&[("hasWebVideoFiles", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = skip_count {
            local_var_req_builder =
                local_var_req_builder.query(&[("skipCount", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = start {
            local_var_req_builder =
                local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = count {
            local_var_req_builder =
                local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = sort {
            local_var_req_builder =
                local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = exclude_already_watched {
            local_var_req_builder = local_var_req_builder
                .query(&[("excludeAlreadyWatched", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = search {
            local_var_req_builder =
                local_var_req_builder.query(&[("search", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetVideosError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// **PeerTube >= 6.0**
    pub async fn list_video_storyboards(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
    ) -> Result<models::ListVideoStoryboards200Response, Error<ListVideoStoryboardsError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/storyboards",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<ListVideoStoryboardsError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn put_video(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
        thumbnailfile: Option<std::path::PathBuf>,
        previewfile: Option<std::path::PathBuf>,
        category: Option<i32>,
        licence: Option<i32>,
        language: Option<&str>,
        privacy: Option<models::VideoPrivacySet>,
        description: Option<&str>,
        wait_transcoding: Option<&str>,
        support: Option<&str>,
        nsfw: Option<bool>,
        name: Option<&str>,
        tags: Option<Vec<String>>,
        comments_enabled: Option<bool>,
        comments_policy: Option<models::VideoCommentsPolicySet>,
        download_enabled: Option<bool>,
        originally_published_at: Option<String>,
        schedule_update: Option<models::VideoScheduledUpdate>,
        video_passwords: Option<Vec<String>>,
    ) -> Result<(), Error<PutVideoError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_form = reqwest::multipart::Form::new();
        // TODO: support file upload for 'thumbnailfile' parameter
        // TODO: support file upload for 'previewfile' parameter
        if let Some(local_var_param_value) = category {
            local_var_form = local_var_form.text("category", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = licence {
            local_var_form = local_var_form.text("licence", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = language {
            local_var_form = local_var_form.text("language", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = privacy {
            local_var_form = local_var_form.text("privacy", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = description {
            local_var_form = local_var_form.text("description", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = wait_transcoding {
            local_var_form =
                local_var_form.text("waitTranscoding", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = support {
            local_var_form = local_var_form.text("support", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = nsfw {
            local_var_form = local_var_form.text("nsfw", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = name {
            local_var_form = local_var_form.text("name", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = tags {
            local_var_form = local_var_form.text(
                "tags",
                local_var_param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            );
        }
        if let Some(local_var_param_value) = comments_enabled {
            local_var_form =
                local_var_form.text("commentsEnabled", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = comments_policy {
            local_var_form =
                local_var_form.text("commentsPolicy", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = download_enabled {
            local_var_form =
                local_var_form.text("downloadEnabled", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = originally_published_at {
            local_var_form =
                local_var_form.text("originallyPublishedAt", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = schedule_update {
            local_var_form =
                local_var_form.text("scheduleUpdate", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = video_passwords {
            local_var_form = local_var_form.text(
                "videoPasswords",
                local_var_param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            );
        }
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<PutVideoError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// **PeerTube >= 6.0** Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to continue, pause or resume the replacement of a video
    pub async fn replace_video_source_resumable(
        &self,
        id: &str,
        upload_id: &str,
        content_range: &str,
        content_length: f64,
        body: Option<std::path::PathBuf>,
    ) -> Result<(), Error<ReplaceVideoSourceResumableError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/source/replace-resumable",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        local_var_req_builder =
            local_var_req_builder.query(&[("upload_id", &upload_id.to_string())]);
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder =
            local_var_req_builder.header("Content-Range", content_range.to_string());
        local_var_req_builder =
            local_var_req_builder.header("Content-Length", content_length.to_string());
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&body);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<ReplaceVideoSourceResumableError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// **PeerTube >= 6.0** Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to cancel the replacement of a video
    pub async fn replace_video_source_resumable_cancel(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
        upload_id: &str,
        content_length: f64,
    ) -> Result<(), Error<ReplaceVideoSourceResumableCancelError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/source/replace-resumable",
            local_var_configuration.base_path, id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        local_var_req_builder =
            local_var_req_builder.query(&[("upload_id", &upload_id.to_string())]);
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder =
            local_var_req_builder.header("Content-Length", content_length.to_string());
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<ReplaceVideoSourceResumableCancelError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// **PeerTube >= 6.0** Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to initialize the replacement of a video
    pub async fn replace_video_source_resumable_init(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
        x_upload_content_length: f64,
        x_upload_content_type: &str,
        video_replace_source_request_resumable: Option<models::VideoReplaceSourceRequestResumable>,
    ) -> Result<(), Error<ReplaceVideoSourceResumableInitError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/source/replace-resumable",
            local_var_configuration.base_path, id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.header(
            "X-Upload-Content-Length",
            x_upload_content_length.to_string(),
        );
        local_var_req_builder = local_var_req_builder
            .header("X-Upload-Content-Type", x_upload_content_type.to_string());
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&video_replace_source_request_resumable);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<ReplaceVideoSourceResumableInitError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Request special tokens that expire quickly to use them in some context (like accessing private static files)
    pub async fn request_video_token(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
        x_peertube_video_password: Option<&str>,
    ) -> Result<models::VideoTokenResponse, Error<RequestVideoTokenError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/{id}/token",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(local_var_param_value) = x_peertube_video_password {
            local_var_req_builder = local_var_req_builder.header(
                "x-peertube-video-password",
                local_var_param_value.to_string(),
            );
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<RequestVideoTokenError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn search_videos_0(
        &self,
        search: &str,
        category_one_of: Option<models::GetAccountVideosCategoryOneOfParameter>,
        is_live: Option<bool>,
        tags_one_of: Option<models::GetAccountVideosTagsOneOfParameter>,
        tags_all_of: Option<models::GetAccountVideosTagsAllOfParameter>,
        licence_one_of: Option<models::GetAccountVideosLicenceOneOfParameter>,
        language_one_of: Option<models::GetAccountVideosLanguageOneOfParameter>,
        auto_tag_one_of: Option<models::GetAccountVideosTagsAllOfParameter>,
        nsfw: Option<&str>,
        is_local: Option<bool>,
        include: Option<i32>,
        privacy_one_of: Option<models::VideoPrivacySet>,
        uuids: Option<Vec<String>>,
        has_hls_files: Option<bool>,
        has_web_video_files: Option<bool>,
        skip_count: Option<&str>,
        start: Option<i32>,
        count: Option<i32>,
        search_target: Option<&str>,
        sort: Option<&str>,
        exclude_already_watched: Option<bool>,
        host: Option<&str>,
        start_date: Option<String>,
        end_date: Option<String>,
        originally_published_start_date: Option<String>,
        originally_published_end_date: Option<String>,
        duration_min: Option<i32>,
        duration_max: Option<i32>,
    ) -> Result<models::VideoListResponse, Error<SearchVideos0Error>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str =
            format!("{}/api/v1/search/videos", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        local_var_req_builder = local_var_req_builder.query(&[("search", &search.to_string())]);
        if let Some(ref local_var_str) = category_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("categoryOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = is_live {
            local_var_req_builder =
                local_var_req_builder.query(&[("isLive", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = tags_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("tagsOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = tags_all_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("tagsAllOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = licence_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("licenceOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = language_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("languageOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = auto_tag_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("autoTagOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = nsfw {
            local_var_req_builder =
                local_var_req_builder.query(&[("nsfw", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = is_local {
            local_var_req_builder =
                local_var_req_builder.query(&[("isLocal", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = include {
            local_var_req_builder =
                local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = privacy_one_of {
            local_var_req_builder =
                local_var_req_builder.query(&[("privacyOneOf", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = uuids {
            local_var_req_builder = match "multi" {
                "multi" => local_var_req_builder.query(
                    &local_var_str
                        .into_iter()
                        .map(|p| ("uuids".to_owned(), p.to_string()))
                        .collect::<Vec<(std::string::String, std::string::String)>>(),
                ),
                _ => local_var_req_builder.query(&[(
                    "uuids",
                    &local_var_str
                        .into_iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                        .to_string(),
                )]),
            };
        }
        if let Some(ref local_var_str) = has_hls_files {
            local_var_req_builder =
                local_var_req_builder.query(&[("hasHLSFiles", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = has_web_video_files {
            local_var_req_builder =
                local_var_req_builder.query(&[("hasWebVideoFiles", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = skip_count {
            local_var_req_builder =
                local_var_req_builder.query(&[("skipCount", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = start {
            local_var_req_builder =
                local_var_req_builder.query(&[("start", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = count {
            local_var_req_builder =
                local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = search_target {
            local_var_req_builder =
                local_var_req_builder.query(&[("searchTarget", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = sort {
            local_var_req_builder =
                local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = exclude_already_watched {
            local_var_req_builder = local_var_req_builder
                .query(&[("excludeAlreadyWatched", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = host {
            local_var_req_builder =
                local_var_req_builder.query(&[("host", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = start_date {
            local_var_req_builder =
                local_var_req_builder.query(&[("startDate", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = end_date {
            local_var_req_builder =
                local_var_req_builder.query(&[("endDate", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = originally_published_start_date {
            local_var_req_builder = local_var_req_builder
                .query(&[("originallyPublishedStartDate", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = originally_published_end_date {
            local_var_req_builder = local_var_req_builder
                .query(&[("originallyPublishedEndDate", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = duration_min {
            local_var_req_builder =
                local_var_req_builder.query(&[("durationMin", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = duration_max {
            local_var_req_builder =
                local_var_req_builder.query(&[("durationMax", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<SearchVideos0Error> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn update_live_id_0(
        &self,
        id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,
        live_video_update: Option<models::LiveVideoUpdate>,
    ) -> Result<(), Error<UpdateLiveId0Error>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/live/{id}",
            local_var_configuration.base_path,
            id = id.to_string()
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&live_video_update);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<UpdateLiveId0Error> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Uses a single request to upload a video.
    pub async fn upload_legacy(
        &self,
        name: &str,
        videofile: std::path::PathBuf,
        channel_id: Option<i32>,
        privacy: Option<models::VideoPrivacySet>,
        category: Option<i32>,
        licence: Option<i32>,
        language: Option<&str>,
        description: Option<&str>,
        wait_transcoding: Option<bool>,
        generate_transcription: Option<bool>,
        support: Option<&str>,
        nsfw: Option<bool>,
        tags: Option<Vec<String>>,
        comments_enabled: Option<bool>,
        comments_policy: Option<models::VideoCommentsPolicySet>,
        download_enabled: Option<bool>,
        originally_published_at: Option<String>,
        schedule_update: Option<models::VideoScheduledUpdate>,
        thumbnailfile: Option<std::path::PathBuf>,
        previewfile: Option<std::path::PathBuf>,
        video_passwords: Option<Vec<String>>,
    ) -> Result<models::VideoUploadResponse, Error<UploadLegacyError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str =
            format!("{}/api/v1/videos/upload", local_var_configuration.base_path);
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        let mut local_var_form = reqwest::multipart::Form::new();
        local_var_form = local_var_form.text("name", name.to_string());
        if let Some(local_var_param_value) = channel_id {
            local_var_form = local_var_form.text("channelId", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = privacy {
            local_var_form = local_var_form.text("privacy", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = category {
            local_var_form = local_var_form.text("category", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = licence {
            local_var_form = local_var_form.text("licence", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = language {
            local_var_form = local_var_form.text("language", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = description {
            local_var_form = local_var_form.text("description", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = wait_transcoding {
            local_var_form =
                local_var_form.text("waitTranscoding", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = generate_transcription {
            local_var_form =
                local_var_form.text("generateTranscription", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = support {
            local_var_form = local_var_form.text("support", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = nsfw {
            local_var_form = local_var_form.text("nsfw", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = tags {
            local_var_form = local_var_form.text(
                "tags",
                local_var_param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            );
        }
        if let Some(local_var_param_value) = comments_enabled {
            local_var_form =
                local_var_form.text("commentsEnabled", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = comments_policy {
            local_var_form =
                local_var_form.text("commentsPolicy", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = download_enabled {
            local_var_form =
                local_var_form.text("downloadEnabled", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = originally_published_at {
            local_var_form =
                local_var_form.text("originallyPublishedAt", local_var_param_value.to_string());
        }
        if let Some(local_var_param_value) = schedule_update {
            local_var_form =
                local_var_form.text("scheduleUpdate", local_var_param_value.to_string());
        }
        // TODO: support file upload for 'thumbnailfile' parameter
        // TODO: support file upload for 'previewfile' parameter
        if let Some(local_var_param_value) = video_passwords {
            local_var_form = local_var_form.text(
                "videoPasswords",
                local_var_param_value
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            );
        }
        // TODO: support file upload for 'videofile' parameter
        local_var_req_builder = local_var_req_builder.multipart(local_var_form);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UploadLegacyError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to continue, pause or resume the upload of a video
    pub async fn upload_resumable(
        &self,
        upload_id: &str,
        content_range: &str,
        content_length: f64,
        body: Option<std::path::PathBuf>,
    ) -> Result<models::VideoUploadResponse, Error<UploadResumableError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/upload-resumable",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        local_var_req_builder =
            local_var_req_builder.query(&[("upload_id", &upload_id.to_string())]);
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder =
            local_var_req_builder.header("Content-Range", content_range.to_string());
        local_var_req_builder =
            local_var_req_builder.header("Content-Length", content_length.to_string());
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&body);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<UploadResumableError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to cancel the upload of a video
    pub async fn upload_resumable_cancel(
        &self,
        upload_id: &str,
        content_length: f64,
    ) -> Result<(), Error<UploadResumableCancelError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/upload-resumable",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        local_var_req_builder =
            local_var_req_builder.query(&[("upload_id", &upload_id.to_string())]);
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder =
            local_var_req_builder.header("Content-Length", content_length.to_string());
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<UploadResumableCancelError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to initialize the upload of a video
    pub async fn upload_resumable_init(
        &self,
        x_upload_content_length: f64,
        x_upload_content_type: &str,
        video_upload_request_resumable: Option<models::VideoUploadRequestResumable>,
    ) -> Result<(), Error<UploadResumableInitError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/videos/upload-resumable",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        local_var_req_builder = local_var_req_builder.header(
            "X-Upload-Content-Length",
            x_upload_content_length.to_string(),
        );
        local_var_req_builder = local_var_req_builder
            .header("X-Upload-Content-Type", x_upload_content_type.to_string());
        if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&video_upload_request_resumable);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<UploadResumableInitError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}
