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
    #[serde(rename = "positionId")]
    pub id: String,

    #[serde(rename = "accountId")]
    pub account_id: String,

    #[serde(rename = "lots")]
    pub lots: String,

    #[serde(rename = "lotSize")]
    pub lot_size: String,

    #[serde(rename = "units")]
    pub units: String,

    #[serde(rename = "openDateTime")]
    pub open_date_time: DateTime<Utc>,

    #[serde(rename = "pnl")]
    pub pnl: String,

    #[serde(rename = "swap")]
    pub swap: String,

    #[serde(rename = "slPrice")]
    pub sl_price: Option<String>,

    #[serde(rename = "tpPrice")]
    pub tp_price: Option<String>,

    #[serde(rename = "openPrice")]
    pub open_price: String,

    #[serde(rename = "side")]
    pub side: OpenedPositionSide,

    #[serde(rename = "instrument")]
    pub instrument: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetOpenedPositionsResponse {
    pub data: Vec<OpenedPositionModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetClosedPositionsRequest {
    pub account_id: String,
    pub account_type: AccountType,
    /// Cursor to fetch the next page of events. If not provided, the first page of events will be returned.
    /// Must be an integer string, greater than or equal to 0 and less than or equal to 9223372036854775807
    pub cursor: Option<String>,
    /// Page size; max 1000, default 20.
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClosedPositionModel {
    #[serde(rename = "instrument")]
    pub instrument: String,

    #[serde(rename = "openMilliseconds")]
    pub open_milliseconds: String,

    #[serde(rename = "openDateTime")]
    pub open_date_time: DateTime<Utc>,

    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    #[serde(rename = "positionSide")]
    pub position_side: ClosedPositionSide,

    #[serde(rename = "closeAmount")]
    pub close_amount: String,

    #[serde(rename = "averageOpenPrice")]
    pub average_open_price: String,

    #[serde(rename = "closePrice")]
    pub close_price: String,

    #[serde(rename = "closeMilliseconds")]
    pub close_milliseconds: String,

    #[serde(rename = "closeDateTime")]
    pub close_date_time: DateTime<Utc>,

    #[serde(rename = "openAmount")]
    pub open_amount: String,

    #[serde(rename = "closeTradeId")]
    pub close_trade_id: String,

    #[serde(rename = "openTradeId")]
    pub open_trade_id: String,

    #[serde(rename = "closeOrderId")]
    pub close_order_id: String,

    #[serde(rename = "positionId")]
    pub position_id: String,
    #[serde(rename = "openOrderId")]
    pub open_order_id: String,
    #[serde(rename = "strategyId")]
    pub strategy_id: String,
    #[serde(rename = "slPrice")]
    pub sl_price: String,

    #[serde(rename = "slOrderType")]
    pub sl_order_type: SlOrderType,

    #[serde(rename = "slTrailingOffset")]
    pub sl_trailing_offset: String,

    #[serde(rename = "tpPrice")]
    pub tp_price: String,

    #[serde(rename = "commission")]
    pub commission: String,

    #[serde(rename = "swap")]
    pub swap: String,

    #[serde(rename = "profit")]
    pub profit: String,

    #[serde(rename = "netProfit")]
    pub net_profit: String,
}

/// Represents pagination links with associated parameters.
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    /// The URL for the next set of results.
    #[serde(rename = "next")]
    pub next: NextLink,
}

/// Represents the next link and its associated parameters.
#[derive(Debug, Serialize, Deserialize)]
pub struct NextLink {
    /// The URL for the next page.
    pub url: Option<String>,

    /// Parameters associated with the next link.
    pub params: NextLinkParams,
}

/// Represents the parameters for the next link.
#[derive(Debug, Serialize, Deserialize)]
pub struct NextLinkParams {
    /// The account number.
    pub acc_num: String,

    /// The last trade ID.
    pub last_trade_id: String,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum SlOrderType {
    #[strum(to_string = "STOP")]
    #[serde(rename = "STOP")]
    Stop,
    #[strum(to_string = "STOP_LIMIT")]
    #[serde(rename = "STOP_LIMIT")]
    StopLimit,
    #[strum(to_string = "TRAILING_STOP")]
    #[serde(rename = "TRAILING_STOP")]
    TrailingStop,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    #[strum(to_string = "Market")]
    #[serde(rename = "Market")]
    Market,

    #[strum(to_string = "Protective Stop")]
    #[serde(rename = "Protective Stop")]
    ProtectiveStop,

    #[strum(to_string = "Stop Loss")]
    #[serde(rename = "Stop Loss")]
    StopLoss,

    #[strum(to_string = "Stop")]
    #[serde(rename = "Stop")]
    Stop,

    #[strum(to_string = "Stop Out")]
    #[serde(rename = "Stop Out")]
    StopOut,

    #[strum(to_string = "Protective Limit")]
    #[serde(rename = "Protective Limit")]
    ProtectiveLimit,

    #[strum(to_string = "Take Profit")]
    #[serde(rename = "Take Profit")]
    TakeProfit,

    #[strum(to_string = "Limit")]
    #[serde(rename = "Limit")]
    Limit,

    #[strum(to_string = "Stop Limit")]
    #[serde(rename = "Stop Limit")]
    StopLimit,

    #[strum(to_string = "Trailing Stop Loss")]
    #[serde(rename = "Trailing Stop Loss")]
    TrailingStopLoss,

    #[strum(to_string = "Trailing Stop")]
    #[serde(rename = "Trailing Stop")]
    TrailingStop,

    #[strum(to_string = "Buy")]
    #[serde(rename = "Buy")]
    Buy,

    #[strum(to_string = "Sell")]
    #[serde(rename = "Sell")]
    Sell,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum ClosedPositionSide {
    #[strum(to_string = "Buy")]
    #[serde(rename = "Buy")]
    Buy,
    #[strum(to_string = "Sell")]
    #[serde(rename = "Sell")]
    Sell,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum OpenedPositionSide {
    #[strum(to_string = "BUY")]
    #[serde(rename = "BUY")]
    Buy,
    #[strum(to_string = "SELL")]
    #[serde(rename = "SELL")]
    Sell,
    #[strum(to_string = "SHORT_SELL")]
    #[serde(rename = "SHORT_SELL")]
    ShortSell,
    #[strum(to_string = "BUY_TO_COVER")]
    #[serde(rename = "BUY_TO_COVER")]
    BuyToConvert,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetClosedPositionsResponse {
    pub data: Vec<ClosedPositionModel>,
    /// Links to the next page of the report. Use params for the next page URL search params.
    pub links: Links,
}
