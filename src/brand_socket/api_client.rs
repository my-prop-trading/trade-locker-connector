use crate::brand_socket::event_handler::BrandSocketEventHandler;
use std::sync::Arc;

pub struct BrandSocketApiClient {
    event_handler: Arc<dyn BrandSocketEventHandler + Send + Sync + 'static>,
}
