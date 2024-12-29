use reqwest::Method;
use super::PolygonClient;
use crate::models::{conditions, tickers, splits, dividends, exchanges, contracts::{self, ListOptionsContractsParamsBuilder}};

impl PolygonClient {
    pub fn tickers(&self) -> tickers::TickersParamsBuilder {
        let path = "/v3/reference/tickers";
        tickers::TickersParamsBuilder(self.builder(Method::GET, &path))
    }

    // Ticker Details
    pub fn ticker_details(&self, ticker: &str) -> tickers::TickerDetailsParamsBuilder {
        let path = format!("/v3/reference/tickers/{ticker}");
        tickers::TickerDetailsParamsBuilder(self.builder(Method::GET, &path))
    }

    // Ticker News
    pub fn ticker_news(&self) -> tickers::TickerNewsParamsBuilder {
        let path = "/v2/reference/news";
        tickers::TickerNewsParamsBuilder(self.builder(Method::GET, &path))
    }

    // Ticker Types
    pub fn ticker_types(&self) -> tickers::TickerTypesParamsBuilder {
        let path = "/v3/reference/tickers/types";
        tickers::TickerTypesParamsBuilder(self.builder(Method::GET, &path))
    }

    pub fn splits(&self) -> splits::SplitsParamsBuilder {
        let path = "/v3/reference/splits";
        splits::SplitsParamsBuilder(self.builder(Method::GET, &path))
    }

    pub fn dividends(&self) -> dividends::DividendsParamsBuilder {
        let path = "/v3/reference/dividends";
        dividends::DividendsParamsBuilder(self.builder(Method::GET, &path))
    }

    pub fn conditions(&self) -> conditions::ConditionsParamsBuilder {
        conditions::ConditionsParamsBuilder(self.builder(Method::GET, "/v3/reference/conditions"))
    }

    pub fn exchanges(&self) -> exchanges::ExchangesParamsBuilder {
        let path = "/v3/reference/exchanges";
        exchanges::ExchangesParamsBuilder(self.builder(Method::GET, &path))
    }

    pub fn get_options_contract(&self, ticker: &str) -> contracts::GetOptionsContractParamsBuilder {
        let path = format!("/v3/reference/options/contracts/{ticker}");
        contracts::GetOptionsContractParamsBuilder(self.builder(Method::GET, &path))
    }

    pub fn list_options_contract(&self) -> ListOptionsContractsParamsBuilder {
        let path = "/v3/reference/options/contracts";
        contracts::ListOptionsContractsParamsBuilder(self.builder(Method::GET, &path))
    }
}
