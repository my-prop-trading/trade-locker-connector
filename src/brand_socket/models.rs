use my_socket_io_client::SocketIoSubscribeEventModel;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct StreamTypeModel {
    #[serde(rename = "type")]
    pub r#type: String,
}

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

impl SocketIoSubscribeEventModel for BrandSocketEvent {
    const NAME_SPACE: &'static str = "/brand-socket";

    const EVENT_NAME: &'static str = "stream";

    fn deserialize(payload: &str) -> Self {
        let type_model: StreamTypeModel = serde_json::from_str(payload).unwrap();

        match type_model.r#type.as_str() {
            id if id == AccountStatusMessage::get_id() => {
                Self::AccountStatus(serde_json::from_str(payload).unwrap())
            }
            id if id == PropertyMessage::get_id() => {
                Self::Property(serde_json::from_str(payload).unwrap())
            }
            id if id == PositionMessage::get_id() => {
                Self::Position(serde_json::from_str(payload).unwrap())
            }
            id if id == ClosePositionMessage::get_id() => {
                Self::ClosePosition(serde_json::from_str(payload).unwrap())
            }
            id if id == OpenOrderMessage::get_id() => {
                Self::OpenOrder(serde_json::from_str(payload).unwrap())
            }
            id if id == ConnectionErrorMessage::get_id() => {
                Self::ConnectionErrorMessage(serde_json::from_str(payload).unwrap())
            }
            _ => {
                panic!("Unknown stream type: {}", type_model.r#type);
            }
        }
    }
}

impl BrandSocketEvent {
    pub fn get_message_id(&self) -> &'static str {
        match self {
            BrandSocketEvent::AccountStatus(_) => AccountStatusMessage::get_id(),
            BrandSocketEvent::Property(_) => PropertyMessage::get_id(),
            BrandSocketEvent::Position(_) => PositionMessage::get_id(),
            BrandSocketEvent::ClosePosition(_) => ClosePositionMessage::get_id(),
            BrandSocketEvent::OpenOrder(_) => OpenOrderMessage::get_id(),
            BrandSocketEvent::ConnectionErrorMessage(_) => ConnectionErrorMessage::get_id(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountStatusMessage {
    pub account_id: String,
    pub currency: String,
    pub balance: String,
    pub margin_available: String,
    pub margin_used: String,
    pub blocked_balance: String,
    pub credit: String,
}

impl AccountStatusMessage {
    pub fn get_id() -> &'static str {
        "AccountStatus"
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PropertyMessage {
    // Name of the property.
    //
    // Allowed values: "SyncEnd"
    pub name: String,
}

impl PropertyMessage {
    pub fn get_id() -> &'static str {
        "Property"
    }
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

impl PositionMessage {
    pub fn get_id() -> &'static str {
        "Position"
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClosePositionMessage {
    pub positions_id: String,
    pub close_price: String,
    pub close_date_time: String,
}

impl ClosePositionMessage {
    pub fn get_id() -> &'static str {
        "ClosePosition"
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenOrderMessage {
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

impl OpenOrderMessage {
    pub fn get_id() -> &'static str {
        "OpenOrder"
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionErrorMessage {
    // Allowed values: "ok", "error"
    pub status: String,
    pub message: String,
}

impl ConnectionErrorMessage {
    pub fn get_id() -> &'static str {
        "ConnectionErrorMessage"
    }
}
