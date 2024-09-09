use http::Method;

#[derive(Clone, Copy, Debug)]
pub enum WebservicesApiEndpoint {
    CreateManagerToken,
    CreateCtid,
    CreateTrader,
    LinkCtid,
    /// Changes of a trader entity (including allocating/removing credit).
    /// Requires trader {login}
    UpdateTrader(i64),
    /// Changes the balance of a trader entity (including allocating/removing credit).
    /// Requires trader {login}
    UpdateTraderBalance(i64),
    GetTraders,
    /// Reads the details of an existing trader entity.
    /// Requires trader {login}
    GetTrader(i64),
    GetClosedPositions,
    GetOpenedPositions,
    GetTraderGroups,
    GetSymbols,
}

impl From<&WebservicesApiEndpoint> for String {
    fn from(item: &WebservicesApiEndpoint) -> Self {
        let api_version = "v2";

        match item {
            WebservicesApiEndpoint::CreateManagerToken => {
                format!("/{}/webserv/managers/token", api_version)
            }
            WebservicesApiEndpoint::CreateCtid => "/cid/ctid/create".to_string(),
            WebservicesApiEndpoint::CreateTrader => format!("/{api_version}/webserv/traders"),
            WebservicesApiEndpoint::LinkCtid => "/cid/ctid/link".to_string(),
            WebservicesApiEndpoint::UpdateTrader(login) => {
                format!("/{api_version}/webserv/traders/{login}")
            }
            WebservicesApiEndpoint::UpdateTraderBalance(login) => {
                format!("/{api_version}/webserv/traders/{login}/changebalance")
            }
            WebservicesApiEndpoint::GetTraders => {
                format!("/{api_version}/webserv/traders/")
            }
            WebservicesApiEndpoint::GetClosedPositions => {
                format!("/{api_version}/webserv/closedPositions")
            }
            WebservicesApiEndpoint::GetTraderGroups => {
                format!("/{api_version}/webserv/tradergroups")
            }
            WebservicesApiEndpoint::GetSymbols => {
                format!("/{api_version}/webserv/symbols")
            }
            WebservicesApiEndpoint::GetTrader(login) => {
                format!("/{api_version}/webserv/traders/{login}")
            }
            WebservicesApiEndpoint::GetOpenedPositions => {
                format!("/{api_version}/webserv/openPositions")
            }
        }
    }
}

impl WebservicesApiEndpoint {
    pub fn get_http_method(&self) -> Method {
        match &self {
            WebservicesApiEndpoint::CreateManagerToken => Method::POST,
            WebservicesApiEndpoint::CreateCtid => Method::POST,
            WebservicesApiEndpoint::CreateTrader => Method::POST,
            WebservicesApiEndpoint::LinkCtid => Method::POST,
            WebservicesApiEndpoint::UpdateTrader(_) => Method::PATCH,
            WebservicesApiEndpoint::UpdateTraderBalance(_) => Method::POST,
            WebservicesApiEndpoint::GetTraders => Method::GET,
            WebservicesApiEndpoint::GetClosedPositions => Method::GET,
            WebservicesApiEndpoint::GetTraderGroups => Method::GET,
            WebservicesApiEndpoint::GetSymbols => Method::GET,
            WebservicesApiEndpoint::GetTrader(_) => Method::GET,
            WebservicesApiEndpoint::GetOpenedPositions => Method::GET,
        }
    }
}
