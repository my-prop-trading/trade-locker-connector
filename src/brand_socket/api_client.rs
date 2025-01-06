use crate::brand_socket::callback::{BrandSocketApiEventHandler, BrandSocketApiInner};
use my_socket_io_client::{my_web_socket_client, MySocketIoClient, WsClientSettings};
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
        let config_wrapper = Arc::new(BrandSocketApiConfigWrapper {
            config: Arc::clone(&config),
        });
        Self {
            inner: Arc::new(BrandSocketApiInner::new(handler, config_wrapper.clone())),
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
        .set_debug_payloads(false);

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
}

#[async_trait::async_trait]
impl WsClientSettings for BrandSocketApiConfigWrapper {
    async fn get_url(&self, _client_name: &str) -> String {
        self.config.get_server_url().await
    }
}
