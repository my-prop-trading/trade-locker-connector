use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_derive::Deserialize;
use crate::models::AccountType;

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
    pub position_ids: Vec<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
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
pub struct GetClosedTradesReportRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "type")]
    pub account_type: AccountType,
    /// Cursor to fetch the next page of events. If not provided, the first page of events will be returned.
    /// Must be an integer string, greater than or equal to 0 and less than or equal to 9223372036854775807
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Page size; max 1000, default 20.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

pub fn get_default_cursor() -> String {
    "9223372036854775807".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClosedTradeReportModel {
    #[serde(rename = "instrument")]
    pub instrument: String,

    #[serde(rename = "openMilliseconds")]
    pub open_milliseconds: String,

    //#[serde(rename = "openDateTime")]
    //pub open_date_time: DateTime<Utc>,
    #[serde(rename = "orderType")]
    pub order_type: String,

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

    //#[serde(rename = "closeDateTime")]
    //pub close_date_time: DateTime<Utc>,
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
    pub strategy_id: Option<String>,
    #[serde(rename = "slPrice")]
    pub sl_price: Option<String>,

    #[serde(rename = "slOrderType")]
    pub sl_order_type: Option<SlOrderType>,
    #[serde(rename = "slTrailingOffset")]
    pub sl_trailing_offset: Option<String>,
    #[serde(rename = "tpPrice")]
    pub tp_price: Option<String>,
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
pub struct PageLinks {
    /// The URL for the next set of results.
    #[serde(rename = "next")]
    pub next: Option<NextPageLink>,
}

/// Represents the next link and its associated parameters.
#[derive(Debug, Serialize, Deserialize)]
pub struct NextPageLink {
    /// The URL for the next page.
    pub url: Option<String>,

    /// Parameters associated with the next link.
    pub params: NextPageLinkParams,
}

/// Represents the parameters for the next link.
#[derive(Debug, Serialize, Deserialize)]
pub struct NextPageLinkParams {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "type")]
    pub account_type: AccountType,
    pub cursor: String,
    pub limit: u32,
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
pub enum PositionOrderType {
    #[strum(to_string = "Market")]
    #[serde(rename = "Market")]
    Market,

    #[strum(to_string = "Protective stop")]
    #[serde(rename = "Protective stop")]
    ProtectiveStop,

    #[strum(to_string = "Stop loss")]
    #[serde(rename = "Stop loss")]
    StopLoss,

    #[strum(to_string = "Stop")]
    #[serde(rename = "Stop")]
    Stop,

    #[strum(to_string = "Stop Out")]
    #[serde(rename = "Stop Out")]
    StopOut,

    #[strum(to_string = "Protective limit")]
    #[serde(rename = "Protective limit")]
    ProtectiveLimit,

    #[strum(to_string = "Take profit")]
    #[serde(rename = "Take profit")]
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
pub struct GetClosedTradesReportResponse {
    pub data: Vec<ClosedTradeReportModel>,
    /// Links to the next page of the report. Use params for the next page URL search params.
    pub links: PageLinks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGroupsRequest {
    #[serde(rename = "type")]
    pub account_type: AccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupModel {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetGroupsResponse {
    pub data: Vec<GroupModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountsReportRequest {
    #[serde(rename = "type")]
    pub account_type: AccountType,
    #[serde(rename = "accountIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "accountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<AccountStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAccountsReportResponse {
    pub data: Vec<AccountReportModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountReportModel {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub balance: String,
    pub credit: String,
    pub equity: String,
    pub pnl: String,
    #[serde(rename = "marginUsed")]
    pub margin_used: String,
    #[serde(rename = "marginAvailable")]
    pub margin_available: String,
    #[serde(rename = "userGroupId")]
    pub user_group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetApiStatusResponse {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTradesReportRequest {
    #[serde(rename = "type")]
    pub account_type: AccountType,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Start time in ISO format. 2021-12-31T23:59:59.999Z
    #[serde(rename = "startDateTime")]
    pub start_date_time: String,
    /// End time in ISO format. 2021-12-31T23:59:59.999Z
    #[serde(rename = "endDateTime")]
    pub end_date_time: String,
    #[serde(rename = "enableSLTP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sl_tp: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTradesReportResponse {
    pub data: Vec<TradeReportModel>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeReportModel {
    #[serde(rename = "tradeId")]
    pub trade_id: String,

    #[serde(rename = "orderId")]
    pub order_id: String,

    #[serde(rename = "accountId")]
    pub account_id: String,

    #[serde(rename = "side")]
    pub side: TradeReportSide,

    #[serde(rename = "orderType")]
    pub order_type: String,

    #[serde(rename = "positionStatus")]
    pub position_status: TradeReportPositionStatus,

    #[serde(rename = "tradeTime")]
    #[deprecated(note = "Use tradeDateTime instead.")]
    pub trade_time: i64, // trade time in milliseconds since Unix epoch

    #[serde(rename = "tradeDateTime")]
    pub trade_date_time: String, // ISO format

    #[serde(rename = "price")]
    pub price: String,

    #[serde(rename = "lots")]
    pub lots: String,

    #[serde(rename = "instrument")]
    pub instrument: String,

    #[serde(rename = "positionId")]
    pub position_id: String,

    #[serde(rename = "pnl")]
    pub pnl: String,

    #[serde(rename = "executionFee")]
    pub execution_fee: String,

    #[serde(rename = "stopLoss")]
    pub stop_loss: Option<String>,

    #[serde(rename = "stopLossLimit")]
    pub stop_loss_limit: Option<String>,

    #[serde(rename = "takeProfit")]
    pub take_profit: Option<String>,
}

// Enums for trade sides, order types, and position status
#[derive(strum::Display, Serialize, Deserialize, Debug, Clone)]
pub enum TradeReportSide {
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
    BuyToCover,
}

#[derive(strum::Display, Serialize, Deserialize, Debug, Clone)]
pub enum TradeReportOrderType {
    Market,
    ProtectiveStop,
    StopLoss,
    Stop,
    StopOut,
    ProtectiveLimit,
    TakeProfit,
    Limit,
    StopLimit,
    TrailingStopLoss,
    TrailingStop,
}

#[derive(strum::Display, Serialize, Deserialize, Debug, Clone)]
pub enum TradeReportPositionStatus {
    #[strum(to_string = "CLOSE")]
    #[serde(rename = "CLOSE")]
    Close,
    #[strum(to_string = "OPEN")]
    #[serde(rename = "OPEN")]
    Open,
    #[strum(to_string = "INCREASE")]
    #[serde(rename = "INCREASE")]
    Increase,
    #[strum(to_string = "DECREASE")]
    #[serde(rename = "DECREASE")]
    Decrease,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetsRequest {
    #[serde(rename = "type")]
    pub account_type: AccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetModel {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetsResponse {
    pub data: Vec<AssetModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    #[serde(rename = "type")]
    pub account_type: AccountType,
    #[serde(rename = "orderId")]
    pub order_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOrdersRequest {
    #[serde(rename = "type")]
    pub account_type: AccountType,
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    /// Default is 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Default is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOrdersResponse {
    pub data: Vec<OrderModel>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModel {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "lotSize")]
    pub lot_size: String,
    #[serde(rename = "averageFilledPrice")]
    pub average_filled_price: Option<String>,
    #[serde(rename = "createdDateTime")]
    pub created_date_time: String,
    #[serde(rename = "expireDateTime")]
    pub expire_date_time: Option<String>,
    #[serde(rename = "expireTime")]
    pub expire_time: Option<String>,
    #[serde(rename = "filledAmount")]
    pub filled_amount: String,
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "positionId")]
    pub position_id: Option<String>,
    #[serde(rename = "price")]
    pub price: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "slLimitPrice")]
    pub sl_limit_price: Option<String>,
    #[serde(rename = "slPrice")]
    pub sl_price: Option<String>,
    #[serde(rename = "slPriceType")]
    pub sl_price_type: String,
    #[serde(rename = "status")]
    pub status: OrderStatus,
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,
    /// Time-in-force of the order.
    #[serde(rename = "tif")]
    pub tif: String,
    #[serde(rename = "tpPrice")]
    pub tp_price: Option<String>,
    #[serde(rename = "tpPriceType")]
    pub tp_price_type: String,
    #[serde(rename = "instrument")]
    pub instrument: String,
    #[serde(rename = "type")]
    pub order_type: OrderType,
}

#[derive(strum::Display, Serialize, Deserialize, Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum OrderStatus {
    #[strum(to_string = "STATUS_NONE")]
    #[serde(rename = "STATUS_NONE")]
    None,

    #[strum(to_string = "STATUS_PENDING_NEW")]
    #[serde(rename = "STATUS_PENDING_NEW")]
    PendingNew,

    #[strum(to_string = "STATUS_PENDING_EXECUTION")]
    #[serde(rename = "STATUS_PENDING_EXECUTION")]
    PendingExecution,

    #[strum(to_string = "STATUS_PENDING_CANCEL")]
    #[serde(rename = "STATUS_PENDING_CANCEL")]
    PendingCancel,

    #[strum(to_string = "STATUS_PENDING_REPLACE")]
    #[serde(rename = "STATUS_PENDING_REPLACE")]
    PendingReplace,

    #[strum(to_string = "STATUS_PENDING_REPLACE_NOT_ACTIVE")]
    #[serde(rename = "STATUS_PENDING_REPLACE_NOT_ACTIVE")]
    PendingReplaceNotActive,

    #[strum(to_string = "STATUS_NEW")]
    #[serde(rename = "STATUS_NEW")]
    New,

    #[strum(to_string = "STATUS_ACCEPTED")]
    #[serde(rename = "STATUS_ACCEPTED")]
    Accepted,

    #[strum(to_string = "STATUS_REPLACED")]
    #[serde(rename = "STATUS_REPLACED")]
    Replaced,

    #[strum(to_string = "STATUS_PART_FILLED")]
    #[serde(rename = "STATUS_PART_FILLED")]
    PartFilled,

    #[strum(to_string = "STATUS_FILLED")]
    #[serde(rename = "STATUS_FILLED")]
    Filled,

    #[strum(to_string = "STATUS_CANCELED")]
    #[serde(rename = "STATUS_CANCELED")]
    Canceled,

    #[strum(to_string = "STATUS_REFUSED")]
    #[serde(rename = "STATUS_REFUSED")]
    Refused,

    #[strum(to_string = "STATUS_RESTATED")]
    #[serde(rename = "STATUS_RESTATED")]
    Restated,

    #[strum(to_string = "EXEC_TYPE_ACTIVATED")]
    #[serde(rename = "EXEC_TYPE_ACTIVATED")]
    Activated,

    #[strum(to_string = "STATUS_WAITING_MARKET")]
    #[serde(rename = "STATUS_WAITING_MARKET")]
    WaitingMarket,

    #[strum(to_string = "STATUS_OFF_MARKET")]
    #[serde(rename = "STATUS_OFF_MARKET")]
    OffMarket,

    #[strum(to_string = "STATUS_UNPLACED")]
    #[serde(rename = "STATUS_UNPLACED")]
    Unplaced,

    #[strum(to_string = "STATUS_REMOVED")]
    #[serde(rename = "STATUS_REMOVED")]
    Removed,

    #[strum(to_string = "STATUS_MODIFY_TRADING_MODE")]
    #[serde(rename = "STATUS_MODIFY_TRADING_MODE")]
    ModifyTradingMode,
}

#[derive(strum::Display, Serialize, Deserialize, Debug, Clone, Eq, PartialOrd, PartialEq)]
pub enum OrderType {
    #[strum(to_string = "MANUAL")]
    #[serde(rename = "MANUAL")]
    Manual,

    #[strum(to_string = "MARKET")]
    #[serde(rename = "MARKET")]
    Market,

    #[strum(to_string = "STOP")]
    #[serde(rename = "STOP")]
    Stop,

    #[strum(to_string = "LIMIT")]
    #[serde(rename = "LIMIT")]
    Limit,

    #[strum(to_string = "STOP_LIMIT")]
    #[serde(rename = "STOP_LIMIT")]
    StopLimit,

    #[strum(to_string = "TRAILING_STOP")]
    #[serde(rename = "TRAILING_STOP")]
    TrailingStop,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountOperationRequest {
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Amount of the operation. Must be positive.
    pub amount: String,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountOperationResponse {
    #[serde(rename = "operationId")]
    pub operation_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonthlyActiveAccountsRequest {
    /// Activity for set month. ISO format YYYY-MM. All dates are in UTC. 2021-08
    #[serde(rename = "forMonth")]
    pub for_month: String,
    /// Return data as Json or binary CSV.
    #[serde(rename = "returnType")]
    pub return_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonthlyActiveAccountsResponse {
    pub data: Vec<MonthlyActiveAccountModel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonthlyActiveAccountModel {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub group: String,
    pub sessions: i32,
    pub events: i32,
    pub accounts: i32,
    #[serde(rename = "openPositions")]
    pub open_positions: i32,
    pub orders: i32,
}
