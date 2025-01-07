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
    pub balance: Option<String>,
    #[serde(rename = "marginAvailable")]
    pub margin_available: Option<String>,
    #[serde(rename = "marginUsed")]
    pub margin_used: Option<String>,
    #[serde(rename = "blockedBalance")]
    pub blocked_balance: Option<String>,
    pub credit: Option<String>,
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
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "positionId")]
    pub position_id: String,
    /// Number of lots in the position.
    pub lots: String,
    /// Size of each lot.
    #[serde(rename = "lotSize")]
    pub lot_size: Option<String>,
    /// Total number of units in the position.
    pub units: Option<String>,
    pub instrument: String,
    #[serde(rename = "openPrice")]
    pub open_price: String,
    #[serde(rename = "openDateTime")]
    pub open_date_time: String,
    #[serde(rename = "openOrderId")]
    pub open_order_id: String,
    #[serde(rename = "stopLossOrderId")]
    pub stop_loss_order_id: Option<String>,
    #[serde(rename = "stopLossLimit")]
    pub stop_loss_limit: Option<String>,
    /// Maintenance margin required for the position.
    #[serde(rename = "maintMargin")]
    pub maint_margin: String,
    #[serde(rename = "takeProfitOrderId")]
    pub take_profit_order_id: Option<String>,
    #[serde(rename = "takeProfitLimit")]
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
    #[serde(rename = "positionId")]
    pub positions_id: String,
    #[serde(rename = "closePrice")]
    pub close_price: Option<String>,
    #[serde(rename = "closeDateTime")]
    pub close_date_time: String,
}

impl ClosePositionMessage {
    pub fn get_message_type() -> &'static str {
        "ClosePosition"
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenOrderMessage {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    pub instrument: String,
    pub amount: String,
    #[serde(rename = "lotSize")]
    pub lot_size: String,
    /// Allowed values: "BUY""SELL"
    pub side: String,
    pub price: String,
    /// Allowed values: "PENDING""EXECUTED""CANCELLED
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
    pub status: Option<String>,
    pub message: Option<String>,
}

impl ConnectionErrorMessage {
    pub fn get_message_type() -> &'static str {
        "ConnectionErrorMessage"
    }
}
