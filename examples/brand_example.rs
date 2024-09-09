use trade_locker_connector::brand::api_client::{BrandApiClient, BrandApiConfig};
use trade_locker_connector::brand::CreateUserRequest;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("TRADE_LOCKER_API_KEY").unwrap();
    let config = BrandApiConfig::new_dev(api_key);
    let brand_api = BrandApiClient::new(config);
    create_user(&brand_api).await;
}

pub async fn create_user(rest_client: &BrandApiClient) {
    let resp = rest_client.create_user(&CreateUserRequest {
        email: "trade-locker-test123@mailinator.com".to_string(),
        password: "Qwerty123!".to_string(),
        first_name: Some("test".to_string()),
        last_name: Some("test".to_string()),
    }).await;

    println!("{:?}", resp)
}
