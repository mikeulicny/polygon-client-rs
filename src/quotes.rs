use reqwest::Method;
use crate::PolygonClient;
use crate::models::quotes::{self, GetRealTimeCurrencyConversionParamsBuilder};

impl PolygonClient {
    /// Get NBBO quotes for a ticker symbol in a given time range.
    pub fn list_quotes(&self, ticker: &str) -> quotes::ListQuotesParamsBuilder {
        let path = format!("/v3/quotes/{ticker}");
        quotes::ListQuotesParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the most recent NBBO (Quote) tick for a given stock.
    pub fn get_last_quote(&self, ticker: &str) -> quotes::GetLastQuoteParamsBuilder {
        let path = format!("/v2/last/nbbo/{ticker}");
        quotes::GetLastQuoteParamsBuilder(self.builder(Method::GET, &path))
    }

    pub fn get_last_forex_quote(&self, from: &str, to: &str) -> quotes::GetLastForexQuoteParamsBuilder {
        let path = format!("/v1/last_quote/currencies/{from}/{to}");
        quotes::GetLastForexQuoteParamsBuilder(self.builder(Method::GET, &path))
    }

    pub fn get_real_time_currency_conversion(&self, from: &str, to: &str) -> GetRealTimeCurrencyConversionParamsBuilder {
        let path = format!("/v1/conversion/{from}/{to}");
        quotes::GetRealTimeCurrencyConversionParamsBuilder(self.builder(Method::GET, &path))
    }
}
