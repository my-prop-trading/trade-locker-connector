use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUserResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckEmailRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckEmailResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetUserPasswordRequest {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetAccountRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
}

/// Represents the reason for trading being disabled.
#[derive(Debug, Serialize, Deserialize)]
pub struct TradingDisabledReason {
    /// The reason type for trading being disabled (e.g., RISK_RULE).
    #[serde(rename = "type")]
    pub reason_type: String,
}

/// Represents an account with various attributes.
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountModel {
    /// The name of the account.
    #[serde(rename = "accountName")]
    pub account_name: String,

    /// The unique identifier for the account.
    #[serde(rename = "accountId")]
    pub account_id: String,

    /// The unique identifier for the user associated with the account (UUID).
    #[serde(rename = "userId")]
    pub user_id: String,

    /// The user group ID of the account.
    #[serde(rename = "userGroupId")]
    pub user_group_id: String,

    /// The type of the account to create (e.g., LIVE, DEMO).
    #[serde(rename = "type")]
    pub account_type: AccountType,

    /// The status of the account (e.g., ACTIVE, RESTRICTED, SUSPENDED).
    /// If ACTIVE, trading could be disabled by a risk rule.
    /// Check the 'tradingDisabledReason' property.
    #[serde(rename = "status")]
    pub status: AccountStatus,

    /// The reason for trading being disabled. If null, trading is enabled.
    #[serde(rename = "tradingDisabledReason")]
    pub trading_disabled_reason: Option<TradingDisabledReason>,

    /// The 3-letter ISO 4217 code of the currency or ticker symbol for the crypto asset of this account.
    #[serde(rename = "currency")]
    pub currency: String,

    /// The leverage for the account (optional).
    #[serde(rename = "leverage")]
    pub leverage: Option<String>,

    /// The current account balance.
    #[serde(rename = "balance")]
    pub balance: String,

    /// The current account credit.
    #[serde(rename = "credit")]
    pub credit: String,

    /// The current account equity.
    #[serde(rename = "equity")]
    pub equity: String,

    /// The current account profit and loss (PNL).
    #[serde(rename = "pnl")]
    pub pnl: String,

    /// The current account margin available.
    #[serde(rename = "marginAvailable")]
    pub margin_available: String,

    /// The current account margin used.
    #[serde(rename = "marginUsed")]
    pub margin_used: String,

    /// The date and time when the account was created.
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    #[strum(to_string = "DEMO")]
    #[serde(rename = "DEMO")]
    Demo,
    #[strum(to_string = "LIVE")]
    #[serde(rename = "LIVE")]
    Live,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateAccountRequest {
    /// The unique identifier for the user (UUID).
    #[serde(rename = "userId")]
    pub user_id: String,

    /// The name of the account.
    #[serde(rename = "accountName")]
    pub account_name: String,

    /// The type of the account to create (e.g., LIVE).
    #[serde(rename = "type")]
    pub account_type: AccountType,

    /// The 3-letter ISO 4217 code of the currency or ticker symbol for the crypto asset of this account.
    #[serde(rename = "currency")]
    pub currency: String,

    /// The ID of the group to place the account into. If not provided, placed into the brand's default group.
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAccountStatusRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    #[strum(to_string = "ACTIVE")]
    #[serde(rename = "ACTIVE")]
    Active,
    #[strum(to_string = "RESTRICTED")]
    #[serde(rename = "RESTRICTED")]
    Restricted,
    #[strum(to_string = "SUSPENDED")]
    #[serde(rename = "SUSPENDED")]
    Suspended,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateAccountStatusResponse {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub status: AccountStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetAccountGroupRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "newGroupId")]
    pub group_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloseAccountPositionsRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CloseAccountPositionsResponse {
    #[serde(rename = "positionIdsOrderedToBeClosed")]
    pub position_ids: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreditAccountRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Amount of the operation. Positive to add, negative to subtract.
    pub amount: String,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreditAccountResponse {
    #[serde(rename = "operationId")]
    pub operation_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetInstrumentsRequest {
    #[serde(rename = "type")]
    pub account_type: AccountType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstrumentModel {
    pub name: String,
    #[serde(rename = "lotSize")]
    pub lot_size: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetInstrumentsResponse {
    pub data: Vec<InstrumentModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOpenedPositionsRequest {
    #[serde(rename = "type")]
    pub account_type: AccountType,
    #[serde(rename = "accountId")]
    pub account_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenedPositionModel {
    /// The unique identifier for the position.
    #[serde(rename = "positionId")]
    pub id: String,

    /// The account ID associated with the position.
    #[serde(rename = "accountId")]
    pub account_id: String,

    /// The number of lots in the position.
    #[serde(rename = "lots")]
    pub lots: String,

    /// The lot size for the position.
    #[serde(rename = "lotSize")]
    pub lot_size: String,

    /// The number of units in the position.
    #[serde(rename = "units")]
    pub units: String,

    /// The date and time when the position was opened.
    #[serde(rename = "openDateTime")]
    pub open_date_time: DateTime<Utc>,

    /// The profit and loss (PNL) of the position.
    #[serde(rename = "pnl")]
    pub pnl: String,

    /// The swap value for the position.
    #[serde(rename = "swap")]
    pub swap: String,

    /// The stop-loss price for the position.
    #[serde(rename = "slPrice")]
    pub sl_price: Option<String>,

    /// The take-profit price for the position.
    #[serde(rename = "tpPrice")]
    pub tp_price: Option<String>,

    /// The price at which the position was opened.
    #[serde(rename = "openPrice")]
    pub open_price: String,

    /// The side of the position (e.g., BUY or SELL).
    #[serde(rename = "side")]
    pub side: String,

    /// The financial instrument associated with the position (e.g., EURUSD).
    #[serde(rename = "instrument")]
    pub instrument: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOpenedPositionsResponse {
    pub data: Vec<OpenedPositionModel>,
}
