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
    GetClosedTradesReport,
    GetGroups,
    GetAccountsReport,
    GetApiStatus,
    IsApiAlive,
    GetTradesReport,
    GetAssets,
    GetOrders,
    CancelOrder,
    Deposit,
    Withdraw,
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
            BrandApiEndpoint::GetClosedTradesReport => {
                format!(
                    "/{api_name}/{}/reports/close-trades-history-report",
                    api_version
                )
            }
            BrandApiEndpoint::GetGroups => {
                format!("/{api_name}/{api_version}/groups/all")
            }
            BrandApiEndpoint::GetAccountsReport => {
                format!("/{api_name}/{api_version}/reports/account-statement-report",)
            }
            BrandApiEndpoint::GetApiStatus => {
                format!("/{api_name}/ready")
            }
            BrandApiEndpoint::IsApiAlive => {
                format!("/{api_name}/alive")
            }
            BrandApiEndpoint::GetTradesReport => {
                format!("/{api_name}/{api_version}/reports/trades-history-report")
            }
            BrandApiEndpoint::GetAssets => {
                format!("/{api_name}/{api_version}/brand/assets")
            }
            BrandApiEndpoint::GetOrders => {
                format!("/{api_name}/{api_version}/orders/all")
            }
            BrandApiEndpoint::CancelOrder => {
                format!("/{api_name}/{api_version}/orders/cancel")
            }
            BrandApiEndpoint::Deposit => {
                format!("/{api_name}/{api_version}/account-operations/deposit")
            }
            BrandApiEndpoint::Withdraw => {
                format!("/{api_name}/{api_version}/account-operations/withdraw")
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
            BrandApiEndpoint::GetClosedTradesReport => Method::POST,
            BrandApiEndpoint::GetGroups => Method::POST,
            BrandApiEndpoint::GetAccountsReport => Method::POST,
            BrandApiEndpoint::GetApiStatus => Method::GET,
            BrandApiEndpoint::IsApiAlive => Method::GET,
            BrandApiEndpoint::GetTradesReport => Method::POST,
            BrandApiEndpoint::GetAssets => Method::POST,
            BrandApiEndpoint::GetOrders => Method::POST,
            BrandApiEndpoint::CancelOrder => Method::POST,
            BrandApiEndpoint::Deposit => Method::POST,
            BrandApiEndpoint::Withdraw => Method::POST,
        }
    }
}
