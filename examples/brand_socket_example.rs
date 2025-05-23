use std::collections::HashMap;
use std::sync::Arc;
use trade_locker_connector::brand_socket::api_client::{
    BrandSocketApiClient, BrandSocketApiConfig,
};
use trade_locker_connector::brand_socket::callback::BrandSocketApiEventHandler;
use trade_locker_connector::brand_socket::models::BrandSocketEvent;
use trade_locker_connector::models::AccountType;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("TRADE_LOCKER_API_KEY").unwrap();
    let config = Arc::new(ExampleBrandSocketApiConfig {
        server_url: "wss://api-dev.tradelocker.com".to_string(),
        //server_url: "wss://api.tradelocker.com".to_string(),
        api_key,
    });
    let handler = Arc::new(ExampleBrandSocketApiEventHandler);

    loop {
        let brand_api =
            BrandSocketApiClient::new(handler.clone(), config.clone(), Arc::new(ConsoleLogger));
        let result = brand_api.connect().await;

        if let Err(error) = result {
            println!("Error connect: {:?}", error);
            continue;
        }

        let result = brand_api
            .wait_until_sync_ended(std::time::Duration::from_secs(15))
            .await;

        if let Err(error) = result {
            println!("===========================");
            println!("Error wait_until_sync_ended: {:?}", error);
            println!("===========================");
            _ = brand_api.disconnect().await;
            continue;
        }
        
        if !brand_api.is_connected().await {
            panic!("brand_api is not connected");
        }
        
        println!("===========================");
        println!("SYNC ENDED");
        println!("===========================");
        break;
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

pub struct ExampleBrandSocketApiConfig {
    pub server_url: String,
    pub api_key: String,
}

#[async_trait::async_trait]
impl BrandSocketApiConfig for ExampleBrandSocketApiConfig {
    async fn get_server_url(&self) -> String {
        self.server_url.clone()
    }

    async fn get_api_key(&self) -> String {
        self.api_key.clone()
    }

    async fn get_account_type(&self) -> AccountType {
        AccountType::Live
    }
}

pub struct ExampleBrandSocketApiEventHandler;

#[async_trait::async_trait]
impl BrandSocketApiEventHandler for ExampleBrandSocketApiEventHandler {
    async fn on_event(&self, event: BrandSocketEvent) {
        println!("on_event: {:?}", event);
    }

    async fn on_connected(&self) {
        println!("on_connected");
    }

    async fn on_disconnected(&self) {}
}

pub struct ConsoleLogger;

impl rust_extensions::Logger for ConsoleLogger {
    fn write_info(&self, _process: String, message: String, _ctx: Option<HashMap<String, String>>) {
        println!("INFO:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_warning(
        &self,
        _process: String,
        message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
        println!("WARNING:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_error(
        &self,
        _process: String,
        message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
        println!("ERROR:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_fatal_error(
        &self,
        _process: String,
        message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
        println!("FATAL ERROR:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_debug_info(
        &self,
        _process: String,
        message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
        println!("DEBUG:");
        println!("{}", message);
        println!("===========================");
    }
}
