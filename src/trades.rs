use crate::PolygonClient;
use reqwest::Method;
use crate::models::trades;

impl PolygonClient {
    /// Get trades for a ticker symbol in a given time range.
    /// Requires a "Stocks Developer" subscription
    pub async fn trades(&self, ticker: &str) -> trades::TradesParamsBuilder {
        let path = format!("/v3/trades/{ticker}");
        trades::TradesParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the most recent trade for a given stock.
    /// Requires a "Stocks Developer" subscription
    pub async fn last_trade(&self, ticker: &str) -> trades::LastTradeParamsBuilder {
        let path = format!("/v2/last/trade/{ticker}");
        trades::LastTradeParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the last trade tick for a cryptocurrency pair
    /// Requires a "Currencies Starter" subscription
    pub async fn last_crypto_trade(&self, from: &str, to: &str) -> trades::LastCryptoTradeParamsBuilder {
        let path = format!("/v2/last/trade/{from}/{to}");
        trades::LastCryptoTradeParamsBuilder(self.builder(Method::GET, &path))
    }
}
