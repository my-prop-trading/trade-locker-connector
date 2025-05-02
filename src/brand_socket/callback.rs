use super::models::*;
use my_socket_io_client::{SocketIoCallbacks, SocketIoConnection, SocketIoEventSubscriberCallback};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use rust_extensions::Logger;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicBool, AtomicI64};
use std::sync::Arc;
use std::time::{Duration, Instant};
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
    sync_ended: AtomicBool,
    last_event_timestamp: AtomicI64,
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
            sync_ended: AtomicBool::new(false),
            last_event_timestamp: Default::default(),
        }
    }

    pub async fn is_connected(&self) -> bool {
        self.connection.read().await.is_some()
    }

    pub async fn wait_until_sync_ended(&self, timeout: Duration) -> Result<(), String> {
        let instant = Instant::now();

        loop {
            if self.is_connected().await && self.sync_ended.load(Relaxed) {
                return Ok(());
            }

            tokio::time::sleep(Duration::from_millis(250)).await;

            if instant.elapsed() > timeout {
                return Err(format!("Timeout {:?}", timeout));
            }
        }
    }

    pub async fn disconnect(&self) {
        let connection = self.connection.write().await.take();

        if let Some(connection) = connection {
            connection.disconnect().await;
        }
    }

    pub fn get_last_event_timestamp(&self) -> Option<DateTimeAsMicroseconds> {
        let last_event_timestamp = self.last_event_timestamp.load(Relaxed);

        if last_event_timestamp <= 0 {
            None
        } else {
            Some(DateTimeAsMicroseconds::from(last_event_timestamp))
        }
    }
}

#[async_trait::async_trait]
impl SocketIoCallbacks for BrandSocketApiInner {
    async fn on_connect(&self, connection: Arc<SocketIoConnection>) {
        let prev_connection = self.connection.write().await.replace(connection);

        if let Some(prev_connection) = prev_connection {
            prev_connection.disconnect().await;
        }

        self.handler.on_connected().await;
    }

    async fn on_disconnect(&self, _connection: Arc<SocketIoConnection>) {
        _ = self.connection.write().await.take();
        self.handler.on_disconnected().await;
    }
}

#[async_trait::async_trait]
impl SocketIoEventSubscriberCallback<BrandSocketEventDeserialized, ()> for BrandSocketApiInner {
    async fn on_event(&self, event: BrandSocketEventDeserialized) -> () {
        self.last_event_timestamp
            .store(DateTimeAsMicroseconds::now().unix_microseconds, Relaxed);

        match event.result {
            Ok(event) => {
                match &event {
                    BrandSocketEvent::AccountStatus(_) => {}
                    BrandSocketEvent::Property(message) => {
                        if message.name == "SyncEnd" {
                            self.sync_ended.store(true, Relaxed);
                        }
                    }
                    BrandSocketEvent::Position(_) => {}
                    BrandSocketEvent::ClosePosition(_) => {}
                    BrandSocketEvent::OpenOrder(_) => {}
                    BrandSocketEvent::ConnectionError(_) => {}
                };

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
