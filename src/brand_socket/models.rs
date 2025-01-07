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
    ConnectionError(ConnectionErrorMessage),
}

pub struct BrandSocketEventDeserialized {
    pub result: Result<BrandSocketEvent, BrandSocketEventDeserializeErr>,
}

pub enum BrandSocketEventDeserializeErr {
    NotSupported(String),
    Serde(serde_json::Error),
}

impl SocketIoSubscribeEventModel for BrandSocketEventDeserialized {
    const NAME_SPACE: &'static str = "/brand-socket";

    const EVENT_NAME: &'static str = "stream";

    fn deserialize(payload: &str) -> Self {
        let type_model: StreamTypeModel = serde_json::from_str(payload).unwrap();

        let result = match type_model.r#type.as_str() {
            id if id == AccountStatusMessage::get_message_type() => {
                match serde_json::from_str(payload) {
                    Ok(event) => Ok(BrandSocketEvent::AccountStatus(event)),
                    Err(err) => Err(BrandSocketEventDeserializeErr::Serde(err)),
                }
            }
            id if id == PropertyMessage::get_message_type() => {
                match serde_json::from_str(payload) {
                    Ok(event) => Ok(BrandSocketEvent::Property(event)),
                    Err(err) => Err(BrandSocketEventDeserializeErr::Serde(err)),
                }
            }
            id if id == PositionMessage::get_message_type() => {
                match serde_json::from_str(payload) {
                    Ok(event) => Ok(BrandSocketEvent::Position(event)),
                    Err(err) => Err(BrandSocketEventDeserializeErr::Serde(err)),
                }
            }
            id if id == ClosePositionMessage::get_message_type() => {
                match serde_json::from_str(payload) {
                    Ok(event) => Ok(BrandSocketEvent::ClosePosition(event)),
                    Err(err) => Err(BrandSocketEventDeserializeErr::Serde(err)),
                }
            }
            id if id == OpenOrderMessage::get_message_type() => {
                match serde_json::from_str(payload) {
                    Ok(event) => Ok(BrandSocketEvent::OpenOrder(event)),
                    Err(err) => Err(BrandSocketEventDeserializeErr::Serde(err)),
                }
            }
            id if id == ConnectionErrorMessage::get_message_type() => {
                match serde_json::from_str(payload) {
                    Ok(event) => Ok(BrandSocketEvent::ConnectionError(event)),
                    Err(err) => Err(BrandSocketEventDeserializeErr::Serde(err)),
                }
            }
            _ => Err(BrandSocketEventDeserializeErr::NotSupported(format!(
                "message type {:?}",
                type_model
            ))),
        };

        BrandSocketEventDeserialized { result }
    }
}

impl BrandSocketEvent {
    pub fn get_message_type(&self) -> &'static str {
        match self {
            BrandSocketEvent::AccountStatus(_) => AccountStatusMessage::get_message_type(),
            BrandSocketEvent::Property(_) => PropertyMessage::get_message_type(),
            BrandSocketEvent::Position(_) => PositionMessage::get_message_type(),
            BrandSocketEvent::ClosePosition(_) => ClosePositionMessage::get_message_type(),
            BrandSocketEvent::OpenOrder(_) => OpenOrderMessage::get_message_type(),
            BrandSocketEvent::ConnectionError(_) => ConnectionErrorMessage::get_message_type(),
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
    pub fn get_message_type() -> &'static str {
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
    pub fn get_message_type() -> &'static str {
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
    pub fn get_message_type() -> &'static str {
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
    pub fn get_message_type() -> &'static str {
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
    pub fn get_message_type() -> &'static str {
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
    pub fn get_message_type() -> &'static str {
        "ConnectionErrorMessage"
    }
}
