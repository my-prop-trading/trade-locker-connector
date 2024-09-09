use http::Method;

#[derive(Clone, Copy, Debug)]
pub enum BrandApiEndpoint {
    CreateUser,
    CheckEmail,
    SetUserPassword,
    GetAccount,
    CreateAccount,
    ActivateAccount,
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
            BrandApiEndpoint::ActivateAccount => Method::PUT
        }
    }
}
