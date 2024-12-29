use serde::Deserialize;
use crate::BaseResponse;

use crate::macros::{
    impl_order_method,
    impl_limit_method,
    impl_adjusted_method,
    impl_window_method,
    impl_timestamp_method, impl_send_method
};
use crate::models::types::{SeriesType, Timespan, Comparator, Order};
use crate::models::aggs::Agg;

macro_rules! impl_timespan_method {
    () => {
        pub fn timespan(mut self, timespan: Timespan) -> Self {
            self.0 = self.0.query(&[("timespan", timespan)]);
            return self;
        }
    };
}


macro_rules! impl_series_type_method {
    () => {
        pub fn series_type(mut self, series_type: SeriesType) -> Self {
            self.0 = self.0.query(&[("series_type", series_type)]);
            self
        }
    };
}

macro_rules! impl_expand_underlying_method {
    () => {
        pub fn expand_underlying(mut self, expand_underlying: bool) -> Self {
            self.0 = self.0.query(&[("expand_underlying", expand_underlying)]);
            self
        }
    };
}

pub struct GetSMAParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetSMAParamsBuilder {
    impl_timespan_method!();
    impl_timestamp_method!();
    impl_series_type_method!();
    impl_expand_underlying_method!();
    impl_adjusted_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_window_method!();
    impl_send_method!(GetSMAResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetSMAResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<SingleIndicatorResults>
}

pub struct GetEMAParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetEMAParamsBuilder {
    impl_timespan_method!();
    impl_timestamp_method!();
    impl_series_type_method!();
    impl_expand_underlying_method!();
    impl_adjusted_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_window_method!();
    impl_send_method!(GetEMAResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetEMAResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<SingleIndicatorResults>
}

pub struct GetRSIParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetRSIParamsBuilder {
    impl_timespan_method!();
    impl_timestamp_method!();
    impl_series_type_method!();
    impl_expand_underlying_method!();
    impl_adjusted_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_window_method!();
    impl_send_method!(GetRSIResponse);
}


#[derive(Deserialize, Debug, Clone)]
pub struct GetRSIResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<SingleIndicatorResults>
}

pub struct GetMACDParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetMACDParamsBuilder {
    impl_timespan_method!();
    impl_timestamp_method!();
    impl_series_type_method!();
    impl_expand_underlying_method!();
    impl_adjusted_method!();
    impl_order_method!();
    impl_limit_method!();

    pub fn short_window(mut self, window: i64) -> Self {
        self.0 = self.0.query(&[("short_window", window)]);
        self
    }

    pub fn long_window(mut self, window: i64) -> Self {
        self.0 = self.0.query(&[("long_window", window)]);
        self
    }

    pub fn signal_window(mut self, window: i64) -> Self {
        self.0 = self.0.query(&[("signal_window", window)]);
        self
    }

    impl_send_method!(GetMACDResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetMACDResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<MACDIndicatorResults>
}

#[derive(Deserialize, Debug, Clone)]
pub struct MACDIndicatorResults {
    pub underlying: Option<UnderlyingResults>,
    pub values: Option<Vec<MACDIndicatorValue>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct MACDIndicatorValue {
    pub timestamp: Option<String>, // TODO: Millis
    pub value: Option<f64>,
    pub signal: Option<f64>,
    pub histogram: Option<f64>
}

#[derive(Deserialize, Debug, Clone)]
pub struct SingleIndicatorResults {
    pub underlying: Option<UnderlyingResults>,
    pub values: Option<SingleIndicatorValues>
}

#[derive(Deserialize, Debug, Clone)]
pub struct UnderlyingResults {
    pub aggregates: Option<Vec<Agg>>,
    pub url: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct SingleIndicatorValues {
    pub timestamp: Option<String>, //TODO: Millis
    pub value: Option<f64>
}
