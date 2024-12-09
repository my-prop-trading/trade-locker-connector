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

pub struct BrandSocketApiClient {
    event_handler: Arc<dyn BrandSocketEventHandler + Send + Sync + 'static>,
}
