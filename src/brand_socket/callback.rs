use super::models::*;
use my_socket_io_client::{SocketIoBeforeConnectResult, SocketIoCallbacks, SocketIoConnection, SocketIoEventSubscriberCallback};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::brand_socket::api_client::BrandSocketApiConfigWrapper;

#[async_trait::async_trait]
pub trait BrandSocketApiEventHandler {
    async fn on_event(&self, event: BrandSocketEvent);
    async fn on_connected(&self);
    async fn on_disconnected(&self);
}

pub struct BrandSocketApiInner {
    handler: Arc<dyn BrandSocketApiEventHandler + Send + Sync + 'static>,
    connection: RwLock<Option<Arc<SocketIoConnection>>>,
    config_wrapper: Arc<BrandSocketApiConfigWrapper>,
}

impl BrandSocketApiInner {
    pub fn new(handler: Arc<dyn BrandSocketApiEventHandler + Send + Sync + 'static>, config_wrapper: Arc<BrandSocketApiConfigWrapper>) -> Self {
        Self {
            handler,
            connection: Default::default(),
            config_wrapper,
        }
    }

    pub async fn connected(&self) {
        self.handler.on_connected().await;
    }
}

#[async_trait::async_trait]
impl SocketIoCallbacks for BrandSocketApiInner {
    async fn before_connect(&self) -> SocketIoBeforeConnectResult {
        let conf = SocketIoConfig::default();
        
        SocketIoBeforeConnectResult {
            append_headers: vec![(conf.api_key_header.into(), self.config_wrapper.config.get_api_key().await.into())].into(),
            append_query_params: vec![(conf.query_param_type_name.into(), self.config_wrapper.config.get_account_type().await.to_string().into())].into(),
        }
    }

    async fn on_connect(&self, connection: Arc<SocketIoConnection>) {
        let mut current_connection = self.connection.write().await;
        *current_connection = Some(connection.clone());
        drop(current_connection);
        self.handler.on_connected().await;
    }

    async fn on_disconnect(&self, _connection: Arc<SocketIoConnection>) {
        let mut current_connection = self.connection.write().await;
        *current_connection = None;
        drop(current_connection);
        self.handler.on_disconnected().await;
    }
}

#[async_trait::async_trait]
impl SocketIoEventSubscriberCallback<BrandSocketEvent, ()> for BrandSocketApiInner {
    async fn on_event(&self, event: BrandSocketEvent) -> () {
        self.handler.on_event(event).await;
        
        ()
    }
}

#[derive(Clone, Debug)]
struct SocketIoConfig {
    //pub namespace: &'static str,
    //pub handshake_path: &'static str,
    //pub transport: &'static str,
    pub api_key_header: &'static str,
    pub query_param_type_name: &'static str,
}

impl Default for SocketIoConfig {
    fn default() -> Self {
        Self {
            //namespace: "/brand-socket",
            //handshake_path: "/brand-socket/socket.io",
            //transport: "websocket",
            api_key_header: "BRAND_API_KEY",
            query_param_type_name: "type",
        }
    }
}
