use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::macros::{impl_order_method, impl_limit_method, impl_sort_method, impl_send_method};
use crate::BaseResponse;
use crate::models::types::{Order, Sort, Timeframe};

pub struct StockFinancialsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl StockFinancialsParamsBuilder {
    pub fn ticker(mut self, ticker: &str) -> Self {
        self.0 = self.0.query(&[("ticker", ticker)]);
        return self;
    }

    pub fn cik(mut self, cik: &str) -> Self {
        self.0 = self.0.query(&[("cik", cik)]);
        return self;
    }

    pub fn company_name(mut self, name: &str) -> Self {
        self.0 = self.0.query(&[("company_name", name)]);
        return self;
    }

    pub fn sic(mut self, sic: &str) -> Self {
        self.0 = self.0.query(&[("sic", sic)]);
        return self;
    }

    pub fn filing_date(mut self, date: &str) -> Self {
        self.0 = self.0.query(&[("filing_date", date)]);
        return self;
    }

    pub fn period_of_report_date(mut self, date: &str) -> Self {
        self.0 = self.0.query(&[("period_of_report_date", date)]);
        return self;
    }

    pub fn timeframe(mut self, timeframe: Timeframe) -> Self {
        self.0 = self.0.query(&[("timeframe", timeframe)]);
        return self;
    }

    pub fn include_sources(mut self, include: bool) -> Self {
        self.0 = self.0.query(&[("include_sources", include)]);
        return self;
    }

    impl_order_method!();
    impl_limit_method!();
    impl_sort_method!();
    impl_send_method!(StockFinancialsResponse);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Financial {
    // pub derived_from: Option<Vec<String>>, // only when data is from xbrl, include_sources=true, and source is SourceInterReportDerived
    pub formula: Option<String>,
    pub label: String,
    pub order: i32,
    pub unit: String,
    pub value: f64,
    pub xpath: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FinancialStatements {
    pub balance_sheet: Option<HashMap<String, Financial>>,
    pub comprehensive_income: Option<HashMap<String, Financial>>,
    pub cash_flow_statement: Option<HashMap<String, Financial>>,
    pub income_statement: Option<HashMap<String, Financial>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StockFinancial {
    // acceptance date time?
    pub cik: String,
    pub company_name: String,
    pub end_date: Option<String>,
    pub filing_date: Option<String>,
    pub financials: FinancialStatements,
    pub fiscal_period: String,
    pub fiscal_year: Option<String>,
    pub source_filing_file_url: Option<String>,
    pub source_filing_url: Option<String>,
    pub start_date: Option<String>,
    pub tickers: Option<Vec<String>>,
    pub timeframe: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct StockFinancialsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Vec<StockFinancial>
}
