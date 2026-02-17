use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostbackRequest {
    // Input one of the following options, based on how many steps purchased challenge has
    #[serde(rename = "customParams.advS1")]
    pub steps: PostbackSteps,
    #[serde(rename = "customParams.advS2")]
    // Input account balance for the challenge purchased
    pub balance: u32,
    #[serde(rename = "customParams.advS3")]
    pub trading_platform: PostbackTradingPlatform,
    #[serde(rename = "customParams.advS4")]
    // Input challenge price base price in USD, before any discounts are applied - without currency symbol
    pub full_price: f64,
    #[serde(rename = "amount.value")]
    // Input challenge price final price in USD, after discounts are applied - without currency symbol
    pub final_price: f64,
    #[serde(rename = "customParams.advS5")]
    // Input the ISO code from trader’s country
    pub country: String,
    #[serde(rename = "externalId")]
    // Input your unique order or transaction ID - in order to be easily debug
    pub external_id: String,
    // Input the clickId that we passed you in the referring link
    pub cid: String,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum PostbackSteps {
    #[strum(to_string = "instant")]
    #[serde(rename = "instant")]
    Instant,

    #[strum(to_string = "1-step")]
    #[serde(rename = "1-step")]
    OneStep,

    #[strum(to_string = "2-step")]
    #[serde(rename = "2-step")]
    TwoStep,

    #[strum(to_string = "3-step")]
    #[serde(rename = "3-step")]
    ThreeStep,

    #[strum(to_string = "4-step")]
    #[serde(rename = "4-step")]
    FourStep,
}

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum PostbackTradingPlatform {
    #[strum(to_string = "tradelocker")]
    #[serde(rename = "tradelocker")]
    Tradelocker,

    #[strum(to_string = "mt4")]
    #[serde(rename = "mt4")]
    Mt4,

    #[strum(to_string = "mt5")]
    #[serde(rename = "mt5")]
    Mt5,

    #[strum(to_string = "tradingview")]
    #[serde(rename = "tradingview")]
    Tradingview,

    #[strum(to_string = "ctrader")]
    #[serde(rename = "ctrader")]
    Ctrader,

    #[strum(to_string = "matchtrader")]
    #[serde(rename = "matchtrader")]
    Matchtrader,

    #[strum(to_string = "oxtrade")]
    #[serde(rename = "oxtrade")]
    Oxtrade,

    #[strum(to_string = "volumetrica")]
    #[serde(rename = "volumetrica")]
    Volumetrica,

    #[strum(to_string = "thinktrader")]
    #[serde(rename = "thinktrader")]
    Thinktrader,

    #[strum(to_string = "other")]
    #[serde(rename = "other")]
    Other,
}
