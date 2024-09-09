use crate::brand::endpoints::BrandApiEndpoint;
use crate::brand::errors::Error;
use crate::brand::models::CreateUserRequest;
use crate::brand::{
    CreateUserResponse
};
use error_chain::bail;
use http::{Method, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub struct BrandApiConfig {
    pub api_url: String,
    pub api_key: String,
}

pub struct BrandApiClient {
    config: BrandApiConfig,
    inner_client: reqwest::Client,
}

impl BrandApiClient {
    pub fn new(config: BrandApiConfig) -> Self {
        Self {
            config,
            inner_client: reqwest::Client::new(),
        }
    }

    pub async fn create_user(
        &self,
        request: &CreateUserRequest,
    ) -> Result<CreateUserResponse, Error> {
        let endpoint = BrandApiEndpoint::CreateUser;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn send_deserialized<R: Serialize, T: DeserializeOwned + Debug>(
        &self,
        endpoint: BrandApiEndpoint,
        request: Option<&R>,
    ) -> Result<T, Error> {
        let base_url = &self.config.api_url;
        let (builder, url, request) = self.get_builder(base_url, endpoint, request)?;
        let response = builder.send().await;

        handle_json(response?, request, &url, endpoint.get_http_method()).await
    }

    pub async fn send<R: Serialize>(
        &self,
        endpoint: BrandApiEndpoint,
        request: Option<&R>,
    ) -> Result<String, Error> {
        let base_url = &self.config.api_url;
        let (builder, url, request) = self.get_builder(base_url, endpoint, request)?;
        let response = builder.send().await;

        handle_text(response?, &request, &url, endpoint.get_http_method()).await
    }

    fn get_builder<R: Serialize>(
        &self,
        base_url: &str,
        endpoint: BrandApiEndpoint,
        request: Option<&R>,
    ) -> Result<(RequestBuilder, String, Option<String>), Error> {
        let headers = self.build_headers();
        let http_method = endpoint.get_http_method();
        let mut request_json = None;

        let url = if http_method == Method::GET {
            let query_string = serde_qs::to_string(&request).expect("must be valid model");
            self.build_full_url(base_url, &endpoint, Some(query_string))
        } else {
            self.build_full_url(base_url, &endpoint, None)
        };

        let mut builder = self.inner_client.request(http_method, &url);

        if let Some(request) = request {
            let body = serde_json::to_string(request)?;
            request_json = Some(body.clone());
            builder = builder.body(body);
        }

        Ok((builder.headers(headers), url, request_json))
    }

    fn build_headers(&self) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();
        let json_content_str = "application/json";

        custom_headers.insert(
            "Content-Type",
            HeaderValue::from_str(json_content_str).unwrap(),
        );
        custom_headers.insert("Accept", HeaderValue::from_str(json_content_str).unwrap());
        custom_headers.insert("brand-api-key", self.config.api_key.parse().unwrap());

        custom_headers
    }

    pub fn build_query_string(&self, params: Vec<(&str, &str)>) -> String {
        let mut query_string = String::new();

        for (key, value) in params {
            let param = format!("{key}={value}&");
            query_string.push_str(&param);
        }

        query_string.pop(); // remove last & symbol

        query_string
    }

    fn build_full_url(
        &self,
        base_url: &str,
        endpoint: &BrandApiEndpoint,
        query_string: Option<String>,
    ) -> String {
        let endpoint_str = String::from(endpoint);

        if let Some(query_string) = query_string {
            format!("{base_url}{endpoint_str}?{query_string}")
        } else {
            format!("{base_url}{endpoint_str}")
        }
    }
}

async fn handle_json<T: DeserializeOwned + Debug>(
    response: Response,
    request_json: Option<String>,
    request_url: &str,
    request_method: Method,
) -> Result<T, Error> {
    let text = handle_text(response, &request_json, request_url, request_method).await?;
    let result: Result<T, _> = serde_json::from_str(&text);

    let Ok(body) = result else {
        bail!(
            "Failed to deserialize body. Url: {}.  Error: {:?}. Body: {}",
            request_url,
            result.unwrap_err(),
            text
        );
    };

    Ok(body)
}

async fn handle_text(
    response: Response,
    request_json: &Option<String>,
    request_url: &str,
    request_method: Method,
) -> Result<String, Error> {
    match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => {
            let result: Result<String, _> = response.text().await;

            let Ok(text) = result else {
                bail!(format!(
                    "Failed to read response body. Url: {request_method:?} {request_url}"
                ));
            };

            Ok(text)
        }
        StatusCode::INTERNAL_SERVER_ERROR => {
            bail!(format!(
                "Internal Server Error. Url: {request_method:?} {request_url}"
            ));
        }
        StatusCode::SERVICE_UNAVAILABLE => {
            bail!(format!(
                "Service Unavailable. Url: {request_method:?} {request_url}"
            ));
        }
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
            bail!(format!(
                "Unauthorized or forbidden. Url: {request_method:?} {request_url}"
            ));
        }
        StatusCode::BAD_REQUEST => {
            let error = response.text().await?;
            bail!(format!(
                "Received bad request status. Url: {request_method:?} {request_url}. Request: {request_json:?}. Response: {error:?}"
            ));
        }
        code => {
            let error = response.text().await?;
            bail!(format!("Received response code: {code:?}. Url: {request_method:?} {request_url}. Request: {request_json:?} Response: {error:?}"));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {}
}
