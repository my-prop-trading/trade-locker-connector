use trade_locker_connector::brand::api_client::{BrandApiClient, BrandApiConfig};
use trade_locker_connector::brand::CreateUserRequest;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("TRADE_LOCKER_API_KEY").unwrap();
    let config = ExampleBrandApiConfig {
        api_url: "https://api-dev.tradelocker.com".to_string(),
        api_key,
    };
    let brand_api = BrandApiClient::new(config);
    create_user(&brand_api).await;
}

pub async fn create_user(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client.create_user(&CreateUserRequest {
        email: "trade-locker-test123@mailinator.com".to_string(),
        password: "Qwerty123!".to_string(),
        first_name: Some("test".to_string()),
        last_name: Some("test".to_string()),
    }).await;

    println!("{:?}", resp)
}

pub struct ExampleBrandApiConfig  {
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