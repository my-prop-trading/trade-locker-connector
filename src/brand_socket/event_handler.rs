use super::models::*;

#[async_trait::async_trait]
pub trait BrandSocketEventHandler {
    async fn on_event(&self, event: BrandSocketEvent);
    async fn on_connected(&self);
}
