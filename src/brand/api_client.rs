use crate::brand::endpoints::BrandApiEndpoint;
use crate::brand::errors::Error;
use crate::brand::models::CreateUserRequest;
use crate::brand::{AccountModel, CheckEmailRequest, CheckEmailResponse, CloseAccountPositionsRequest, CloseAccountPositionsResponse, CreateAccountRequest, CreateUserResponse, CreditAccountRequest, CreditAccountResponse, GetAccountRequest, GetAccountsReportRequest, GetAccountsReportResponse, GetClosedTradesReportRequest, GetClosedTradesReportResponse, GetGroupsRequest, GetGroupsResponse, GetInstrumentsRequest, GetInstrumentsResponse, GetOpenedPositionsRequest, GetOpenedPositionsResponse, SetAccountGroupRequest, SetUserPasswordRequest, UpdateAccountStatusRequest, UpdateAccountStatusResponse};
use error_chain::bail;
use http::{Method, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait BrandApiConfig {
    async fn get_api_url(&self) -> String;
    async fn get_api_key(&self) -> String;
}

pub struct BrandApiClient<C: BrandApiConfig> {
    config: C,
    inner_client: reqwest::Client,
}

impl<C: BrandApiConfig> BrandApiClient<C> {
    pub fn new(config: C) -> Self {
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

    pub async fn check_email(
        &self,
        request: &CheckEmailRequest,
    ) -> Result<CheckEmailResponse, Error> {
        let endpoint = BrandApiEndpoint::CheckEmail;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn set_user_password(&self, request: &SetUserPasswordRequest) -> Result<(), Error> {
        let endpoint = BrandApiEndpoint::CheckEmail;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn get_account(&self, request: &GetAccountRequest) -> Result<AccountModel, Error> {
        let endpoint = BrandApiEndpoint::GetAccount;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn create_account(
        &self,
        request: &CreateAccountRequest,
    ) -> Result<AccountModel, Error> {
        let endpoint = BrandApiEndpoint::CreateAccount;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn activate_account(
        &self,
        request: &UpdateAccountStatusRequest,
    ) -> Result<UpdateAccountStatusResponse, Error> {
        let endpoint = BrandApiEndpoint::ActivateAccount;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Restricts an existing TradeLocker account. Restricted accounts cannot open positions.
    pub async fn restrict_account(
        &self,
        request: &UpdateAccountStatusRequest,
    ) -> Result<UpdateAccountStatusResponse, Error> {
        let endpoint = BrandApiEndpoint::RestrictAccount;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Suspend an existing TradeLocker account.
    /// Trading is prohibited for suspended accounts and they do not show up in the TradeLocker application.
    pub async fn suspend_account(
        &self,
        request: &UpdateAccountStatusRequest,
    ) -> Result<UpdateAccountStatusResponse, Error> {
        let endpoint = BrandApiEndpoint::SuspendAccount;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn set_account_group(&self, request: &SetAccountGroupRequest) -> Result<(), Error> {
        let endpoint = BrandApiEndpoint::SetAccountGroup;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn close_account_positions(
        &self,
        request: &CloseAccountPositionsRequest,
    ) -> Result<CloseAccountPositionsResponse, Error> {
        let endpoint = BrandApiEndpoint::CloseAccountPositions;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Add or remove credit to specified account of a user.
    /// The ID of the resulting operation is not unique across account types (LIVE,DEMO).
    pub async fn credit_account(
        &self,
        request: &CreditAccountRequest,
    ) -> Result<CreditAccountResponse, Error> {
        let endpoint = BrandApiEndpoint::CreditAccount;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn get_instruments(
        &self,
        request: &GetInstrumentsRequest,
    ) -> Result<GetInstrumentsResponse, Error> {
        let endpoint = BrandApiEndpoint::GetInstruments;
        self.send_deserialized(endpoint, Some(request)).await
    }

    /// Get all open positions. Positions are sorted by open timestamp in reverse-chronological order,
    /// so that the first position is the most recently opened one.
    pub async fn get_opened_positions(
        &self,
        request: &GetOpenedPositionsRequest,
    ) -> Result<GetOpenedPositionsResponse, Error> {
        let endpoint = BrandApiEndpoint::GetOpenedPositions;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn get_closed_trades_report(
        &self,
        request: &GetClosedTradesReportRequest,
    ) -> Result<GetClosedTradesReportResponse, Error> {
        let endpoint = BrandApiEndpoint::GetClosedTradesReport;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn get_groups(&self, request: &GetGroupsRequest) -> Result<GetGroupsResponse, Error> {
        let endpoint = BrandApiEndpoint::GetGroups;
        self.send_deserialized(endpoint, Some(request)).await
    }

    pub async fn get_accounts_report(&self, request: &GetAccountsReportRequest) -> Result<GetAccountsReportResponse, Error> {
        let endpoint = BrandApiEndpoint::GetAccountsReport;
        self.send_deserialized(endpoint, Some(request)).await
    }

    async fn send_deserialized<R: Serialize, T: DeserializeOwned + Debug>(
        &self,
        endpoint: BrandApiEndpoint,
        request: Option<&R>,
    ) -> Result<T, Error> {
        let base_url = &self.config.get_api_url().await;
        let (builder, url, request) = self.get_builder(base_url, endpoint, request).await?;
        let response = builder.send().await;

        handle_json(response?, request, &url, endpoint.get_http_method()).await
    }

    async fn get_builder<R: Serialize>(
        &self,
        base_url: &str,
        endpoint: BrandApiEndpoint,
        request: Option<&R>,
    ) -> Result<(RequestBuilder, String, Option<String>), Error> {
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
        let headers = self.build_headers().await;

        Ok((builder.headers(headers), url, request_json))
    }

    async fn build_headers(&self) -> HeaderMap {
        let mut custom_headers = HeaderMap::new();
        let json_content_str = "application/json";

        custom_headers.insert(
            "Content-Type",
            HeaderValue::from_str(json_content_str).unwrap(),
        );
        custom_headers.insert("Accept", HeaderValue::from_str(json_content_str).unwrap());
        custom_headers.insert("brand-api-key", self.config.get_api_key().await.parse().unwrap());

        custom_headers
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
