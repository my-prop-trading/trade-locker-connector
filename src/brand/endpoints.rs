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
    GetGroups,
}

impl From<&BrandApiEndpoint> for String {
    fn from(item: &BrandApiEndpoint) -> Self {
        let api_version = "v1";
        let api_name = "brand-api";

        match item {
            BrandApiEndpoint::CreateUser => {
                format!("/{api_name}/{}/users/create", api_version)
            }
            BrandApiEndpoint::CheckEmail => {
                format!("/{api_name}/{}/users/check-by-email", api_version)
            }
            BrandApiEndpoint::SetUserPassword => {
                format!("/{api_name}/{}/users/set-password", api_version)
            }
            BrandApiEndpoint::GetAccount => {
                format!("/{api_name}/{}/accounts/details", api_version)
            }
            BrandApiEndpoint::CreateAccount => {
                format!("/{api_name}/{}/accounts/create", api_version)
            }
            BrandApiEndpoint::ActivateAccount => {
                format!("/{api_name}/{}/accounts/activate", api_version)
            }
            BrandApiEndpoint::RestrictAccount => {
                format!("/{api_name}/{}/accounts/restrict", api_version)
            }
            BrandApiEndpoint::SuspendAccount => {
                format!("/{api_name}/{}/accounts/suspend", api_version)
            }
            BrandApiEndpoint::SetAccountGroup => {
                format!("/{api_name}/{}/accounts/set-group", api_version)
            }
            BrandApiEndpoint::CloseAccountPositions => {
                format!("/{api_name}/{}/accounts/close-all-positions", api_version)
            }
            BrandApiEndpoint::CreditAccount => {
                format!("/{api_name}/{}/account-operations/credit", api_version)
            }
            BrandApiEndpoint::GetInstruments => {
                format!("/{api_name}/{}/brand/instruments", api_version)
            }
            BrandApiEndpoint::GetOpenedPositions => {
                format!("/{api_name}/{}/positions/get-open-positions", api_version)
            }
            BrandApiEndpoint::GetClosedPositions => {
                format!("/{api_name}/{}/reports/close-trades-history-report", api_version)
            }
            BrandApiEndpoint::GetGroups => {
                format!("/{api_name}/{}/groups/all", api_version)
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
            BrandApiEndpoint::GetGroups => Method::POST,
        }
    }
}
