use serde::Deserialize;
use chrono::{DateTime, Utc};
use crate::BaseResponse;
use crate::macros::{impl_sort_method, impl_order_method, impl_limit_method, impl_locale_method, impl_asset_class_method, impl_ticker_method, impl_date_method, impl_send_method};
use crate::models::types::{AssetClass, Sort, Order, Comparator, MarketLocale};

#[derive(Deserialize, Clone, Debug)]
pub struct Ticker {
    pub active: Option<bool>,
    pub cik: Option<String>,
    pub composite_figi: Option<String>,
    pub currency_name: Option<String>,
    pub delisted_utc: Option<DateTime<Utc>>,
    pub last_updated_utc: Option<DateTime<Utc>>,
    pub locale: String,
    pub market: String,
    pub name: String,
    pub primary_exchange: Option<String>,
    pub share_class_figi: Option<String>,
    pub ticker: String,
    #[serde(rename = "type")]
    pub ticker_type: Option<String>,


    pub market_cap: Option<f64>,
    pub description: Option<String>,
    // used for stock
    pub share_class_shares_outstanding: Option<f64>,
    pub sic_code: Option<String>,
    pub sic_description: Option<String>,

    // used for forex/crypto
    pub currency_symbol: Option<String>,
    pub base_currency_symbol: Option<String>,
    pub base_currency_name: Option<String>
}

pub struct TickersParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl TickersParamsBuilder {
    impl_ticker_method!();

    pub fn ticker_type(mut self, t: String) -> Self {
        self.0 = self.0.query(&[("type", t)]);
        self
    }

    pub fn market(mut self, market: AssetClass) -> Self {
        self.0 = self.0.query(&[("market", market)]);
        self
    }

    pub fn exchange(mut self, exchange: String) -> Self {
        self.0 = self.0.query(&[("exchange", exchange)]);
        self
    }

    pub fn cusip(mut self, cusip: i32) -> Self {
        self.0 = self.0.query(&[("cusip", cusip)]);
        self
    }

    pub fn cik(mut self, cik: i32) -> Self {
        self.0 = self.0.query(&[("cik", cik)]);
        self
    }

    impl_date_method!();

    pub fn active(mut self, active: bool) -> Self {
        self.0 = self.0.query(&[("active", active)]);
        self
    }
    pub fn search(mut self, search: String) -> Self {
        self.0 = self.0.query(&[("search", search)]);
        self
    }

    impl_sort_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_send_method!(TickersResponse);
}

#[derive(Deserialize, Clone)]
pub struct TickersResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<Ticker>>,
}

pub struct TickerDetailsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl TickerDetailsParamsBuilder {
    impl_date_method!();
    impl_send_method!(TickerDetailsResponse);
}

#[derive(Deserialize, Clone, Debug)]
pub struct TickerDetailsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<TickerWithDetails>,
}

pub struct TickerNewsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl TickerNewsParamsBuilder {
    impl_ticker_method!();

    pub fn published_utc(mut self, comparator: Comparator, date: &str) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("published_utc", date)]),
            Comparator::LT => self.0.query(&[("published_utc.lt", date)]),
            Comparator::LTE => self.0.query(&[("published_utc.lte", date)]),
            Comparator::GT => self.0.query(&[("published_utc.gt", date)]),
            Comparator::GTE => self.0.query(&[("published_utc.gte", date)]),
        };
        self
    }

    impl_sort_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_send_method!(TickerNewsResponse);
}

#[derive(Deserialize, Clone)]
pub struct Publisher {
    pub favicon_url: Option<String>,
    pub homepage_url: String,
    pub logo_url: String,
    pub name: String
}

#[derive(Deserialize, Clone)]
pub struct TickerNews {
    pub amp_url: Option<String>,
    pub article_url: String,
    pub author: String,
    pub description: Option<String>,
    pub id: String,
    pub image_url: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub published_utc: DateTime<Utc>,
    pub publisher: Publisher,
    pub tickers: Vec<String>,
    pub title: String
}

#[derive(Deserialize, Clone)]
pub struct TickerNewsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<TickerNews>>
}

pub struct TickerTypesParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl TickerTypesParamsBuilder {
    impl_asset_class_method!();
    impl_locale_method!();
    impl_send_method!(TickerTypesResponse);
}

#[derive(Deserialize, Clone)]
pub struct TickerType {
    pub asset_class: String,
    pub code: String,
    pub description: String,
    pub locale: String
}

#[derive(Deserialize, Clone)]
pub struct TickerTypesResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<TickerType>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Address {
    pub address1: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub state: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Branding {
    pub icon_url: Option<String>,
    pub logo_url: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct TickerWithDetails {
    pub active: bool,
    pub address: Option<Address>,
    pub branding: Option<Branding>,
    pub cik: Option<String>,
    pub composite_figi: Option<String>,
    pub currency_name: String,
    pub delisted_utc: Option<String>,
    pub description: Option<String>,
    pub homepage_url: Option<String>,
    pub list_date: Option<String>,
    pub locale: String,
    pub market: String,
    pub market_cap: Option<f64>,
    pub name: String,
    pub phone_number: Option<String>,
    pub primary_exchange: Option<String>,
    pub round_lot: Option<f64>,
    pub share_class_figi: Option<String>,
    pub share_class_shares_outstanding: Option<f64>,
    pub sic_code: Option<String>,
    pub sic_description: Option<String>,
    pub ticker: String,
    pub ticker_root: Option<String>,
    pub ticker_suffix: Option<String>,
    pub total_employees: Option<i64>,
    #[serde(rename = "type")]
    pub ticker_type: Option<String>,
    pub weighted_shares_outstanding: Option<f64>
}
