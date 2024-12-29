use serde::Deserialize;
use chrono::NaiveDate;
use crate::BaseResponse;
use crate::macros::{impl_order_method, impl_limit_method, impl_sort_method, impl_ticker_method, impl_send_method};
use crate::models::types::{Sort, Order, Comparator, Frequency, DividendType};

pub struct DividendsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl DividendsParamsBuilder {
    impl_ticker_method!();

    pub fn ex_dividend_date(mut self, comparator: Comparator, date: String) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("ex_dividend_date", date)]),
            Comparator::LT => self.0.query(&[("ex_dividend_date.lt", date)]),
            Comparator::LTE => self.0.query(&[("ex_dividend_date.lte", date)]),
            Comparator::GT => self.0.query(&[("ex_dividend_date.gt", date)]),
            Comparator::GTE => self.0.query(&[("ex_dividend_date.gte", date)]) 
        };
        self
    }

    pub fn record_date(mut self, comparator: Comparator, date: String) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("record_date", date)]),
            Comparator::LT => self.0.query(&[("record_date.lt", date)]),
            Comparator::LTE => self.0.query(&[("record_date.lte", date)]),
            Comparator::GT => self.0.query(&[("record_date.gt", date)]),
            Comparator::GTE => self.0.query(&[("record_date.gte", date)])
        };
        self
    }

    pub fn declaration_date(mut self, comparator: Comparator, date: String) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("declaration_date", date)]),
            Comparator::LT => self.0.query(&[("declaration_date.lt", date)]),
            Comparator::LTE => self.0.query(&[("declaration_date.lte", date)]),
            Comparator::GT => self.0.query(&[("declaration_date.gt", date)]),
            Comparator::GTE => self.0.query(&[("declaration_date.gte", date)])
        };
        self
    }
    
    pub fn pay_date(mut self, comparator: Comparator, date: String) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("pay_date", date)]),
            Comparator::LT => self.0.query(&[("pay_date.lt", date)]),
            Comparator::LTE => self.0.query(&[("pay_date.lte", date)]),
            Comparator::GT => self.0.query(&[("pay_date.gt", date)]),
            Comparator::GTE => self.0.query(&[("pay_date.gte", date)])
        };
        self
    }
    
    pub fn frequency(mut self, frequency: Frequency) -> Self {
        self.0 = self.0.query(&[("frequency", frequency)]);
        self
    }

    pub fn cash_amount(mut self, comparator: Comparator, cash: f64) -> Self {
        self.0 = match comparator {
            Comparator::EQ => self.0.query(&[("cash_amount", cash)]),
            Comparator::LT => self.0.query(&[("cash_amount.lt", cash)]),
            Comparator::LTE => self.0.query(&[("cash_amount.lte", cash)]),
            Comparator::GT => self.0.query(&[("cash_amount.gt", cash)]),
            Comparator::GTE => self.0.query(&[("cash_amount.gte", cash)])
        };
        self
    }

    pub fn dividend_type(mut self, dividend_type: DividendType) -> Self {
        self.0 = self.0.query(&[("dividend_type", dividend_type)]);
        self
    }

    impl_order_method!();
    impl_limit_method!();
    impl_sort_method!();
    impl_send_method!(DividendsResponse);
}

#[derive(Deserialize, Debug, Clone)]
pub struct Dividend {
    pub cash_amount: f64,
    pub currency: Option<String>,
    pub declaration_date: Option<NaiveDate>,
    pub dividend_type: String,
    pub ex_dividend_date: String,
    pub frequency: i64,
    pub pay_date: Option<NaiveDate>,
    pub record_date: Option<NaiveDate>,
    pub ticker: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct DividendsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Vec<Dividend>>
}
