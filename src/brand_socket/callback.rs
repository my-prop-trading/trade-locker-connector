use super::models::*;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait BrandSocketApiEventHandler {
    async fn on_event(&self, event: BrandSocketEvent);
    async fn on_connected(&self);
}

pub struct BrandSocketApiInner {
    handler: Arc<dyn BrandSocketApiEventHandler + Send + Sync + 'static>,
}

impl BrandSocketApiInner {
    pub fn new(handler: Arc<dyn BrandSocketApiEventHandler + Send + Sync + 'static>) -> Self {
        Self { handler }
    }

    pub async fn connected(&self) {
        self.handler.on_connected().await;
    }

    pub async fn payload(
        &self,
        payload: rust_socketio::Payload,
        _socket: rust_socketio::RawClient,
    ) {
        println!("Received payload: {:?}", payload);

        // todo: self.handler.on_event().await;
    }

    pub async fn event(
        &self,
        event: rust_socketio::Event,
        payload: rust_socketio::Payload,
        _socket: rust_socketio::RawClient,
    ) {
        println!("Received event: {:?} with payload: {:?}", event, payload);

        // todo: self.handler.on_event().await;
    }
}
