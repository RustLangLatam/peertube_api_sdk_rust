/*
 * PeerTube
 *
 * The PeerTube API is built on HTTP(S) and is RESTful. You can use your favorite HTTP/REST library for your programming language to use PeerTube. The spec API is fully compatible with [openapi-generator](https://github.com/OpenAPITools/openapi-generator/wiki/API-client-generator-HOWTO) which generates a client SDK in the language of your choice - we generate some client SDKs automatically:  - [Python](https://framagit.org/framasoft/peertube/clients/python) - [Go](https://framagit.org/framasoft/peertube/clients/go) - [Kotlin](https://framagit.org/framasoft/peertube/clients/kotlin)  See the [REST API quick start](https://docs.joinpeertube.org/api/rest-getting-started) for a few examples of using the PeerTube API.  # Authentication  When you sign up for an account on a PeerTube instance, you are given the possibility to generate sessions on it, and authenticate there using an access token. Only __one access token can currently be used at a time__.  ## Roles  Accounts are given permissions based on their role. There are three roles on PeerTube: Administrator, Moderator, and User. See the [roles guide](https://docs.joinpeertube.org/admin/managing-users#roles) for a detail of their permissions.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call, completed by a [RFC7807-compliant](https://tools.ietf.org/html/rfc7807) response body.  ``` HTTP 1.1 404 Not Found Content-Type: application/problem+json; charset=utf-8  {   \"detail\": \"Video not found\",   \"docs\": \"https://docs.joinpeertube.org/api-rest-reference.html#operation/getVideo\",   \"status\": 404,   \"title\": \"Not Found\",   \"type\": \"about:blank\" } ```  We provide error `type` (following RFC7807) and `code` (internal PeerTube code) values for [a growing number of cases](https://github.com/Chocobozzz/PeerTube/blob/develop/packages/models/src/server/server-error-code.enum.ts), but it is still optional. Types are used to disambiguate errors that bear the same status code and are non-obvious:  ``` HTTP 1.1 403 Forbidden Content-Type: application/problem+json; charset=utf-8  {   \"detail\": \"Cannot get this video regarding follow constraints\",   \"docs\": \"https://docs.joinpeertube.org/api-rest-reference.html#operation/getVideo\",   \"status\": 403,   \"title\": \"Forbidden\",   \"type\": \"https://docs.joinpeertube.org/api-rest-reference.html#section/Errors/does_not_respect_follow_constraints\" } ```  Here a 403 error could otherwise mean that the video is private or blocklisted.  ### Validation errors  Each parameter is evaluated on its own against a set of rules before the route validator proceeds with potential testing involving parameter combinations. Errors coming from validation errors appear earlier and benefit from a more detailed error description:  ``` HTTP 1.1 400 Bad Request Content-Type: application/problem+json; charset=utf-8  {   \"detail\": \"Incorrect request parameters: id\",   \"docs\": \"https://docs.joinpeertube.org/api-rest-reference.html#operation/getVideo\",   \"instance\": \"/api/v1/videos/9c9de5e8-0a1e-484a-b099-e80766180\",   \"invalid-params\": {     \"id\": {       \"location\": \"params\",       \"msg\": \"Invalid value\",       \"param\": \"id\",       \"value\": \"9c9de5e8-0a1e-484a-b099-e80766180\"     }   },   \"status\": 400,   \"title\": \"Bad Request\",   \"type\": \"about:blank\" } ```  Where `id` is the name of the field concerned by the error, within the route definition. `invalid-params.<field>.location` can be either 'params', 'body', 'header', 'query' or 'cookies', and `invalid-params.<field>.value` reports the value that didn't pass validation whose `invalid-params.<field>.msg` is about.  ### Deprecated error fields  Some fields could be included with previous versions. They are still included but their use is deprecated: - `error`: superseded by `detail`  # Rate limits  We are rate-limiting all endpoints of PeerTube's API. Custom values can be set by administrators:  | Endpoint (prefix: `/api/v1`) | Calls         | Time frame   | |------------------------------|---------------|--------------| | `/_*`                         | 50            | 10 seconds   | | `POST /users/token`          | 15            | 5 minutes    | | `POST /users/register`       | 2<sup>*</sup> | 5 minutes    | | `POST /users/ask-send-verify-email` | 3      | 5 minutes    |  Depending on the endpoint, <sup>*</sup>failed requests are not taken into account. A service limit is announced by a `429 Too Many Requests` status code.  You can get details about the current state of your rate limit by reading the following headers:  | Header                  | Description                                                | |-------------------------|------------------------------------------------------------| | `X-RateLimit-Limit`     | Number of max requests allowed in the current time period  | | `X-RateLimit-Remaining` | Number of remaining requests in the current time period    | | `X-RateLimit-Reset`     | Timestamp of end of current time period as UNIX timestamp  | | `Retry-After`           | Seconds to delay after the first `429` is received         |  # CORS  This API features [Cross-Origin Resource Sharing (CORS)](https://fetch.spec.whatwg.org/), allowing cross-domain communication from the browser for some routes:  | Endpoint                    | |------------------------- ---| | `/api/_*`                    | | `/download/_*`               | | `/lazy-static/_*`            | | `/.well-known/webfinger`    |  In addition, all routes serving ActivityPub are CORS-enabled for all origins. 
 *
 * The version of the OpenAPI document: 7.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`import_video_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImportVideo0Error {
    Status400(),
    Status403(),
    Status409(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_video_source_resumable_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceVideoSourceResumable0Error {
    Status403(),
    Status404(),
    Status409(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_video_source_resumable_cancel_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceVideoSourceResumableCancel0Error {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_video_source_resumable_init_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceVideoSourceResumableInit0Error {
    Status413(),
    Status415(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_legacy_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadLegacy0Error {
    Status403(),
    Status408(),
    Status413(),
    Status415(),
    Status422(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_resumable_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadResumable0Error {
    Status403(),
    Status404(),
    Status409(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_resumable_cancel_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadResumableCancel0Error {
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload_resumable_init_0`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadResumableInit0Error {
    Status413(),
    Status415(),
    UnknownValue(serde_json::Value),
}


/// Import a torrent or magnetURI or HTTP resource (if enabled by the instance administrator)
pub async fn import_video_0(configuration: &configuration::Configuration, name: &str, channel_id: i32, target_url: Option<String>, magnet_uri: Option<String>, torrentfile: Option<std::path::PathBuf>, privacy: Option<models::VideoPrivacySet>, category: Option<i32>, licence: Option<i32>, language: Option<&str>, description: Option<&str>, wait_transcoding: Option<bool>, generate_transcription: Option<bool>, support: Option<&str>, nsfw: Option<bool>, tags: Option<Vec<String>>, comments_enabled: Option<bool>, comments_policy: Option<models::VideoCommentsPolicySet>, download_enabled: Option<bool>, originally_published_at: Option<String>, schedule_update: Option<models::VideoScheduledUpdate>, thumbnailfile: Option<std::path::PathBuf>, previewfile: Option<std::path::PathBuf>, video_passwords: Option<Vec<String>>) -> Result<models::VideoUploadResponse, Error<ImportVideo0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/videos/imports", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    if let Some(local_var_param_value) = target_url {
        local_var_form = local_var_form.text("targetUrl", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = magnet_uri {
        local_var_form = local_var_form.text("magnetUri", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = torrentfile {
        local_var_form = local_var_form.text("torrentfile", local_var_param_value.display().to_string());
    }
    local_var_form = local_var_form.text("name", name.to_string());
    local_var_form = local_var_form.text("channelId", channel_id.to_string());
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
        local_var_form = local_var_form.text("waitTranscoding", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = generate_transcription {
        local_var_form = local_var_form.text("generateTranscription", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = support {
        local_var_form = local_var_form.text("support", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = nsfw {
        local_var_form = local_var_form.text("nsfw", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tags {
        local_var_form = local_var_form.text("tags", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = comments_enabled {
        local_var_form = local_var_form.text("commentsEnabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comments_policy {
        local_var_form = local_var_form.text("commentsPolicy", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = download_enabled {
        local_var_form = local_var_form.text("downloadEnabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = originally_published_at {
        local_var_form = local_var_form.text("originallyPublishedAt", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = schedule_update {
        local_var_form = local_var_form.text("scheduleUpdate", local_var_param_value.to_string());
    }
    // TODO: support file upload for 'thumbnailfile' parameter
    // TODO: support file upload for 'previewfile' parameter
    if let Some(local_var_param_value) = video_passwords {
        local_var_form = local_var_form.text("videoPasswords", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ImportVideo0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// **PeerTube >= 6.0** Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to continue, pause or resume the replacement of a video
pub async fn replace_video_source_resumable_0(configuration: &configuration::Configuration, id:
models::ApiV1VideosOwnershipIdAcceptPostIdParameter, upload_id: &str, content_range: &str, content_length: f64, body: Option<std::path::PathBuf>) -> Result<(), Error<ReplaceVideoSourceResumable0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/videos/{id}/source/replace-resumable", local_var_configuration
        .base_path, id=id.to_string());
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("upload_id", &upload_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Content-Range", content_range.to_string());
    local_var_req_builder = local_var_req_builder.header("Content-Length", content_length.to_string());
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
        let local_var_entity: Option<ReplaceVideoSourceResumable0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// **PeerTube >= 6.0** Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to cancel the replacement of a video
pub async fn replace_video_source_resumable_cancel_0(configuration: &configuration::Configuration, id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,upload_id: &str, content_length: f64) -> Result<(), Error<ReplaceVideoSourceResumableCancel0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/videos/{id}/source/replace-resumable", local_var_configuration
        .base_path, id=id.to_string());
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("upload_id", &upload_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Content-Length", content_length.to_string());
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
        let local_var_entity: Option<ReplaceVideoSourceResumableCancel0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// **PeerTube >= 6.0** Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to initialize the replacement of a video
pub async fn replace_video_source_resumable_init_0(configuration: &configuration::Configuration, id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter,x_upload_content_length: f64, x_upload_content_type: &str, video_replace_source_request_resumable: Option<models::VideoReplaceSourceRequestResumable>) -> Result<(), Error<ReplaceVideoSourceResumableInit0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/videos/{id}/source/replace-resumable", local_var_configuration
        .base_path, id=id.to_string());
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Upload-Content-Length", x_upload_content_length.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Upload-Content-Type", x_upload_content_type.to_string());
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
        let local_var_entity: Option<ReplaceVideoSourceResumableInit0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Uses a single request to upload a video.
pub async fn upload_legacy_0(configuration: &configuration::Configuration, name: &str, videofile: std::path::PathBuf, channel_id: Option<i32>, privacy: Option<models::VideoPrivacySet>, category: Option<i32>, licence: Option<i32>, language: Option<&str>, description: Option<&str>, wait_transcoding: Option<bool>, generate_transcription: Option<bool>, support: Option<&str>, nsfw: Option<bool>, tags: Option<Vec<String>>, comments_enabled: Option<bool>, comments_policy: Option<models::VideoCommentsPolicySet>, download_enabled: Option<bool>, originally_published_at: Option<String>, schedule_update: Option<models::VideoScheduledUpdate>, thumbnailfile: Option<std::path::PathBuf>, previewfile: Option<std::path::PathBuf>, video_passwords: Option<Vec<String>>) -> Result<models::VideoUploadResponse, Error<UploadLegacy0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/videos/upload", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
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
        local_var_form = local_var_form.text("waitTranscoding", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = generate_transcription {
        local_var_form = local_var_form.text("generateTranscription", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = support {
        local_var_form = local_var_form.text("support", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = nsfw {
        local_var_form = local_var_form.text("nsfw", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = tags {
        local_var_form = local_var_form.text("tags", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    if let Some(local_var_param_value) = comments_enabled {
        local_var_form = local_var_form.text("commentsEnabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = comments_policy {
        local_var_form = local_var_form.text("commentsPolicy", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = download_enabled {
        local_var_form = local_var_form.text("downloadEnabled", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = originally_published_at {
        local_var_form = local_var_form.text("originallyPublishedAt", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = schedule_update {
        local_var_form = local_var_form.text("scheduleUpdate", local_var_param_value.to_string());
    }
    // TODO: support file upload for 'thumbnailfile' parameter
    // TODO: support file upload for 'previewfile' parameter
    if let Some(local_var_param_value) = video_passwords {
        local_var_form = local_var_form.text("videoPasswords", local_var_param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
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
        let local_var_entity: Option<UploadLegacy0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to continue, pause or resume the upload of a video
pub async fn upload_resumable_0(configuration: &configuration::Configuration, upload_id: &str, content_range: &str, content_length: f64, body: Option<std::path::PathBuf>) -> Result<models::VideoUploadResponse, Error<UploadResumable0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/videos/upload-resumable", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("upload_id", &upload_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Content-Range", content_range.to_string());
    local_var_req_builder = local_var_req_builder.header("Content-Length", content_length.to_string());
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
        let local_var_entity: Option<UploadResumable0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to cancel the upload of a video
pub async fn upload_resumable_cancel_0(configuration: &configuration::Configuration, upload_id: &str, content_length: f64) -> Result<(), Error<UploadResumableCancel0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/videos/upload-resumable", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("upload_id", &upload_id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("Content-Length", content_length.to_string());
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
        let local_var_entity: Option<UploadResumableCancel0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Uses [a resumable protocol](https://github.com/kukhariev/node-uploadx/blob/master/proto.md) to initialize the upload of a video
pub async fn upload_resumable_init_0(configuration: &configuration::Configuration, x_upload_content_length: f64, x_upload_content_type: &str, video_upload_request_resumable: Option<models::VideoUploadRequestResumable>) -> Result<(), Error<UploadResumableInit0Error>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/api/v1/videos/upload-resumable", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Upload-Content-Length", x_upload_content_length.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Upload-Content-Type", x_upload_content_type.to_string());
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
        let local_var_entity: Option<UploadResumableInit0Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

