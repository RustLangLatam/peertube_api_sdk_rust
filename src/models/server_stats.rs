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
pub struct ServerStats {
    #[serde(rename = "totalUsers", skip_serializing_if = "Option::is_none")]
    pub total_users: Option<f64>,
    #[serde(rename = "totalDailyActiveUsers", skip_serializing_if = "Option::is_none")]
    pub total_daily_active_users: Option<f64>,
    #[serde(rename = "totalWeeklyActiveUsers", skip_serializing_if = "Option::is_none")]
    pub total_weekly_active_users: Option<f64>,
    #[serde(rename = "totalMonthlyActiveUsers", skip_serializing_if = "Option::is_none")]
    pub total_monthly_active_users: Option<f64>,
    /// **PeerTube >= 6.1** Value is null if the admin disabled total moderators stats
    #[serde(rename = "totalModerators", skip_serializing_if = "Option::is_none")]
    pub total_moderators: Option<f64>,
    /// **PeerTube >= 6.1** Value is null if the admin disabled total admins stats
    #[serde(rename = "totalAdmins", skip_serializing_if = "Option::is_none")]
    pub total_admins: Option<f64>,
    #[serde(rename = "totalLocalVideos", skip_serializing_if = "Option::is_none")]
    pub total_local_videos: Option<f64>,
    /// Total video views made on the instance
    #[serde(rename = "totalLocalVideoViews", skip_serializing_if = "Option::is_none")]
    pub total_local_video_views: Option<f64>,
    /// Total comments made by local users
    #[serde(rename = "totalLocalVideoComments", skip_serializing_if = "Option::is_none")]
    pub total_local_video_comments: Option<f64>,
    #[serde(rename = "totalLocalVideoFilesSize", skip_serializing_if = "Option::is_none")]
    pub total_local_video_files_size: Option<f64>,
    #[serde(rename = "totalVideos", skip_serializing_if = "Option::is_none")]
    pub total_videos: Option<f64>,
    #[serde(rename = "totalVideoComments", skip_serializing_if = "Option::is_none")]
    pub total_video_comments: Option<f64>,
    #[serde(rename = "totalLocalVideoChannels", skip_serializing_if = "Option::is_none")]
    pub total_local_video_channels: Option<f64>,
    #[serde(rename = "totalLocalDailyActiveVideoChannels", skip_serializing_if = "Option::is_none")]
    pub total_local_daily_active_video_channels: Option<f64>,
    #[serde(rename = "totalLocalWeeklyActiveVideoChannels", skip_serializing_if = "Option::is_none")]
    pub total_local_weekly_active_video_channels: Option<f64>,
    #[serde(rename = "totalLocalMonthlyActiveVideoChannels", skip_serializing_if = "Option::is_none")]
    pub total_local_monthly_active_video_channels: Option<f64>,
    #[serde(rename = "totalLocalPlaylists", skip_serializing_if = "Option::is_none")]
    pub total_local_playlists: Option<f64>,
    #[serde(rename = "totalInstanceFollowers", skip_serializing_if = "Option::is_none")]
    pub total_instance_followers: Option<f64>,
    #[serde(rename = "totalInstanceFollowing", skip_serializing_if = "Option::is_none")]
    pub total_instance_following: Option<f64>,
    #[serde(rename = "videosRedundancy", skip_serializing_if = "Option::is_none")]
    pub videos_redundancy: Option<Vec<models::ServerStatsVideosRedundancyInner>>,
    #[serde(rename = "totalActivityPubMessagesProcessed", skip_serializing_if = "Option::is_none")]
    pub total_activity_pub_messages_processed: Option<f64>,
    #[serde(rename = "totalActivityPubMessagesSuccesses", skip_serializing_if = "Option::is_none")]
    pub total_activity_pub_messages_successes: Option<f64>,
    #[serde(rename = "totalActivityPubMessagesErrors", skip_serializing_if = "Option::is_none")]
    pub total_activity_pub_messages_errors: Option<f64>,
    #[serde(rename = "activityPubMessagesProcessedPerSecond", skip_serializing_if = "Option::is_none")]
    pub activity_pub_messages_processed_per_second: Option<f64>,
    #[serde(rename = "totalActivityPubMessagesWaiting", skip_serializing_if = "Option::is_none")]
    pub total_activity_pub_messages_waiting: Option<f64>,
    /// **PeerTube >= 6.1** Value is null if the admin disabled registration requests stats
    #[serde(rename = "averageRegistrationRequestResponseTimeMs", skip_serializing_if = "Option::is_none")]
    pub average_registration_request_response_time_ms: Option<f64>,
    /// **PeerTube >= 6.1** Value is null if the admin disabled registration requests stats
    #[serde(rename = "totalRegistrationRequestsProcessed", skip_serializing_if = "Option::is_none")]
    pub total_registration_requests_processed: Option<f64>,
    /// **PeerTube >= 6.1** Value is null if the admin disabled registration requests stats
    #[serde(rename = "totalRegistrationRequests", skip_serializing_if = "Option::is_none")]
    pub total_registration_requests: Option<f64>,
    /// **PeerTube >= 6.1** Value is null if the admin disabled abuses stats
    #[serde(rename = "averageAbuseResponseTimeMs", skip_serializing_if = "Option::is_none")]
    pub average_abuse_response_time_ms: Option<f64>,
    /// **PeerTube >= 6.1** Value is null if the admin disabled abuses stats
    #[serde(rename = "totalAbusesProcessed", skip_serializing_if = "Option::is_none")]
    pub total_abuses_processed: Option<f64>,
    /// **PeerTube >= 6.1** Value is null if the admin disabled abuses stats
    #[serde(rename = "totalAbuses", skip_serializing_if = "Option::is_none")]
    pub total_abuses: Option<f64>,
}

impl ServerStats {
    pub fn new() -> ServerStats {
        ServerStats {
            total_users: None,
            total_daily_active_users: None,
            total_weekly_active_users: None,
            total_monthly_active_users: None,
            total_moderators: None,
            total_admins: None,
            total_local_videos: None,
            total_local_video_views: None,
            total_local_video_comments: None,
            total_local_video_files_size: None,
            total_videos: None,
            total_video_comments: None,
            total_local_video_channels: None,
            total_local_daily_active_video_channels: None,
            total_local_weekly_active_video_channels: None,
            total_local_monthly_active_video_channels: None,
            total_local_playlists: None,
            total_instance_followers: None,
            total_instance_following: None,
            videos_redundancy: None,
            total_activity_pub_messages_processed: None,
            total_activity_pub_messages_successes: None,
            total_activity_pub_messages_errors: None,
            activity_pub_messages_processed_per_second: None,
            total_activity_pub_messages_waiting: None,
            average_registration_request_response_time_ms: None,
            total_registration_requests_processed: None,
            total_registration_requests: None,
            average_abuse_response_time_ms: None,
            total_abuses_processed: None,
            total_abuses: None,
        }
    }
}

