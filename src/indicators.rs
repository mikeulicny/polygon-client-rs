use super::PolygonClient;
use crate::models::indicators;
use reqwest::Method;

impl PolygonClient {
    /// Get the simple moving average (SMA) for a ticker symbol over a given time range.
    #[allow(non_snake_case)]
    pub fn get_SMA(&self, ticker: &str) -> indicators::GetSMAParamsBuilder {
        let path = format!("/v1/indicators/sma/{ticker}");
        indicators::GetSMAParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the exponential moving average (EMA) for a ticker symbol over a given time range.
    #[allow(non_snake_case)]
    pub fn get_EMA(&self, ticker: &str) -> indicators::GetEMAParamsBuilder {
        let path = format!("/v1/indicators/ema/{ticker}");
        indicators::GetEMAParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get moving average convergence/divergence (MACD) data for a ticker symbol over a given time range.
    #[allow(non_snake_case)]
    pub fn get_MACD(&self, ticker: &str) -> indicators::GetMACDParamsBuilder {
        let path = format!("/v1/indicators/macd/{ticker}");
        indicators::GetMACDParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the relative strength index (RSI) for a ticker symbol over a given time range.
    #[allow(non_snake_case)]
    pub fn get_RSI(&self, ticker: &str) -> indicators::GetRSIParamsBuilder {
        let path = format!("/v1/indicators/rsi/{ticker}");
        indicators::GetRSIParamsBuilder(self.builder(Method::GET, &path))
    }
}
