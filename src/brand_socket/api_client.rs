use crate::brand_socket::event_handler::BrandSocketEventHandler;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct SocketIoConfig {
    pub namespace: &'static str,
    pub handshake_path: &'static str,
    pub transport: &'static str,
}

impl Default for SocketIoConfig {
    fn default() -> Self {
        Self {
            namespace: "/brand-socket",
            handshake_path: "/brand-socket/socket.io",
            transport: "websocket",
        }
    }
}

#[async_trait::async_trait]
pub trait BrandSocketApiConfig {
    async fn get_server_url(&self) -> String;
    async fn get_api_key(&self) -> String;
}

pub struct BrandSocketApiClient {
    event_handler: Arc<dyn BrandSocketEventHandler + Send + Sync + 'static>,
    config: Arc<dyn BrandSocketApiConfig + Send + Sync>,
}

impl BrandSocketApiClient {
    pub fn new(
        event_handler: Arc<dyn BrandSocketEventHandler + Send + Sync>,
        config: Arc<dyn BrandSocketApiConfig + Send + Sync>,
    ) -> Self {
        Self {
            event_handler,
            config,
        }
    }
}
