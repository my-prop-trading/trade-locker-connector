use crate::brand_socket::callback::{BrandSocketApiEventHandler, BrandSocketApiInner};
use std::sync::Arc;

#[derive(Clone, Debug)]
struct SocketIoConfig {
    pub namespace: &'static str,
    pub handshake_path: &'static str,
    pub transport: &'static str,
    pub key_header: &'static str,
}

impl Default for SocketIoConfig {
    fn default() -> Self {
        Self {
            namespace: "/brand-socket",
            handshake_path: "/brand-socket/socket.io",
            transport: "websocket",
            key_header: "BRAND_API_KEY",
        }
    }
}

#[async_trait::async_trait]
pub trait BrandSocketApiConfig {
    async fn get_server_url(&self) -> String;
    async fn get_api_key(&self) -> String;
}

pub struct BrandSocketApiClient {
    config: Arc<dyn BrandSocketApiConfig + Send + Sync>,
    socket_io_client: tokio::sync::Mutex<Option<rust_socketio::client::Client>>,
    inner: Arc<BrandSocketApiInner>,
}

impl BrandSocketApiClient {
    pub fn new(
        handler: Arc<dyn BrandSocketApiEventHandler + Send + Sync>,
        config: Arc<dyn BrandSocketApiConfig + Send + Sync>,
    ) -> Self {
        Self {
            config,
            socket_io_client: Default::default(),
            inner: Arc::new(BrandSocketApiInner::new(handler)),
        }
    }

    pub async fn connect(self: Arc<Self>) -> Result<(), String> {
        let cloned_self = self.clone();
        let callback = move |event: rust_socketio::Event,
                             payload: rust_socketio::Payload,
                             socket: rust_socketio::RawClient| {
            let inner = cloned_self.inner.clone();

            tokio::spawn(async move {
                inner.event(event, payload, socket).await;
            });
        };

        let socket_io_config = SocketIoConfig::default();
        let result = rust_socketio::ClientBuilder::new(self.config.get_server_url().await)
            .namespace(socket_io_config.namespace)
            .on_any(callback)
            .opening_header(socket_io_config.key_header, self.config.get_api_key().await)
            .connect();

        match result {
            Ok(client) => {
                self.socket_io_client.lock().await.replace(client);
                Ok(())
            }
            Err(error) => Err(format!("{:?}", error)),
        }
    }
}
