use serde_derive::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BrandSocketEvent {
    // Provides updates about the status of an account, such as changes in balance, margin, or account status.
    AccountStatus(),
    // Indicates that the initial synchronization of data is complete. After this, only updates will be sent.
    SyncEnd(),
    // Updates on opening or modifying a positions
    Position(),
    // Sent when a position is closed, providing details of the closure, such as time and realized P&L
    ClosePosition(),
    // Details about new or updated order, allowing you to track pending trades in real-time.
    OpenOrder(),
}
