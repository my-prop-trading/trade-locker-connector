use chrono::{DateTime, TimeDelta, Utc};
use futures_util::future::join_all;
use std::ops::{Sub};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::Instant;
use trade_locker_connector::brand::api_client::{BrandApiClient, BrandApiConfig};
use trade_locker_connector::brand::{
    AccountOperationRequest, AccountStatus, CancelOrderRequest, CheckEmailRequest,
    CloseAccountPositionsRequest, CreateAccountRequest, CreateUserRequest, CreditAccountRequest,
    GetAccountRequest, GetAccountsReportRequest, GetAssetsRequest, GetClosedTradesReportRequest,
    GetGroupsRequest, GetInstrumentsRequest, GetOpenedPositionsRequest, GetOrdersRequest,
    GetTradesReportRequest, MonthlyActiveAccountsRequest, SetUserPasswordRequest,
    UpdateAccountStatusRequest,
};
use trade_locker_connector::models::AccountType;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("TRADE_LOCKER_API_KEY").unwrap();
    let config = ExampleBrandApiConfig {
        api_url: "https://api-dev.tradelocker.com".to_string(),
        api_key,
    };
    let brand_api = BrandApiClient::new(config);
    let instant = Instant::now();
    //load_test(&brand_api).await;
    //is_api_alive(&brand_api).await;
    //get_api_status(&brand_api).await;
    //create_user(&brand_api).await;
    //create_account(&brand_api).await;
    //activate_account(&brand_api).await;
    //credit_account(&brand_api).await;
    //close_account_positions(&brand_api).await;
    //get_account(&brand_api).await;
    get_opened_positions(&brand_api).await;
    //get_closed_positions(&brand_api).await;
    //check_email(&brand_api).await;
    //get_groups(&brand_api).await;
    //get_instruments(&brand_api).await;
    //restrict_account(&brand_api).await;
    //suspend_account(&brand_api).await;
    //get_accounts_report(&brand_api).await;
    //set_user_password(&brand_api).await
    //get_assets(&brand_api).await;
    //cancel_order(&brand_api).await;
    //get_orders(&brand_api).await;
    //deposit_account(&brand_api).await;
    //withdraw_account(&brand_api).await;
    //get_monthly_active_accounts(&brand_api).await;
    //get_trades_report(&brand_api).await;
    get_closed_trades_report(&brand_api).await;

    println!("elapsed time: {:?}", instant.elapsed());
}

pub fn get_user_id() -> String {
    //"e1ae0e5a-863e-41f2-889f-a2194f3561b5".to_string() // prod
    "226ada22-8ab4-42f4-a03a-764020f530d3".to_string() // dev
}

pub fn get_account_id() -> String {
    "L#705322".to_string()
    //"L#705611".to_string()
    //"L#705618".to_string()
    //"L#705519".to_string()
    //"L#708261".to_string()
    //"D#847500".to_string()
}

pub fn get_password() -> String {
    "Qwerty!123".to_string()
}

pub fn get_email() -> String {
    "trade-locker-test123@mailinator.com".to_string()
}

pub fn get_group_id() -> Option<String> {
    //Some("829256".to_string()) // prod PRO365-50K-1STEP
    Some("709605".to_string()) // dev
}

pub fn get_idempotency_key() -> String {
    "274944e0-3047-4c05-b04a-0f695b744589".to_string()
}

pub fn get_account_type() -> AccountType {
    AccountType::Live
}

pub async fn create_user(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .create_user(&CreateUserRequest {
            email: get_email(),
            password: get_password(),
            first_name: Some("test".to_string()),
            last_name: Some("test".to_string()),
        }, Some(&get_idempotency_key()))
        .await;

    println!("{:?}", resp)
}

pub async fn create_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .create_account(&CreateAccountRequest {
            user_id: get_user_id(),
            account_name: "TEST".to_string(),
            account_type: get_account_type(),
            currency: "USD".to_string(),
            group_id: get_group_id(),
        }, Some(&get_idempotency_key()))
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
        }, Some(&get_idempotency_key()))
        .await;

    println!("{:?}", resp)
}

pub async fn deposit_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .deposit_account(&AccountOperationRequest {
            account_id: get_account_id(),
            amount: "10000".to_string(),
            note: None,
        }, Some(&get_idempotency_key()))
        .await;

    println!("{:?}", resp)
}

pub async fn withdraw_account(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .withdraw_account(&AccountOperationRequest {
            account_id: get_account_id(),
            amount: "1000".to_string(),
            note: None,
        }, Some(&get_idempotency_key()))
        .await;

    println!("{:?}", resp)
}

pub async fn get_monthly_active_accounts(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_monthly_active_accounts(&MonthlyActiveAccountsRequest {
            for_month: "2024-12".to_string(),
            return_type: "json".to_string(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn get_closed_trades_report(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_closed_trades_report(&GetClosedTradesReportRequest {
            account_ids: None,
            account_type: get_account_type(),
            start_date_time: Utc::now().sub(TimeDelta::days(30)).to_rfc3339(),
            end_date_time: Utc::now().to_rfc3339(),
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

pub async fn get_opened_positions(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_opened_positions(&GetOpenedPositionsRequest {
            account_id: None,
            account_type: get_account_type(),
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
            account_type: get_account_type(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn get_instruments(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_instruments(&GetInstrumentsRequest {
            account_type: get_account_type(),
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
            account_type: get_account_type(),
            account_ids: Some(vec!["L#705519".to_string()]),
            account_status: Some(AccountStatus::Active),
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
    let resp = rest_client.get_api_status().await;

    println!("{:?}", resp)
}

pub async fn is_api_alive(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client.is_api_alive().await;

    println!("{:?}", resp)
}

pub async fn get_assets(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_assets(&GetAssetsRequest {
            account_type: get_account_type(),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn get_trades_report(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let now = Utc::now();
    let request = GetTradesReportRequest {
        account_type: AccountType::Live,
        account_ids: None,
        start_date_time: Some(date_to_string(now.sub(TimeDelta::days(10)))),
        end_date_time: Some(date_to_string(now)),
    };
    println!("==========");
    println!("{:?} sending {:?}", Utc::now(), request);

    let resp = rest_client.get_trades_report(&request).await;

    println!("{:?} got response {:?}", Utc::now(), resp,);
    println!("==========");
}
pub fn date_to_string(date: DateTime<Utc>) -> String {
    format!("{}", date.format(FORMAT))
}

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ"; // yyyy-MM-ddTHH:mm:ss.SSSZ e.g., 2021-12-31T23:59:59.999Z

pub async fn load_test(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let max_parallel_requests: usize = 1;
    let num_requests: usize = 10000;
    let delay = core::time::Duration::from_secs(1);
    let semaphore = Arc::new(Semaphore::new(max_parallel_requests));

    let futures = (0..num_requests).map(|i| {
        let semaphore = Arc::clone(&semaphore);

        async move {
            let _permit = semaphore
                .acquire()
                .await
                .expect("Semaphore wasn't been closed");

            tokio::time::sleep(delay).await;
            get_trades_report(rest_client).await
        }
    });

    join_all(futures).await;
}

pub async fn load_test_generic<F, Fut>(custom_fn: F)
where
    F: Fn() -> Fut + Send + Sync + Clone + 'static,
    Fut: std::future::Future<Output = ()> + Send,
{
    let max_parallel_requests = 100;
    let num_requests = 500;
    let semaphore = Arc::new(Semaphore::new(max_parallel_requests));

    let futures = (0..num_requests).map(|_| {
        let semaphore = Arc::clone(&semaphore);
        let custom_fn = custom_fn.clone(); // Clone the custom function to avoid moving it

        async move {
            // limit by max_concurrent_requests
            let _permit = semaphore
                .acquire()
                .await
                .expect("Semaphore wasn't been closed");

            custom_fn().await
        }
    });

    join_all(futures).await;
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

    async fn get_timeout(&self) -> Duration {
        Duration::from_secs(15)
    }
}

pub async fn get_orders(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .get_orders(&GetOrdersRequest {
            account_type: get_account_type(),
            account_id: Some("L#708261".to_string()),
            offset: None,
            limit: Some(1000),
        })
        .await;

    println!("{:?}", resp)
}

pub async fn cancel_order(rest_client: &BrandApiClient<ExampleBrandApiConfig>) {
    let resp = rest_client
        .cancel_order(&CancelOrderRequest {
            account_type: get_account_type(),
            order_id: "72057594042846841".to_string(),
        })
        .await;

    println!("{:?}", resp)
}
