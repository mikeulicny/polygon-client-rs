use serde::Deserialize;
use chrono::NaiveDate;
use crate::BaseResponse;
use crate::macros::{impl_sort_method, impl_order_method, impl_limit_method, impl_ticker_method, impl_send_method};
use crate::models::types::{Order, Sort, Comparator};

pub struct SplitsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl SplitsParamsBuilder {
    impl_ticker_method!();

    pub fn execution_date(mut self, comparator: Comparator, date: String) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("execution_date", date)]),
            Comparator::LT => self.0.query(&[("execution_date.lt", date)]),
            Comparator::LTE => self.0.query(&[("execution_date.lte", date)]),
            Comparator::GT => self.0.query(&[("execution_date.gt", date)]),
            Comparator::GTE => self.0.query(&[("execution_date.gte", date)])
        };
        self
    }

    pub fn reverse_split(mut self, split: bool) -> Self {
        self.0 = self.0.query(&[("reverse_split",split)]);
        self
    }

    impl_sort_method!();
    impl_order_method!();
    impl_limit_method!();
    impl_send_method!(SplitsResponse);
}

#[derive(Deserialize)]
pub struct Split {
    pub execution_date: Option<NaiveDate>,
    pub split_from: f64,
    pub split_to: f64,
    pub ticker: Option<String>
}

#[derive(Deserialize)]
pub struct SplitsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<Split>>
}
