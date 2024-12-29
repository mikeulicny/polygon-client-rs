use serde::Deserialize;
use crate::macros::{impl_sort_method, impl_order_method, impl_limit_method, impl_send_method, impl_as_of_method};
use crate::models::types::{Sort, Order};

use crate::BaseResponse;

use super::types::Comparator;

pub struct GetOptionsContractParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl GetOptionsContractParamsBuilder {
    impl_as_of_method!();
    impl_send_method!(GetOptionsContractResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetOptionsContractResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<OptionsContract>
}

#[derive(Deserialize, Debug, Clone)]
pub struct OptionsContract {
    pub additional_underlyings: Option<Vec<Underlying>>,
    pub cfi: Option<String>,
    pub contract_type: Option<String>,
    pub correction: Option<i32>,
    pub excercise_style: Option<String>,
    pub expiration_date: Option<String>, // TODO: Date
    pub primary_exchange: Option<String>,
    pub shares_per_contract: Option<f64>,
    pub strike_price: Option<f64>,
    pub ticker: Option<String>,
    pub underlying_ticker: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Underlying {
    pub amount: Option<f64>,
    #[serde(rename = "type")]
    pub underlying_type: Option<String>,
    pub underlying: Option<String>
}

pub struct ListOptionsContractsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl ListOptionsContractsParamsBuilder {
    pub fn underlying_ticker(mut self, comparator: Comparator, ticker: String) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("underlying_ticker", ticker)]),
            Comparator::LT => self.0.query(&[("underlying_ticker.lt", ticker)]),
            Comparator::LTE => self.0.query(&[("underlying_ticker.lte", ticker)]),
            Comparator::GT => self.0.query(&[("underlying_ticker.gt", ticker)]),
            Comparator::GTE => self.0.query(&[("underlying_ticker.gte", ticker)]),
        };
        self
    }

    pub fn contract_type(mut self, contract_type: String) -> Self {
        self.0 = self.0.query(&[("contract_type", contract_type)]);
        self
    }

    pub fn expiration_date(mut self, comparator: Comparator, date: String) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("expiration_date", date)]),
            Comparator::LT => self.0.query(&[("expiration_date.lt", date)]),
            Comparator::LTE => self.0.query(&[("expiration_date.lte", date)]),
            Comparator::GT => self.0.query(&[("expiration_date.gt", date)]),
            Comparator::GTE => self.0.query(&[("expiration_date.gte", date)]),
        };
        self
    }

    impl_as_of_method!();

    pub fn strike_price(mut self, comparator: Comparator, price: f64) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("strike_price", price)]),
            Comparator::LT => self.0.query(&[("strike_price.lt", price)]),
            Comparator::LTE => self.0.query(&[("strike_price.lte", price)]),
            Comparator::GT => self.0.query(&[("strike_price.gt", price)]),
            Comparator::GTE => self.0.query(&[("strike_price.gte", price)]),
        };
        self
    }

    pub fn expired(mut self, expired: bool) -> Self {
        self.0 = self.0.query(&[("expired", expired)]);
        self
    }

    impl_sort_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_send_method!(ListOptionsContractsResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListOptionsContractsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<OptionsContract>>
}
