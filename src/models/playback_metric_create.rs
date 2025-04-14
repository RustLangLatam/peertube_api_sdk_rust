/*
 * PeerTube
 *
 * The PeerTube API is built on HTTP(S) and is RESTful. You can use your favorite HTTP/REST library for your programming language to use PeerTube. The spec API is fully compatible with [openapi-generator](https://github.com/OpenAPITools/openapi-generator/wiki/API-client-generator-HOWTO) which generates a client SDK in the language of your choice - we generate some client SDKs automatically:  - [Python](https://framagit.org/framasoft/peertube/clients/python) - [Go](https://framagit.org/framasoft/peertube/clients/go) - [Kotlin](https://framagit.org/framasoft/peertube/clients/kotlin)  See the [REST API quick start](https://docs.joinpeertube.org/api/rest-getting-started) for a few examples of using the PeerTube API.  # Authentication  When you sign up for an account on a PeerTube instance, you are given the possibility to generate sessions on it, and authenticate there using an access token. Only __one access token can currently be used at a time__.  ## Roles  Accounts are given permissions based on their role. There are three roles on PeerTube: Administrator, Moderator, and User. See the [roles guide](https://docs.joinpeertube.org/admin/managing-users#roles) for a detail of their permissions.  # Errors  The API uses standard HTTP status codes to indicate the success or failure of the API call, completed by a [RFC7807-compliant](https://tools.ietf.org/html/rfc7807) response body.  ``` HTTP 1.1 404 Not Found Content-Type: application/problem+json; charset=utf-8  {   \"detail\": \"Video not found\",   \"docs\": \"https://docs.joinpeertube.org/api-rest-reference.html#operation/getVideo\",   \"status\": 404,   \"title\": \"Not Found\",   \"type\": \"about:blank\" } ```  We provide error `type` (following RFC7807) and `code` (internal PeerTube code) values for [a growing number of cases](https://github.com/Chocobozzz/PeerTube/blob/develop/packages/models/src/server/server-error-code.enum.ts), but it is still optional. Types are used to disambiguate errors that bear the same status code and are non-obvious:  ``` HTTP 1.1 403 Forbidden Content-Type: application/problem+json; charset=utf-8  {   \"detail\": \"Cannot get this video regarding follow constraints\",   \"docs\": \"https://docs.joinpeertube.org/api-rest-reference.html#operation/getVideo\",   \"status\": 403,   \"title\": \"Forbidden\",   \"type\": \"https://docs.joinpeertube.org/api-rest-reference.html#section/Errors/does_not_respect_follow_constraints\" } ```  Here a 403 error could otherwise mean that the video is private or blocklisted.  ### Validation errors  Each parameter is evaluated on its own against a set of rules before the route validator proceeds with potential testing involving parameter combinations. Errors coming from validation errors appear earlier and benefit from a more detailed error description:  ``` HTTP 1.1 400 Bad Request Content-Type: application/problem+json; charset=utf-8  {   \"detail\": \"Incorrect request parameters: id\",   \"docs\": \"https://docs.joinpeertube.org/api-rest-reference.html#operation/getVideo\",   \"instance\": \"/api/v1/videos/9c9de5e8-0a1e-484a-b099-e80766180\",   \"invalid-params\": {     \"id\": {       \"location\": \"params\",       \"msg\": \"Invalid value\",       \"param\": \"id\",       \"value\": \"9c9de5e8-0a1e-484a-b099-e80766180\"     }   },   \"status\": 400,   \"title\": \"Bad Request\",   \"type\": \"about:blank\" } ```  Where `id` is the name of the field concerned by the error, within the route definition. `invalid-params.<field>.location` can be either 'params', 'body', 'header', 'query' or 'cookies', and `invalid-params.<field>.value` reports the value that didn't pass validation whose `invalid-params.<field>.msg` is about.  ### Deprecated error fields  Some fields could be included with previous versions. They are still included but their use is deprecated: - `error`: superseded by `detail`  # Rate limits  We are rate-limiting all endpoints of PeerTube's API. Custom values can be set by administrators:  | Endpoint (prefix: `/api/v1`) | Calls         | Time frame   | |------------------------------|---------------|--------------| | `/_*`                         | 50            | 10 seconds   | | `POST /users/token`          | 15            | 5 minutes    | | `POST /users/register`       | 2<sup>*</sup> | 5 minutes    | | `POST /users/ask-send-verify-email` | 3      | 5 minutes    |  Depending on the endpoint, <sup>*</sup>failed requests are not taken into account. A service limit is announced by a `429 Too Many Requests` status code.  You can get details about the current state of your rate limit by reading the following headers:  | Header                  | Description                                                | |-------------------------|------------------------------------------------------------| | `X-RateLimit-Limit`     | Number of max requests allowed in the current time period  | | `X-RateLimit-Remaining` | Number of remaining requests in the current time period    | | `X-RateLimit-Reset`     | Timestamp of end of current time period as UNIX timestamp  | | `Retry-After`           | Seconds to delay after the first `429` is received         |  # CORS  This API features [Cross-Origin Resource Sharing (CORS)](https://fetch.spec.whatwg.org/), allowing cross-domain communication from the browser for some routes:  | Endpoint                    | |------------------------- ---| | `/api/_*`                    | | `/download/_*`               | | `/lazy-static/_*`            | | `/.well-known/webfinger`    |  In addition, all routes serving ActivityPub are CORS-enabled for all origins. 
 *
 * The version of the OpenAPI document: 7.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlaybackMetricCreate {
    #[serde(rename = "playerMode")]
    pub player_mode: PlayerMode,
    /// Current player video resolution
    #[serde(rename = "resolution", skip_serializing_if = "Option::is_none")]
    pub resolution: Option<f64>,
    /// Current player video fps
    #[serde(rename = "fps", skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
    #[serde(rename = "p2pEnabled")]
    pub p2p_enabled: bool,
    /// P2P peers connected (doesn't include WebSeed peers)
    #[serde(rename = "p2pPeers", skip_serializing_if = "Option::is_none")]
    pub p2p_peers: Option<f64>,
    /// How many resolution changes occurred since the last metric creation
    #[serde(rename = "resolutionChanges")]
    pub resolution_changes: f64,
    /// How many times buffer has been stalled since the last metric creation
    #[serde(rename = "bufferStalled", skip_serializing_if = "Option::is_none")]
    pub buffer_stalled: Option<f64>,
    /// How many errors occurred since the last metric creation
    #[serde(rename = "errors")]
    pub errors: f64,
    /// How many bytes were downloaded with P2P since the last metric creation
    #[serde(rename = "downloadedBytesP2P")]
    pub downloaded_bytes_p2_p: f64,
    /// How many bytes were downloaded with HTTP since the last metric creation
    #[serde(rename = "downloadedBytesHTTP")]
    pub downloaded_bytes_http: f64,
    /// How many bytes were uploaded with P2P since the last metric creation
    #[serde(rename = "uploadedBytesP2P")]
    pub uploaded_bytes_p2_p: f64,
    #[serde(rename = "videoId")]
    pub video_id: Box<models::ApiV1VideosOwnershipIdAcceptPostIdParameter>,
}

impl PlaybackMetricCreate {
    pub fn new(player_mode: PlayerMode, p2p_enabled: bool, resolution_changes: f64, errors: f64, downloaded_bytes_p2_p: f64, downloaded_bytes_http: f64, uploaded_bytes_p2_p: f64, video_id: models::ApiV1VideosOwnershipIdAcceptPostIdParameter) -> PlaybackMetricCreate {
        PlaybackMetricCreate {
            player_mode,
            resolution: None,
            fps: None,
            p2p_enabled,
            p2p_peers: None,
            resolution_changes,
            buffer_stalled: None,
            errors,
            downloaded_bytes_p2_p,
            downloaded_bytes_http,
            uploaded_bytes_p2_p,
            video_id: Box::new(video_id),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayerMode {
    #[serde(rename = "p2p-media-loader")]
    P2pMediaLoader,
    #[serde(rename = "web-video")]
    WebVideo,
}

impl Default for PlayerMode {
    fn default() -> PlayerMode {
        Self::P2pMediaLoader
    }
}

