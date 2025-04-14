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
pub struct User {
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<models::Account>>,
    /// Automatically start playing the upcoming video after the currently playing video
    #[serde(rename = "autoPlayNextVideo", skip_serializing_if = "Option::is_none")]
    pub auto_play_next_video: Option<bool>,
    /// Automatically start playing the video on the playlist after the currently playing video
    #[serde(rename = "autoPlayNextVideoPlaylist", skip_serializing_if = "Option::is_none")]
    pub auto_play_next_video_playlist: Option<bool>,
    /// Automatically start playing the video on the watch page
    #[serde(rename = "autoPlayVideo", skip_serializing_if = "Option::is_none")]
    pub auto_play_video: Option<bool>,
    #[serde(rename = "blocked", skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(rename = "blockedReason", skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The user email
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Has the user confirmed their email address?
    #[serde(rename = "emailVerified", skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Auth plugin to use to authenticate the user
    #[serde(rename = "pluginAuth", skip_serializing_if = "Option::is_none")]
    pub plugin_auth: Option<String>,
    #[serde(rename = "lastLoginDate", skip_serializing_if = "Option::is_none")]
    pub last_login_date: Option<String>,
    #[serde(rename = "noInstanceConfigWarningModal", skip_serializing_if = "Option::is_none")]
    pub no_instance_config_warning_modal: Option<bool>,
    #[serde(rename = "noAccountSetupWarningModal", skip_serializing_if = "Option::is_none")]
    pub no_account_setup_warning_modal: Option<bool>,
    #[serde(rename = "noWelcomeModal", skip_serializing_if = "Option::is_none")]
    pub no_welcome_modal: Option<bool>,
    #[serde(rename = "nsfwPolicy", skip_serializing_if = "Option::is_none")]
    pub nsfw_policy: Option<models::NsfwPolicy>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<models::UserRole>>,
    /// Theme enabled by this user
    #[serde(rename = "theme", skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    /// immutable name of the user, used to find or mention its actor
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "videoChannels", skip_serializing_if = "Option::is_none")]
    pub video_channels: Option<Vec<models::VideoChannel>>,
    /// The user video quota in bytes
    #[serde(rename = "videoQuota", skip_serializing_if = "Option::is_none")]
    pub video_quota: Option<i32>,
    /// The user daily video quota in bytes
    #[serde(rename = "videoQuotaDaily", skip_serializing_if = "Option::is_none")]
    pub video_quota_daily: Option<i32>,
    /// Enable P2P in the player
    #[serde(rename = "p2pEnabled", skip_serializing_if = "Option::is_none")]
    pub p2p_enabled: Option<bool>,
}

impl User {
    pub fn new() -> User {
        User {
            account: None,
            auto_play_next_video: None,
            auto_play_next_video_playlist: None,
            auto_play_video: None,
            blocked: None,
            blocked_reason: None,
            created_at: None,
            email: None,
            email_verified: None,
            id: None,
            plugin_auth: None,
            last_login_date: None,
            no_instance_config_warning_modal: None,
            no_account_setup_warning_modal: None,
            no_welcome_modal: None,
            nsfw_policy: None,
            role: None,
            theme: None,
            username: None,
            video_channels: None,
            video_quota: None,
            video_quota_daily: None,
            p2p_enabled: None,
        }
    }
}

