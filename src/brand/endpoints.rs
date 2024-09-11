use http::Method;

#[derive(Clone, Copy, Debug)]
pub enum BrandApiEndpoint {
    CreateUser,
    CheckEmail,
    SetUserPassword,
    GetAccount,
    CreateAccount,
    ActivateAccount,
    RestrictAccount,
    SuspendAccount,
    SetAccountGroup,
    CloseAccountPositions,
    CreditAccount,
    GetInstruments,
    GetOpenedPositions,
    GetClosedPositions,
}

impl From<&BrandApiEndpoint> for String {
    fn from(item: &BrandApiEndpoint) -> Self {
        let api_version = "v1";

        match item {
            BrandApiEndpoint::CreateUser => {
                format!("/{}/users/create", api_version)
            }
            BrandApiEndpoint::CheckEmail => {
                format!("/{}/users/check-by-email", api_version)
            }
            BrandApiEndpoint::SetUserPassword => {
                format!("/{}/users/set-password", api_version)
            }
            BrandApiEndpoint::GetAccount => {
                format!("/{}/accounts/details", api_version)
            }
            BrandApiEndpoint::CreateAccount => {
                format!("/{}/accounts/create", api_version)
            }
            BrandApiEndpoint::ActivateAccount => {
                format!("/{}/accounts/activate", api_version)
            }
            BrandApiEndpoint::RestrictAccount => {
                format!("/{}/accounts/restrict", api_version)
            }
            BrandApiEndpoint::SuspendAccount => {
                format!("/{}/accounts/suspend", api_version)
            }
            BrandApiEndpoint::SetAccountGroup => {
                format!("/{}/accounts/set-group", api_version)
            }
            BrandApiEndpoint::CloseAccountPositions => {
                format!("/{}/accounts/close-all-positions", api_version)
            }
            BrandApiEndpoint::CreditAccount => {
                format!("/{}/account-operations/credit", api_version)
            }
            BrandApiEndpoint::GetInstruments => {
                format!("/{}/brand/instruments", api_version)
            }
            BrandApiEndpoint::GetOpenedPositions => {
                format!("/{}/positions/get-open-positions", api_version)
            }
            BrandApiEndpoint::GetClosedPositions => {
                format!("/{}/reports/close-trades-history-report", api_version)
            }
        }
    }
}

impl BrandApiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            BrandApiEndpoint::CreateUser => Method::POST,
            BrandApiEndpoint::CheckEmail => Method::POST,
            BrandApiEndpoint::SetUserPassword => Method::POST,
            BrandApiEndpoint::GetAccount => Method::POST,
            BrandApiEndpoint::CreateAccount => Method::POST,
            BrandApiEndpoint::ActivateAccount => Method::PUT,
            BrandApiEndpoint::RestrictAccount => Method::PUT,
            BrandApiEndpoint::SuspendAccount => Method::PUT,
            BrandApiEndpoint::SetAccountGroup => Method::PUT,
            BrandApiEndpoint::CloseAccountPositions => Method::POST,
            BrandApiEndpoint::CreditAccount => Method::POST,
            BrandApiEndpoint::GetInstruments => Method::POST,
            BrandApiEndpoint::GetOpenedPositions => Method::POST,
            BrandApiEndpoint::GetClosedPositions => Method::POST,
        }
    }
}
