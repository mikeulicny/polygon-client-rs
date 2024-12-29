use reqwest::Method;
use super::PolygonClient;
use crate::models::{conditions, tickers, splits, dividends, exchanges, contracts::{self, ListOptionsContractsParamsBuilder}};

impl PolygonClient {
    /// Query all ticker symbols which are supported by Polygon.io. This API currently includes Stocks/Equities, Indices, Forex, and Crypto.
    pub fn tickers(&self) -> tickers::TickersParamsBuilder {
        let path = "/v3/reference/tickers";
        tickers::TickersParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get a single ticker supported by Polygon.io. This response will have detailed information about the ticker and the company behind it.
    pub fn ticker_details(&self, ticker: &str) -> tickers::TickerDetailsParamsBuilder {
        let path = format!("/v3/reference/tickers/{ticker}");
        tickers::TickerDetailsParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the most recent news articles relating to a stock ticker symbol, including a summary of the article and a link to the original source.
    pub fn ticker_news(&self) -> tickers::TickerNewsParamsBuilder {
        let path = "/v2/reference/news";
        tickers::TickerNewsParamsBuilder(self.builder(Method::GET, &path))
    }

    /// List all ticker types that Polygon.io has.
    pub fn ticker_types(&self) -> tickers::TickerTypesParamsBuilder {
        let path = "/v3/reference/tickers/types";
        tickers::TickerTypesParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get a list of historical stock splits, including the ticker symbol, the execution date, and the factors of the split ratio.
    pub fn splits(&self) -> splits::SplitsParamsBuilder {
        let path = "/v3/reference/splits";
        splits::SplitsParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get a list of historical cash dividends, including the ticker symbol, declaration date, ex-dividend date, record date, pay date, frequency, and amount.
    pub fn dividends(&self) -> dividends::DividendsParamsBuilder {
        let path = "/v3/reference/dividends";
        dividends::DividendsParamsBuilder(self.builder(Method::GET, &path))
    }

    /// List all conditions that Polygon.io uses.
    pub fn conditions(&self) -> conditions::ConditionsParamsBuilder {
        conditions::ConditionsParamsBuilder(self.builder(Method::GET, "/v3/reference/conditions"))
    }

    /// List all exchanges that Polygon.io knows about.
    pub fn exchanges(&self) -> exchanges::ExchangesParamsBuilder {
        let path = "/v3/reference/exchanges";
        exchanges::ExchangesParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get an options contract
    pub fn get_options_contract(&self, ticker: &str) -> contracts::GetOptionsContractParamsBuilder {
        let path = format!("/v3/reference/options/contracts/{ticker}");
        contracts::GetOptionsContractParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Query for historical options contracts. This provides both active and expired options contracts.
    pub fn list_options_contract(&self) -> ListOptionsContractsParamsBuilder {
        let path = "/v3/reference/options/contracts";
        contracts::ListOptionsContractsParamsBuilder(self.builder(Method::GET, &path))
    }
}
