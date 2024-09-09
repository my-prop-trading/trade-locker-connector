use http::Method;

#[derive(Clone, Copy, Debug)]
pub enum BrandApiEndpoint {
    CreateUser,
    CheckEmail,
}

impl From<&BrandApiEndpoint> for String {
    fn from(item: &BrandApiEndpoint) -> Self {
        let api_version = "v1";

        match item {
            BrandApiEndpoint::CreateUser => {
                format!("/{}/users/create", api_version)
            }
            BrandApiEndpoint::CheckEmail => {
                format!("/{}/emails//users/check-by-email", api_version)
            }
        }
    }
}

impl BrandApiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            BrandApiEndpoint::CreateUser => Method::POST,
            BrandApiEndpoint::CheckEmail => Method::POST,
        }
    }
}
