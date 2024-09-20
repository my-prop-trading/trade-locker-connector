use trade_locker_connector::brand::api_client::{BrandApiClient, BrandApiConfig};
use trade_locker_connector::brand::{
    AccountType, CheckEmailRequest, CloseAccountPositionsRequest, CreateAccountRequest,
    CreateUserRequest, CreditAccountRequest, GetAccountRequest, GetAccountsReportRequest,
    GetClosedTradesReportRequest, GetGroupsRequest, GetInstrumentsRequest,
    GetOpenedPositionsRequest, SetUserPasswordRequest, UpdateAccountStatusRequest,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("TRADE_LOCKER_API_KEY").unwrap();
    let config = ExampleBrandApiConfig {
        api_url: "https://api-dev.tradelocker.com".to_string(),
        api_key,
    };
    let brand_api = BrandApiClient::new(config);
    //create_user(&brand_api).await;
    //create_account(&brand_api).await;
    //activate_account(&brand_api).await;
    //credit_account(&brand_api).await;
    //close_account_positions(&brand_api).await;
    //get_account(&brand_api).await;
    //get_opened_positions(&brand_api).await;
    //get_closed_positions(&brand_api).await;
    //check_email(&brand_api).await;
    //get_groups(&brand_api).await;
    //get_instruments(&brand_api).await;
    //restrict_account(&brand_api).await;
    //suspend_account(&brand_api).await;
    //get_accounts_report(&brand_api).await;
    //get_api_status(&brand_api).await;
    set_user_password(&brand_api).await
}

pub fn get_user_id() -> String {
    "63f3c61e-e11a-495c-82a4-003b244e8434".to_string()
}

pub fn get_account_id() -> String {
    "L#705322".to_string()
}

pub fn get_password() -> String {
    "Qwerty!123".to_string()
}

pub fn get_email() -> String {
    "trade-locker-test123@mailinator.com".to_string()
}

pub async fn create_user(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .create_user(&CreateUserRequest {
            email: get_email(),
            password: get_password(),
            first_name: Some("test".to_string()),
            last_name: Some("test".to_string()),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn create_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .create_account(&CreateAccountRequest {
            user_id: get_user_id(),
            account_name: "test123".to_string(),
            account_type: AccountType::Live,
            currency: "USD".to_string(),
            group_id: None,
        })
        .await;

    println!("{:?}", resp)
}

pub async fn activate_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .activate_account(&UpdateAccountStatusRequest {
            account_id: get_account_id(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn credit_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .credit_account(&CreditAccountRequest {
            account_id: get_account_id(),
            amount: "10000".to_string(),
            note: None,
        })
        .await;

    println!("{:?}", resp)
}

pub async fn close_account_positions(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .close_account_positions(&CloseAccountPositionsRequest {
            account_id: get_account_id(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn get_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_account(&GetAccountRequest {
            account_id: get_account_id(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn get_closed_positions(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_closed_trades_report(&GetClosedTradesReportRequest {
            account_id: get_account_id(),
            account_type: AccountType::Live,
            cursor: None,
            limit: None,
        })
        .await;

    println!("{:?}", resp)
}

pub async fn get_opened_positions(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_opened_positions(&GetOpenedPositionsRequest {
            account_id: get_account_id(),
            account_type: AccountType::Live,
        })
        .await;

    println!("{:?}", resp)
}

pub async fn check_email(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .check_email(&CheckEmailRequest { email: get_email() })
        .await;

    println!("{:?}", resp)
}

pub async fn get_groups(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_groups(&GetGroupsRequest {
            account_type: AccountType::Live,
        })
        .await;

    println!("{:?}", resp)
}

pub async fn get_instruments(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_instruments(&GetInstrumentsRequest {
            account_type: AccountType::Live,
        })
        .await;

    println!("{:?}", resp)
}

pub async fn restrict_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .restrict_account(&UpdateAccountStatusRequest {
            account_id: get_account_id(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn suspend_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .suspend_account(&UpdateAccountStatusRequest {
            account_id: get_account_id(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn get_accounts_report(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_accounts_report(&GetAccountsReportRequest {
            account_type: AccountType::Live,
            account_ids: None,
            account_status: None,
        })
        .await;

    println!("{:?}", resp)
}

pub async fn set_user_password(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .set_user_password(&SetUserPasswordRequest {
            user_id: get_user_id(),
            password: get_password(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn get_api_status(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_api_status()
        .await;

    println!("{:?}", resp)
}

pub struct ExampleBrandApiConfig {
    pub api_url: String,
    pub api_key: String,
}

#[async_trait::async_trait]
impl BrandApiConfig for ExampleBrandApiConfig {
    async fn get_api_url(&self) -> String {
        self.api_url.clone()
    }

    async fn get_api_key(&self) -> String {
        self.api_key.clone()
    }
}
