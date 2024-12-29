use serde::Deserialize;
use serde_with::{serde_as, TimestampNanoSeconds, formats::Strict};
use chrono::{DateTime, Utc};
use crate::BaseResponse;
use crate::macros::{impl_timestamp_method, impl_order_method, impl_limit_method, impl_sort_method, impl_send_method};
use crate::models::types::{Comparator, Order, Sort};

pub struct TradesParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl TradesParamsBuilder {
    impl_timestamp_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_sort_method!();
    impl_send_method!(TradesResponse);
}

#[serde_as]
#[derive(Deserialize)]
pub struct Trade {
    pub conditions: Option<Vec<i32>>,
    pub correction: Option<i64>,
    pub exchange: i64,
    pub id: String,
    #[serde_as(as = "TimestampNanoSeconds<i64, Strict>")]
    pub participant_timestamp: DateTime<Utc>,
    pub price: f64,
    pub sequence_number: i64,
    #[serde_as(as = "TimestampNanoSeconds<i64, Strict>")]
    pub sip_timestamp: DateTime<Utc>,
    pub size: i64,
    pub tape: Option<i8>, // 1 = NYSE, 2 = NYSE ARCA/NYSE American, 3 = NASDAQ
    pub trf_id: Option<i64>,
    #[serde_as(as = "Option<TimestampNanoSeconds<i64, Strict>>")]
    pub trf_timestamp: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct TradesResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<Trade>>
}

pub struct LastTradeParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl LastTradeParamsBuilder {
    impl_send_method!(LastTradeResponse);
}

#[serde_as]
#[derive(Deserialize)]
pub struct LastTrade {
    #[serde(rename = "T")]
    pub ticker: String,
    #[serde(rename = "c")]
    pub conditions: Option<Vec<i32>>,
    #[serde(rename = "e")]
    pub correction: Option<u32>,
    #[serde(rename = "f")]
    #[serde_as(as = "Option<TimestampNanoSeconds<i64, Strict>>")]
    pub trf_timestamp: Option<DateTime<Utc>>,
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "q")]
    pub sequence_number: i64,
    #[serde(rename = "r")]
    pub trf: Option<i32>,
    #[serde(rename = "s")]
    pub size: Option<u32>,
    #[serde(rename = "t")]
    #[serde_as(as = "TimestampNanoSeconds<i64, Strict>")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "x")]
    pub exchange: i32,
    #[serde(rename = "y")]
    #[serde_as(as = "TimestampNanoSeconds<i64, Strict>")]
    pub participant_timestamp: DateTime<Utc>,
    #[serde(rename = "z")]
    pub tape: Option<i32>
}

#[derive(Deserialize)]
pub struct LastTradeResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: LastTrade
}

pub struct LastCryptoTradeParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl LastCryptoTradeParamsBuilder {
    impl_send_method!(LastCryptoTradeResponse);
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct CryptoTrade {
    pub conditions: Option<Vec<i32>>,
    pub exchange: Option<i32>,
    pub price: Option<f64>,
    pub size: Option<f64>,
    #[serde_as(as = "Option<TimestampNanoSeconds<i64, Strict>>")]
    pub timestamp: Option<DateTime<Utc>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LastCryptoTradeResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub symbol: Option<String>,
    pub last: Option<CryptoTrade>
}
