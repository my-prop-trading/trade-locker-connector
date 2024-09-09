use crate::brand::endpoints::WebservicesApiEndpoint;
use crate::brand::errors::Error;
use crate::brand::models::{
    CreateCtidRequest, CreateCtidResponse, CreateCtraderManagerTokenRequest,
    CreateCtraderManagerTokenResponse, CreateTraderRequest,
};
use crate::brand::{
    CreateTraderResponse, 
    GetSymbolsResponse, GetTraderGroupsResponse, GetTradersRequest,
    GetTradersResponse, LinkCtidRequest, LinkCtidResponse,  SymbolModel,
    TraderGroupModel, TraderModel, UpdateTraderBalanceRequest, UpdateTraderBalanceResponse,
    UpdateTraderRequest,
};
use crate::models::ManagerCreds;
use crate::utils::generate_password_hash;
use error_chain::bail;
use http::{Method, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait WebservicesApiConfig {
    async fn get_url(&self) -> String;
}

/// A simple yet powerful RESTful API, designed to cover the basic integration requirements for CRM
/// systems. It offers the capability to handle common CRM related tasks, such as the creation and
/// updates of users and trading accounts, and performing deposits and withdrawals to those accounts.
pub struct WebservicesApiClient<C: WebservicesApiConfig> {
    config: C,
    inner_client: reqwest::Client,
    creds: Arc<dyn ManagerCreds + Send + Sync>,
    auth_token: std::sync::RwLock<Option<String>>,
}

impl<C: WebservicesApiConfig> WebservicesApiClient<C> {
    pub fn new(config: C, creds: Arc<dyn ManagerCreds + Send + Sync>) -> Self {
        Self {
            config,
            inner_client: reqwest::Client::new(),
            creds,
            auth_token: std::sync::RwLock::new(None),
        }
    }

    pub fn clear_token(&self) {
        let _ = self.auth_token.write().unwrap().take();
    }

    pub fn is_authorized(&self) -> bool {
        self.auth_token.read().unwrap().is_some()
    }

    /// Gets the list of all available symbols on the server.
    pub async fn get_symbols(&self) -> Result<Vec<SymbolModel>, Error> {
        let request: Option<&String> = None;
        let endpoint = WebservicesApiEndpoint::GetSymbols;
        let resp: GetSymbolsResponse = self.send_deserialized(endpoint, request).await?;

        Ok(resp.items)
    }

    /// Gets a list of all trader groups.
    pub async fn get_trader_groups(&self) -> Result<Vec<TraderGroupModel>, Error> {
        let request: Option<&String> = None;
        let endpoint = WebservicesApiEndpoint::GetTraderGroups;
        let resp: GetTraderGroupsResponse = self.send_deserialized(endpoint, request).await?;

        Ok(resp.items)
    }

    pub async fn get_traders(
        &self,
        request: &GetTradersRequest,
    ) -> Result<Vec<TraderModel>, Error> {
        let endpoint = WebservicesApiEndpoint::GetTraders;
        let resp: GetTradersResponse = self.send_deserialized(endpoint, Some(request)).await?;

        Ok(resp.items)
    }

    pub async fn get_trader(&self, login: i64) -> Result<TraderModel, Error> {
        let request: Option<&String> = None;
        let endpoint = WebservicesApiEndpoint::GetTrader(login);

        self.send_deserialized(endpoint, request).await
    }

    /// Changes the balance of a trader entity (including allocating/removing credit).
    pub async fn update_trader_balance(
        &self,
        request: &UpdateTraderBalanceRequest,
    ) -> Result<UpdateTraderBalanceResponse, Error> {
        let endpoint = WebservicesApiEndpoint::UpdateTraderBalance(request.login);
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Updates a trader entity.
    pub async fn update_trader(
        &self,
        login: i64,
        request: &UpdateTraderRequest,
    ) -> Result<(), Error> {
        let endpoint = WebservicesApiEndpoint::UpdateTrader(login);
        let _ = self.send(endpoint, Some(request)).await?;

        Ok(())
    }

    /// Links a trader entity to a user entity.
    pub async fn link_ctid(&self, request: &LinkCtidRequest) -> Result<LinkCtidResponse, Error> {
        let endpoint = WebservicesApiEndpoint::LinkCtid;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Creates a new trader (e.g. account)entity.
    pub async fn create_trader(
        &self,
        request: &CreateTraderRequest,
    ) -> Result<CreateTraderResponse, Error> {
        let endpoint = WebservicesApiEndpoint::CreateTrader;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Creates a new user entity. The cTID is used to authorize end users in the trading application(s) of their choice
    pub async fn create_ctid(
        &self,
        request: &CreateCtidRequest,
    ) -> Result<CreateCtidResponse, Error> {
        let endpoint = WebservicesApiEndpoint::CreateCtid;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Creates a token and stores it internally for the next requests
    pub async fn authorize(&self) -> Result<(), Error> {
        let resp = self.create_token().await?;
        let mut token_lock = self.auth_token.write().unwrap();
        *token_lock = Some(resp.token);

        Ok(())
    }

    pub async fn create_token(&self) -> Result<CreateCtraderManagerTokenResponse, Error> {
        let request = CreateCtraderManagerTokenRequest {
            login: self.creds.get_login().await,
            hashed_password: generate_password_hash(&self.creds.get_password().await),
        };
        let endpoint = WebservicesApiEndpoint::CreateManagerToken;

        self.send_deserialized(endpoint, Some(&request)).await
    }

    pub async fn send_deserialized<R: Serialize, T: DeserializeOwned + Debug>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
    ) -> Result<T, Error> {
        let token = self.get_token_cloned();
        let base_url = self.config.get_url().await;
        let (builder, url, request) = self.get_builder(&base_url, endpoint, request, &token)?;
        let response = builder.send().await;

        handle_json(response?, request, &url, endpoint.get_http_method()).await
    }

    pub async fn send<R: Serialize>(
        &self,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
    ) -> Result<String, Error> {
        let token = self.get_token_cloned();
        let base_url = self.config.get_url().await;
        let (builder, url, request) = self.get_builder(&base_url, endpoint, request, &token)?;
        let response = builder.send().await;

        handle_text(response?, &request, &url, endpoint.get_http_method()).await
    }

    fn get_builder<R: Serialize>(
        &self,
        base_url: &str,
        endpoint: WebservicesApiEndpoint,
        request: Option<&R>,
        token: &Option<String>,
    ) -> Result<(RequestBuilder, String, Option<String>), Error> {
        let headers = self.build_headers();
        let http_method = endpoint.get_http_method();
        let mut request_json = None;

        let url = if http_method == Method::GET {
            let query_string = serde_qs::to_string(&request).expect("must be valid model");
            self.build_full_url(base_url, &endpoint, Some(query_string), token)
        } else {
            self.build_full_url(base_url, &endpoint, None, token)
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
        endpoint: &WebservicesApiEndpoint,
        query_string: Option<String>,
        token: &Option<String>,
    ) -> String {
        let endpoint_str = String::from(endpoint);

        if let Some(token) = token {
            let token_param_name = "token";

            if let Some(query_string) = query_string {
                format!("{base_url}{endpoint_str}?{query_string}&{token_param_name}={token}")
            } else {
                format!("{base_url}{endpoint_str}?{token_param_name}={token}")
            }
        } else {
            format!("{base_url}{endpoint_str}")
        }
    }

    fn get_token_cloned(&self) -> Option<String> {
        (*self.auth_token.read().unwrap()).clone()
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
