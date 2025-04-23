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

/// struct for typed errors of method [`api_v1_users_me_subscriptions_exist_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1UsersMeSubscriptionsExistGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_users_me_subscriptions_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1UsersMeSubscriptionsGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_users_me_subscriptions_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1UsersMeSubscriptionsPostError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_users_me_subscriptions_subscription_handle_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1UsersMeSubscriptionsSubscriptionHandleDeleteError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_users_me_subscriptions_subscription_handle_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1UsersMeSubscriptionsSubscriptionHandleGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`api_v1_users_me_subscriptions_videos_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiV1UsersMeSubscriptionsVideosGetError {
    UnknownValue(serde_json::Value),
}

pub struct MySubscriptionsApi {
    pub(crate) cfg: Box<configuration::Configuration>,
}

impl MySubscriptionsApi {
    pub fn new(configuration: configuration::Configuration) -> Self {
        Self {
            cfg: Box::new(configuration),
        }
    }

    pub async fn api_v1_users_me_subscriptions_exist_get(
        &self,
        uris: Vec<String>,
    ) -> Result<serde_json::Value, Error<ApiV1UsersMeSubscriptionsExistGetError>> {
        let local_var_configuration = &self.cfg;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/users/me/subscriptions/exist",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        local_var_req_builder = match "multi" {
            "multi" => local_var_req_builder.query(
                &uris
                    .into_iter()
                    .map(|p| ("uris".to_owned(), p.to_string()))
                    .collect::<Vec<(std::string::String, std::string::String)>>(),
            ),
            _ => local_var_req_builder.query(&[(
                "uris",
                &uris
                    .into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]),
        };
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
            let local_var_entity: Option<ApiV1UsersMeSubscriptionsExistGetError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn api_v1_users_me_subscriptions_get(
        &self,
        start: Option<i32>,
        count: Option<i32>,
        sort: Option<&str>,
    ) -> Result<models::VideoChannelList, Error<ApiV1UsersMeSubscriptionsGetError>> {
        let local_var_configuration = &self.cfg;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/users/me/subscriptions",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
            let local_var_entity: Option<ApiV1UsersMeSubscriptionsGetError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn api_v1_users_me_subscriptions_post(
        &self,
        api_v1_users_me_subscriptions_post_request: Option<
            models::ApiV1UsersMeSubscriptionsPostRequest,
        >,
    ) -> Result<(), Error<ApiV1UsersMeSubscriptionsPostError>> {
        let local_var_configuration = &self.cfg;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/users/me/subscriptions",
            local_var_configuration.base_path
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
        local_var_req_builder =
            local_var_req_builder.json(&api_v1_users_me_subscriptions_post_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<ApiV1UsersMeSubscriptionsPostError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn api_v1_users_me_subscriptions_subscription_handle_delete(
        &self,
        subscription_handle: &str,
    ) -> Result<(), Error<ApiV1UsersMeSubscriptionsSubscriptionHandleDeleteError>> {
        let local_var_configuration = &self.cfg;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/users/me/subscriptions/{subscriptionHandle}",
            local_var_configuration.base_path,
            subscriptionHandle = crate::apis::urlencode(subscription_handle)
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
            let local_var_entity: Option<ApiV1UsersMeSubscriptionsSubscriptionHandleDeleteError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn api_v1_users_me_subscriptions_subscription_handle_get(
        &self,
        subscription_handle: &str,
    ) -> Result<models::VideoChannel, Error<ApiV1UsersMeSubscriptionsSubscriptionHandleGetError>>
    {
        let local_var_configuration = &self.cfg;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/users/me/subscriptions/{subscriptionHandle}",
            local_var_configuration.base_path,
            subscriptionHandle = crate::apis::urlencode(subscription_handle)
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
            let local_var_entity: Option<ApiV1UsersMeSubscriptionsSubscriptionHandleGetError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    pub async fn api_v1_users_me_subscriptions_videos_get(
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
    ) -> Result<models::VideoListResponse, Error<ApiV1UsersMeSubscriptionsVideosGetError>> {
        let local_var_configuration = &self.cfg;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v1/users/me/subscriptions/videos",
            local_var_configuration.base_path
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
            let local_var_entity: Option<ApiV1UsersMeSubscriptionsVideosGetError> =
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
