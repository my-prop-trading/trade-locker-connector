use trade_locker_connector::brand::api_client::{BrandApiClient, BrandApiConfig};
use trade_locker_connector::brand::{
    AccountType, CloseAccountPositionsRequest, CreateAccountRequest, CreateUserRequest,
    CreditAccountRequest, UpdateAccountStatusRequest,
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
    close_account_positions(&brand_api).await;
}

pub async fn create_user(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .create_user(&CreateUserRequest {
            email: "trade-locker-test123@mailinator.com".to_string(),
            password: "Qwerty123!".to_string(),
            first_name: Some("test".to_string()),
            last_name: Some("test".to_string()),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn create_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .create_account(&CreateAccountRequest {
            user_id: "63f3c61e-e11a-495c-82a4-003b244e8434".to_string(),
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
            account_id: "L#705322".to_string(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn credit_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .credit_account(&CreditAccountRequest {
            account_id: "L#705322".to_string(),
            amount: "10000".to_string(),
            note: None,
        })
        .await;

    println!("{:?}", resp)
}

pub async fn close_account_positions(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .close_account_positions(&CloseAccountPositionsRequest {
            account_id: "L#705322".to_string(),
        })
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
