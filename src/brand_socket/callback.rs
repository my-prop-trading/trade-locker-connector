use super::models::*;
use my_socket_io_client::{SocketIoCallbacks, SocketIoConnection, SocketIoEventSubscriberCallback};
use rust_extensions::Logger;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait::async_trait]
pub trait BrandSocketApiEventHandler {
    async fn on_event(&self, event: BrandSocketEvent);
    async fn on_connected(&self);
    async fn on_disconnected(&self);
}

pub struct BrandSocketApiInner {
    handler: Arc<dyn BrandSocketApiEventHandler + Send + Sync + 'static>,
    connection: RwLock<Option<Arc<SocketIoConnection>>>,
    logger: Arc<dyn Logger + Send + Sync + 'static>,
}

impl BrandSocketApiInner {
    pub fn new(
        handler: Arc<dyn BrandSocketApiEventHandler + Send + Sync + 'static>,
        logger: Arc<dyn Logger + Send + Sync + 'static>,
    ) -> Self {
        Self {
            handler,
            connection: Default::default(),
            logger,
        }
    }

    pub async fn connected(&self) {
        self.handler.on_connected().await;
    }
}

#[async_trait::async_trait]
impl SocketIoCallbacks for BrandSocketApiInner {
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
impl SocketIoEventSubscriberCallback<BrandSocketEventDeserialized, ()> for BrandSocketApiInner {
    async fn on_event(&self, event: BrandSocketEventDeserialized) -> () {
        match event.result {
            Ok(event) => {
                self.handler.on_event(event).await;
            }
            Err(err) => match err {
                BrandSocketEventDeserializeErr::NotSupported(err) => self.logger.write_error(
                    "BrandSocketApiInner.on_event".to_string(),
                    format!("Not supported event: {}", err),
                    None,
                ),
                BrandSocketEventDeserializeErr::Serde(err) => self.logger.write_error(
                    "BrandSocketApiInner.on_event".to_string(),
                    format!("Failed to deserialize: {}", err),
                    None,
                ),
            },
        }

        ()
    }
}
