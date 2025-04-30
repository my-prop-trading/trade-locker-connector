use crate::brand_socket::callback::{BrandSocketApiEventHandler, BrandSocketApiInner};
use crate::models::AccountType;
use my_socket_io_client::{
    my_web_socket_client, MySocketIoClient, SocketIoClientSettings, WsClientSettings,
};
use rust_extensions::Logger;
use std::sync::Arc;
use std::time::Duration;

#[async_trait::async_trait]
pub trait BrandSocketApiConfig {
    async fn get_server_url(&self) -> String;
    async fn get_api_key(&self) -> String;
    async fn get_account_type(&self) -> AccountType;
}

pub struct BrandSocketApiClient {
    config_wrapper: Arc<BrandSocketApiConfigWrapper>,
    socket_io_client: std::sync::Mutex<Option<MySocketIoClient>>,
    inner: Arc<BrandSocketApiInner>,
    logger: Arc<dyn Logger + Send + Sync + 'static>,
}

impl BrandSocketApiClient {
    pub fn new(
        handler: Arc<dyn BrandSocketApiEventHandler + Send + Sync>,
        config: Arc<dyn BrandSocketApiConfig + Send + Sync>,
        logger: Arc<dyn Logger + Send + Sync + 'static>,
    ) -> Self {
        let config_wrapper = Arc::new(BrandSocketApiConfigWrapper::new(config));

        Self {
            inner: Arc::new(BrandSocketApiInner::new(handler, Arc::clone(&logger))),
            config_wrapper,
            socket_io_client: Default::default(),
            logger,
        }
    }

    pub async fn disconnect(&self) -> Result<(), String> {
        self.inner.disconnect().await;
        let socket_io_client = self.socket_io_client.lock().unwrap().take();

        if let Some(socket_io_client) = socket_io_client {
            socket_io_client.stop();
        }

        Ok(())
    }

    pub async fn is_connected(&self) -> bool {
        self.socket_io_client.lock().unwrap().is_some() && self.inner.is_connected().await
    }

    pub async fn connect(&self) -> Result<(), String> {
        my_web_socket_client::my_tls::install_default_crypto_providers();
        let is_debug = std::env::var("DEBUG").is_ok();
        
        if is_debug {
            println!("BrandSocketApiClient: Debug payloads are enabled");
        }
        
        let socket_io_client = MySocketIoClient::new(
            "trade-locker-brand-socket",
            self.config_wrapper.clone(),
            self.inner.clone(),
            self.logger.clone(),
        )
        .set_debug_payloads(is_debug);

        socket_io_client
            .register_subscriber(self.inner.clone())
            .await;
        socket_io_client.start();
        self.socket_io_client
            .lock()
            .unwrap()
            .replace(socket_io_client);

        Ok(())
    }

    pub async fn wait_until_sync_ended(&self, timeout: Duration) -> Result<(), String> {
        self.inner.wait_until_sync_ended(timeout).await
    }
}

pub struct BrandSocketApiConfigWrapper {
    pub config: Arc<dyn BrandSocketApiConfig + Send + Sync>,
    socket_io_conf: SocketIoConfig,
}

impl BrandSocketApiConfigWrapper {
    pub fn new(config: Arc<dyn BrandSocketApiConfig + Send + Sync>) -> Self {
        Self {
            config,
            socket_io_conf: SocketIoConfig::default(),
        }
    }
}

#[async_trait::async_trait]
impl SocketIoClientSettings for BrandSocketApiConfigWrapper {
    async fn get_server_url(&self, _client_name: &str) -> String {
        self.config.get_server_url().await
    }

    async fn get_handshake_path(&self, _client_name: &str) -> String {
        self.socket_io_conf.handshake_path.to_string()
    }

    async fn get_headers(&self, _client_name: &str) -> Vec<(String, String)> {
        vec![(
            self.socket_io_conf.api_key_header_name.to_string(),
            self.config.get_api_key().await.to_string(),
        )]
    }

    async fn get_query_params(&self, _client_name: &str) -> Vec<(String, String)> {
        vec![(
            self.socket_io_conf.query_param_type_name.to_string(),
            self.config.get_account_type().await.to_string(),
        )]
    }
}

#[async_trait::async_trait]
impl WsClientSettings for BrandSocketApiConfigWrapper {
    async fn get_url(&self, _client_name: &str) -> Option<String> {
        Some(self.config.get_server_url().await)
    }
}

#[derive(Clone, Debug)]
struct SocketIoConfig {
    pub namespace: &'static str,
    pub handshake_path: &'static str,
    //pub transport: &'static str,
    pub api_key_header_name: &'static str,
    pub query_param_type_name: &'static str,
}

impl Default for SocketIoConfig {
    fn default() -> Self {
        Self {
            namespace: "/brand-socket",
            handshake_path: "/brand-socket/socket.io",
            //transport: "websocket",
            api_key_header_name: "brand-api-key",
            query_param_type_name: "type",
        }
    }
}
