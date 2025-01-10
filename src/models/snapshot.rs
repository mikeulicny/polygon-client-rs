use std::collections::HashMap;
use serde::Deserialize;

use crate::{BaseResponse, macros::impl_send_method};
use crate::macros::impl_include_otc_method;
use crate::models::summaries::Session;

use super::types::Comparator;

#[derive(Deserialize, Debug, Clone)]
pub struct DaySnapshot {
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "v")]
    pub volume: f64,
    #[serde(rename = "vw")]
    pub volume_weighted_average: f64,
    pub otc: Option<bool>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastQuoteSnapshot {
    #[serde(rename = "P")]
    pub ask_price: f64,
    #[serde(rename = "p")]
    pub bid_price: f64,
    #[serde(rename = "S")]
    pub ask_size: f64,
    #[serde(rename = "s")]
    pub bid_size: f64,
    #[serde(rename = "t")]
    pub timestamp: String // TODO: Nanos
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastTradeSnapshot {
    #[serde(rename = "c")]
    pub conditions: Vec<i32>,
    #[serde(rename = "i")]
    pub trade_id: String,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: f64,
    #[serde(rename = "t")]
    pub timestamp: String, // TODO: Nanos
    #[serde(rename = "x")]
    pub exchange_id: i32
}

#[derive(Deserialize, Debug, Clone)]
pub struct MinuteSnapshot {
    #[serde(rename = "av")]
    pub accumulated_volume: f64,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "v")]
    pub volume: f64,
    #[serde(rename = "vw")]
    pub volume_weighted_average: f64,
    #[serde(rename = "n")]
    pub number_of_transactions: f64,
    #[serde(rename = "t")]
    pub timestamp: String, // TODO: Millis
    pub otc: Option<bool>
}

#[derive(Deserialize, Debug, Clone)]
pub struct TickerSnapshot {
    pub day: Option<DaySnapshot>,
    pub last_quote: Option<LastQuoteSnapshot>,
    pub last_trade: Option<LastTradeSnapshot>,
    pub minute: Option<MinuteSnapshot>,
    pub previous_day: Option<DaySnapshot>,
    pub ticker: Option<String>,
    pub todays_change: Option<f64>,
    pub todays_change_percent: Option<f64>,
    pub updated: Option<String> // TODO: Nanoseconds
}

#[derive(Deserialize, Debug, Clone)]
pub struct OptionContractSnapshot {
    pub break_even_price: Option<f64>,
    pub day: Option<DayOptionContractSnapshot>,
    pub details: Option<OptionDetails>,
    pub greeks: Option<Greeks>,
    pub implied_volatility: Option<f64>,
    pub last_quote: Option<LastQuoteOptionContractSnapshot>,
    pub last_trade: Option<LastTradeOptionContractSnapshot>,
    pub open_interes: Option<f64>,
    pub underlying_asset: Option<UnderlyingAsset>
}

pub struct GetAllTickersSnapshotParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetAllTickersSnapshotParamsBuilder {
    pub fn tickers(mut self, tickers: String) -> Self {
        self.0 = self.0.query(&[("tickers", tickers)]);
        self
    }

    impl_include_otc_method!();
    impl_send_method!(GetAllTickersSnapshotResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetAllTickersSnapshotResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub tickers: Option<Vec<TickerSnapshot>>
}

pub struct GetTickerSnapshotParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetTickerSnapshotParamsBuilder {
    impl_send_method!(GetTickerSnapshotResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetTickerSnapshotResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub ticker: Option<TickerSnapshot>
}

pub struct GetGainersLosersSnapshotParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetGainersLosersSnapshotParamsBuilder {
    impl_include_otc_method!();
    impl_send_method!(GetGainersLosersSnapshotResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetGainersLosersSnapshotResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub tickers: Option<Vec<TickerSnapshot>>
}

pub struct GetOptionContractSnapshotParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetOptionContractSnapshotParamsBuilder {
    impl_send_method!(GetOptionContractSnapshotResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetOptionContractSnapshotResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<OptionContractSnapshot>
}

pub struct GetIndicesSnapshotParamsBuilder {
    pub(crate) request_builder: reqwest::RequestBuilder,
    pub(crate) ticker_any_of: String,
}

impl GetIndicesSnapshotParamsBuilder {
    pub fn ticker_any_of(mut self, ticker: String) -> Self {
        self.ticker_any_of = self.ticker_any_of + "," + &ticker;
        self
    }

    pub async fn send(mut self) -> GetIndicesSnapshotResponse {
        self.request_builder = self.request_builder.query(&[("ticker.any_of", self.ticker_any_of)]);
        self.request_builder.send().await.expect("")
            .json::<GetIndicesSnapshotResponse>().await
            .expect("Json parse failed for GetIndicesSnapshotResponse")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetIndicesSnapshotResponse{
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<IndexSnapshot>>
}

pub struct ListUniversalSnapshotParamsBuilder {
    pub(crate) request_builder: reqwest::RequestBuilder,
    pub(crate) ticker_any_of: String
}

impl ListUniversalSnapshotParamsBuilder {
    pub fn ticker(mut self, comparator: Comparator, ticker: String) -> Self {
        self.request_builder = match comparator {
            Comparator::EQ => self.request_builder.query(&[("ticker", ticker)]),
            Comparator::LT => self.request_builder.query(&[("ticker.lt", ticker)]),
            Comparator::LTE => self.request_builder.query(&[("ticker.lte", ticker)]),
            Comparator::GT => self.request_builder.query(&[("ticker.gt", ticker)]),
            Comparator::GTE => self.request_builder.query(&[("ticker.gte", ticker)]),
        };
        self
    }

    pub fn ticker_type(mut self, ticker_type: String) -> Self {
        self.request_builder = self.request_builder.query(&[("type", ticker_type)]);
        self
    }

    pub fn ticker_any_of(mut self, ticker: String) -> Self {
        self.ticker_any_of = self.ticker_any_of + "," + &ticker;
        self
    }

    pub async fn send(mut self) -> GetIndicesSnapshotResponse {
        self.request_builder = self.request_builder.query(&[("ticker.any_of", self.ticker_any_of)]);
        self.request_builder.send().await.expect("")
            .json::<GetIndicesSnapshotResponse>().await
            .expect("Json parse failed for GetIndicesSnapshotResponse")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListUniversalSnapshotResponse{
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<SnapshotResponseModel>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct SnapshotResponseModel {
    pub break_even_price: Option<f64>,
    pub details: Option<Details>,
    pub error: Option<String>,
    // pub fmv: Option<f64>,   // business plan
    pub greeks: Option<Greeks>,
    pub implied_volatility: Option<f64>,
    pub last_quote: Option<SnapshotLastQuote>,
    pub last_trade: Option<SnapshotLastTrade>,
    pub market_status: Option<String>,
    pub message: Option<String>,
    pub name: Option<String>,
    pub open_interest: Option<f64>,
    pub session: Option<Session>,
    pub ticker: String,
    #[serde(rename = "type")]
    pub ticker_type: Option<String>,
    pub underlying_asset: Option<UnderlyingAsset>,
    pub value: Option<f64>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct SnapshotLastQuote {
    pub ask: f64,
    pub ask_exchange: Option<i32>,
    pub ask_size: Option<f64>,
    pub bid: f64,
    pub bid_exchange: Option<i32>,
    pub bid_size: Option<f64>,
    pub last_updated: i64,
    pub midpoint: Option<f64>,
    pub timeframe: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SnapshotLastTrade {
    pub conditions: Option<Vec<i32>>,
    pub exchange: Option<i32>,
    pub id: Option<String>,
    pub last_updated: i64,
    pub participant_timestamp: i64,
    pub price: f64,
    pub sip_timestamp: Option<i64>,
    pub size: u32,
    pub timeframe: String, // TODO: Enum: DELAYED, REAL-TIME
}

#[derive(Deserialize, Debug, Clone)]
pub struct Details {
    pub contract_type: String,
    pub exercise_style: String,
    pub expiration_date: String,
    pub shares_per_contract: f64,
    pub strike_price: f64
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderBookQuote {
    #[serde(rename = "p")]
    pub price: Option<f64>,
    #[serde(rename = "x")]
    pub exchange_to_shares: Option<HashMap<String, f64>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64
}

#[derive(Deserialize, Debug, Clone)]
pub struct UnderlyingAsset {
    pub change_to_break_even: f64,
    pub last_updated: i64, // Nanosecond timestamp
    pub price: Option<f64>,
    pub ticker: String,
    pub timeframe: String, // TODO: Enum: DELAYED, REAL-TIME
    pub value: Option<f64>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct IndexSnapshot {
    pub value: Option<f64>,
    pub ticker: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub index_type: Option<String>,
    pub market_status: Option<String>,
    pub session: Option<IndexSession>
}

#[derive(Deserialize, Debug, Clone)]
pub struct IndexSession {
    pub change: Option<f64>,
    pub change_percent: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub previous_close: Option<f64>
}

#[derive(Deserialize, Debug, Clone)]
pub struct DayOptionContractSnapshot {
    pub change: Option<f64>,
    pub change_percent: Option<f64>,
    pub close: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub open: Option<f64>,
    pub last_updated: Option<String>, // TODO: Nanos
    pub previous_close: Option<f64>,
    pub volume: Option<f64>,
    pub vwap: Option<f64>
}

#[derive(Deserialize, Debug, Clone)]
pub struct OptionDetails {
    pub contract_type: Option<String>,
    pub exercise_style: Option<String>,
    pub expiration_date: Option<String>, // TODO: Date
    pub shares_per_contract: Option<f64>,
    pub strike_price: Option<f64>,
    pub ticker: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastQuoteOptionContractSnapshot {
    pub ask: Option<f64>,
    pub ask_size: Option<f64>,
    pub bid: Option<f64>,
    pub bid_size: Option<f64>,
    pub last_updated: Option<String>, // TODO: Nanos
    pub midpoint: Option<f64>,
    pub timeframe: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastTradeOptionContractSnapshot {
    pub sip_timestamp: Option<String>, // TODO: Nanos
    pub conditions: Option<Vec<i32>>,
    pub price: Option<f64>,
    pub size: Option<f64>,
    pub exchange: Option<f64>,
    pub timeframe: Option<String>
}
