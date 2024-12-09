use serde_derive::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum BrandSocketEvent {
    // Provides updates about the status of an account, such as changes in balance, margin, or account status.
    AccountStatus(AccountStatusMessage),
    // Indicates that the initial synchronization of data is complete. After this, only updates will be sent.
    Property(PropertyMessage),
    // Updates on opening or modifying a positions
    Position(PositionMessage),
    // Sent when a position is closed, providing details of the closure, such as time and realized P&L
    ClosePosition(ClosePositionMessage),
    // Details about new or updated order, allowing you to track pending trades in real-time.
    OpenOrder(OpenOrderMessage),
    // Connection updates and errors
    ConnectionErrorMessage(ConnectionErrorMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountStatusMessage {
    // Allowed values: "AccountStatus"
    pub r#type: String,
    pub account_id: String,
    pub currency: String,
    pub balance: String,
    pub margin_available: String,
    pub margin_used: String,
    pub blocked_balance: String,
    pub credit: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyMessage {
    // Property messages. SyncEnd property message means that the initial data sync is over and the upcoming messages are realtime events from the Tradelocker engine.
    //
    // Allowed values: "Property"
    pub r#type: String,
    // Name of the property.
    //
    // Allowed values: "SyncEnd"
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionMessage {
    // Allowed values: "Position"
    pub r#type: String,
    pub account_id: String,
    pub position_id: String,
    pub lots: String,
    pub lot_size: String,
    pub units: String,
    pub instrument: String,
    pub open_price: String,
    pub open_date_time: String,
    pub open_order_id: String,
    pub stop_loss_order_id: Option<String>,
    pub stop_loss_limit: Option<String>,
    pub maint_margin: String,
    pub take_profit_order_id: Option<String>,
    pub take_profit_limit: Option<String>,
    pub side: String,
    pub fee: Option<String>,
    pub swaps: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClosePositionMessage {
    // Allowed values: "ClosePosition"
    pub r#type: String,
    pub positions_id: String,
    pub close_price: String,
    pub close_date_time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenOrderMessage {
    // Allowed values: "OpenOrder"
    pub r#type: String,
    pub account_id: String,
    pub order_id: String,
    pub instrument: String,
    pub amount: String,
    pub lot_size: String,
    // Allowed values: "BUY""SELL"
    pub side: String,
    pub price: String,
    // Allowed values: "PENDING""EXECUTED""CANCELLED
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionErrorMessage {
    // Connection status.
    //
    // Allowed values: "ok""error"
    pub status: String,
    pub message: String,
}
