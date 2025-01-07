use crate::brand_socket::callback::{BrandSocketApiEventHandler, BrandSocketApiInner};
use my_socket_io_client::{my_web_socket_client, MySocketIoClient, SocketIoClientSettings, WsClientSettings};
use rust_extensions::Logger;
use std::sync::Arc;
use crate::models::AccountType;

#[async_trait::async_trait]
pub trait BrandSocketApiConfig {
    async fn get_server_url(&self) -> String;
    async fn get_api_key(&self) -> String;
    async fn get_account_type(&self) -> AccountType;
}

pub struct BrandSocketApiClient {
    config_wrapper: Arc<BrandSocketApiConfigWrapper>,
    socket_io_client: tokio::sync::Mutex<Option<MySocketIoClient>>,
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

    pub async fn connect(self: Arc<Self>) -> Result<(), String> {
        my_web_socket_client::my_tls::install_default_crypto_providers();
        let socket_io_client = MySocketIoClient::new(
            "trade-locker-brand-socket",
            self.config_wrapper.clone(),
            self.inner.clone(),
            self.logger.clone(),
        )
        .set_debug_payloads(true);

        socket_io_client
            .register_subscriber(self.inner.clone())
            .await;

        socket_io_client.start();
        self.socket_io_client.lock().await.replace(socket_io_client);

        Ok(())
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

    async fn get_namespace(&self, _client_name: &str) -> String {
        self.socket_io_conf.namespace.to_string()
    }

    async fn get_headers(&self, _client_name: &str) -> Vec<(String, String)> {
        vec![(self.socket_io_conf.api_key_header_name.to_string(), self.config.get_api_key().await.to_string())]
    }

    async fn get_query_params(&self, _client_name: &str) -> Vec<(String, String)> {
        vec![(self.socket_io_conf.query_param_type_name.to_string(), self.config.get_account_type().await.to_string())]
    }
}

#[async_trait::async_trait]
impl WsClientSettings for BrandSocketApiConfigWrapper {
    async fn get_url(&self, _client_name: &str) -> String {
        self.config.get_server_url().await
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
            api_key_header_name: "BRAND_API_KEY",
            query_param_type_name: "type",
        }
    }
}