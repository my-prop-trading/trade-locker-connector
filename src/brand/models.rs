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
pub struct GetAccountResponse {
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
    pub account_type: String,

    /// The status of the account (e.g., ACTIVE, RESTRICTED, SUSPENDED).
    /// If ACTIVE, trading could be disabled by a risk rule.
    /// Check the 'tradingDisabledReason' property.
    #[serde(rename = "status")]
    pub status: String,

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
