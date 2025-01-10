use serde::Deserialize;
use serde_with::{serde_as, TimestampMilliSeconds, formats::Strict};
use chrono::{DateTime, Utc};
use crate::BaseResponse;
use crate::models::types::Order;
use crate::macros::{
    impl_limit_method,
    impl_order_method,
    impl_adjusted_method,
    impl_include_otc_method,
    impl_send_method
};

pub struct ListAggsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl ListAggsParamsBuilder {
    impl_adjusted_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_send_method!(ListAggsResponse);
}

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct Agg {
    #[serde(rename = "T")]
    pub exchange_symbol: Option<String>, // Exists in Grouped Daily
    #[serde(rename = "t")]
    #[serde_as(as = "TimestampMilliSeconds<i64, Strict>")]
    pub time_stamp: DateTime<Utc>,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "v")]
    pub volume: f64,
    #[serde(rename = "vw")]
    pub vwap: Option<f64>,
    #[serde(rename = "n")]
    pub transactions: Option<f64>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListAggsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub ticker: String,
    pub adjusted: bool,
    #[serde(rename = "queryCount")]
    pub query_count: i64,
    #[serde(rename = "resultsCount")]
    pub results_count: i64,
    pub results: Option<Vec<Agg>>,
}

pub struct GroupedDailyAggsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GroupedDailyAggsParamsBuilder {
    impl_include_otc_method!();
    impl_adjusted_method!();
    impl_send_method!(GroupedDailyAggsResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GroupedDailyAggsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub adjusted: bool,
    #[serde(rename = "queryCount")]
    pub query_count: i64,
    #[serde(rename = "resultsCount")]
    pub results_count: i64,
    pub results: Option<Vec<Agg>>,

    // TODO: is this used?
    pub ticker: Option<String>,
}

pub struct DailyOpenCloseAggsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl DailyOpenCloseAggsParamsBuilder {
    impl_adjusted_method!();
    impl_send_method!(DailyOpenCloseAggsResponse);
}


#[derive(Deserialize, Debug, Clone)]
pub struct DailyOpenCloseAggsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    #[serde(rename = "afterHours")]
    pub after_hours: Option<f64>,
    pub from: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub symbol: String,
    pub volume: f64,
    #[serde(rename = "preMarket")]
    pub premarket: Option<f64>,
    pub otc: Option<bool>
}

pub struct PreviousCloseAggsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl PreviousCloseAggsParamsBuilder {
    impl_adjusted_method!();
    impl_send_method!(PreviousCloseAggsResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct PreviousCloseAggsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub ticker: String,
    pub adjusted: bool,
    #[serde(rename = "queryCount")]
    pub query_count: i64,
    #[serde(rename = "resultsCount")]
    pub results_count: i64,
    pub results: Option<Vec<Agg>>,
}
