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
        server_url: "wss://api-dev.tradelocker.com/brand-api/socket.io/".to_string(),
        //server_url: "wss://api.tradelocker.com/brand-api/socket.io/".to_string(),
        api_key,
    });
    let brand_api = Arc::new(BrandSocketApiClient::new(Arc::new(ExampleBrandSocketApiEventHandler), config, Arc::new(ConsoleLogger)));
    let result = brand_api.connect().await;

    if let Err(error) = result {
        println!("Error connect: {:?}", error);
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

    async fn on_disconnected(&self) {
        
    }
}

pub struct ConsoleLogger;

impl rust_extensions::Logger for ConsoleLogger {
    fn write_info(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("INFO:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_warning(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("WARNING:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_error(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
        println!("ERROR:");
        println!("{}", message);
        println!("===========================");
    }

    fn write_fatal_error(
        &self,
        _process: String,
        message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
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
