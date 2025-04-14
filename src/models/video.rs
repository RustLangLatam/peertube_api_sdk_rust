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
pub struct Video {
    /// object id for the video
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// universal identifier for the video, that can be used across instances
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<uuid::Uuid>,
    /// translation of a uuid v4 with a bigger alphabet to have a shorter uuid
    #[serde(rename = "shortUUID", skip_serializing_if = "Option::is_none")]
    pub short_uuid: Option<String>,
    #[serde(rename = "isLive", skip_serializing_if = "Option::is_none")]
    pub is_live: Option<bool>,
    /// time at which the video object was first drafted
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// time at which the video was marked as ready for playback (with restrictions depending on `privacy`). Usually set after a `state` evolution.
    #[serde(rename = "publishedAt", skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    /// last time the video's metadata was modified
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// used to represent a date of first publication, prior to the practical publication date of `publishedAt`
    #[serde(rename = "originallyPublishedAt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub originally_published_at: Option<Option<String>>,
    /// category in which the video is classified
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<Box<models::VideoConstantNumberCategory>>,
    /// licence under which the video is distributed
    #[serde(rename = "licence", skip_serializing_if = "Option::is_none")]
    pub licence: Option<Box<models::VideoConstantNumberLicence>>,
    /// main language used in the video
    #[serde(rename = "language", skip_serializing_if = "Option::is_none")]
    pub language: Option<Box<models::VideoConstantStringLanguage>>,
    /// privacy policy used to distribute the video
    #[serde(rename = "privacy", skip_serializing_if = "Option::is_none")]
    pub privacy: Option<Box<models::VideoPrivacyConstant>>,
    /// truncated description of the video, written in Markdown. 
    #[serde(rename = "truncatedDescription", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub truncated_description: Option<Option<String>>,
    /// duration of the video in seconds
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// **PeerTube >= 6.1** Aspect ratio of the video stream
    #[serde(rename = "aspectRatio", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<Option<f32>>,
    #[serde(rename = "isLocal", skip_serializing_if = "Option::is_none")]
    pub is_local: Option<bool>,
    /// title of the video
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "thumbnailPath", skip_serializing_if = "Option::is_none")]
    pub thumbnail_path: Option<String>,
    #[serde(rename = "previewPath", skip_serializing_if = "Option::is_none")]
    pub preview_path: Option<String>,
    #[serde(rename = "embedPath", skip_serializing_if = "Option::is_none")]
    pub embed_path: Option<String>,
    #[serde(rename = "views", skip_serializing_if = "Option::is_none")]
    pub views: Option<i32>,
    #[serde(rename = "likes", skip_serializing_if = "Option::is_none")]
    pub likes: Option<i32>,
    #[serde(rename = "dislikes", skip_serializing_if = "Option::is_none")]
    pub dislikes: Option<i32>,
    #[serde(rename = "nsfw", skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
    #[serde(rename = "waitTranscoding", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub wait_transcoding: Option<Option<bool>>,
    /// represents the internal state of the video processing within the PeerTube instance
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<models::VideoStateConstant>>,
    #[serde(rename = "scheduledUpdate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scheduled_update: Option<Option<Box<models::VideoScheduledUpdate>>>,
    #[serde(rename = "blacklisted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub blacklisted: Option<Option<bool>>,
    #[serde(rename = "blacklistedReason", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub blacklisted_reason: Option<Option<String>>,
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<models::AccountSummary>>,
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Box<models::VideoChannelSummary>>,
    #[serde(rename = "userHistory", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_history: Option<Option<Box<models::VideoUserHistory>>>,
}

impl Video {
    pub fn new() -> Video {
        Video {
            id: None,
            uuid: None,
            short_uuid: None,
            is_live: None,
            created_at: None,
            published_at: None,
            updated_at: None,
            originally_published_at: None,
            category: None,
            licence: None,
            language: None,
            privacy: None,
            truncated_description: None,
            duration: None,
            aspect_ratio: None,
            is_local: None,
            name: None,
            thumbnail_path: None,
            preview_path: None,
            embed_path: None,
            views: None,
            likes: None,
            dislikes: None,
            nsfw: None,
            wait_transcoding: None,
            state: None,
            scheduled_update: None,
            blacklisted: None,
            blacklisted_reason: None,
            account: None,
            channel: None,
            user_history: None,
        }
    }
}

