use std::collections::HashMap;
use serde::Deserialize;
use chrono::{DateTime, Utc, NaiveDate};

use crate::macros::impl_send_method;

pub struct MarketHolidaysParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl MarketHolidaysParamsBuilder {
    impl_send_method!(MarketHolidaysResponse);
}

pub struct MarketStatusParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl MarketStatusParamsBuilder {
    impl_send_method!(MarketStatusResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct Exchanges {
    pub nyse: String,
    pub nasdaq: String,
    pub otc: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Currencies {
    pub fx: String,
    pub crypto: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct MarketStatusResponse {
    pub market: String,
    #[serde(rename = "earlyHours")]
    pub early_hours: bool,
    #[serde(rename = "afterHours")]
    pub after_hours: bool,
    #[serde(rename = "serverTime")]
    pub server_time: DateTime<Utc>,
    pub exchanges: HashMap<String, String>,
    pub currencies: HashMap<String, String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct MarketHoliday {
    pub exchange: Option<String>,
    pub name: Option<String>,
    pub date: Option<NaiveDate>,
    pub status: Option<String>,
    pub open: Option<DateTime<Utc>>,
    pub close: Option<DateTime<Utc>>
}

pub type MarketHolidaysResponse = Vec<MarketHoliday>;
