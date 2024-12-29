use crate::BaseResponse;
use crate::macros::{impl_timestamp_method, impl_order_method, impl_limit_method, impl_sort_method, impl_send_method};
use crate::models::types::{Order, Sort, Comparator};

use chrono::prelude::{DateTime, Utc};
use serde::Deserialize;
use serde_with::{serde_as, TimestampNanoSeconds, formats::Strict};

pub struct ListQuotesParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl ListQuotesParamsBuilder {
    impl_timestamp_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_sort_method!();
    impl_send_method!(ListQuotesResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListQuotesResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<Quote>>
}

pub struct GetLastQuoteParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetLastQuoteParamsBuilder {
    impl_send_method!(GetLastQuoteResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetLastQuoteResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<LastQuote>
}

pub struct GetLastForexQuoteParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetLastForexQuoteParamsBuilder {
    impl_send_method!(GetLastForexQuoteResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetLastForexQuoteResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub last: Option<ForexQuote>
}

pub struct GetRealTimeCurrencyConversionParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetRealTimeCurrencyConversionParamsBuilder {
    impl_send_method!(GetRealTimeCurrencyConversionResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetRealTimeCurrencyConversionResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    #[serde(rename = "initialAmount")]
    pub initial_amount: Option<f64>,
    pub converted: Option<f64>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub last: Option<ForexQuote>
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct Quote {
    pub ask_exchange: Option<i64>,
    pub ask_price: Option<f64>,
    pub ask_size: Option<f64>,
    pub bid_exchange: Option<i64>,
    pub bid_price: Option<f64>,
    pub bid_size: Option<f64>,
    pub conditions: Option<Vec<i32>>,
    pub indicators: Option<Vec<i32>>,
    #[serde_as(as = "TimestampNanoSeconds<i64, Strict>")]
    pub participant_timestamp: DateTime<Utc>,
    pub sequence_number: i64,
    #[serde_as(as = "TimestampNanoSeconds<i64, Strict>")]
    pub sip_timestamp: DateTime<Utc>,
    pub tape: Option<i32>,
    #[serde_as(as = "Option<TimestampNanoSeconds<i64, Strict>>")]
    pub trf_timestamp: Option<DateTime<Utc>>
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct LastQuote {
    #[serde(rename = "T")]
    pub ticker: String,
    #[serde(rename = "f")]
    #[serde_as(as = "Option<TimestampNanoSeconds<i64, Strict>>")]
    pub trf_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "q")]
    pub sequence_number: i64,
    #[serde(rename = "t")]
    #[serde_as(as = "TimestampNanoSeconds<i64, Strict>")]
    pub sip_timestamp: DateTime<Utc>,
    #[serde(rename = "y")]
    #[serde_as(as = "TimestampNanoSeconds<i64, Strict>")]
    pub participant_timestamp: DateTime<Utc>,
    #[serde(rename = "P")]
    pub ask_price: Option<f64>,
    #[serde(rename = "S")]
    pub ask_size: Option<f64>,
    #[serde(rename = "X")]
    pub ask_exchange: Option<i64>,
    #[serde(rename = "c")]
    pub conditions: Option<Vec<i32>>,
    #[serde(rename = "i")]
    pub indicators: Option<Vec<i32>>,
    #[serde(rename = "p")]
    pub bid_price: Option<f64>,
    #[serde(rename = "s")]
    pub bid_size: Option<f64>,
    #[serde(rename = "x")]
    pub bid_exchange: Option<i64>,
    #[serde(rename = "z")]
    pub tape: Option<i32>
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct ForexQuote {
    pub ask: Option<f64>,
    pub bid: Option<f64>,
    pub exchange: Option<i32>,
    #[serde_as(as = "Option<TimestampNanoSeconds<i64, Strict>>")]
    pub timestamp: Option<DateTime<Utc>>
}
