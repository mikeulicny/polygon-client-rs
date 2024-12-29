use reqwest::Method;
use crate::models::types::{Direction, MarketLocale, MarketType};
use crate::PolygonClient;
use crate::models::snapshot;

impl PolygonClient {
    /// Get the most up-to-date market data for all trade stock symbols.
    pub fn get_all_tickers_snapshot(&self, locale: MarketLocale, market_type: MarketType) -> snapshot::GetAllTickersSnapshotParamsBuilder {
        let path = format!("/v2/snapshot/locale/{locale}/markets/{market_type}/tickers");
        snapshot::GetAllTickersSnapshotParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the most up-to-date market data for a single traded stock ticker.
    pub fn get_ticker_snapshot(&self, locale: MarketLocale, market_type: MarketType, tickers: &str) -> snapshot::GetTickerSnapshotParamsBuilder {
        let path = format!("/v2/snapshot/locale/{locale}/markets/{market_type}/tickers/{tickers}");
        snapshot::GetTickerSnapshotParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the most up-to-date market data for current top 20 gainers or losers of the day in the
    /// stocks/equities markets.
    pub async fn get_gainers_losers_snapshot(&self, locale: MarketLocale, market_type: MarketType, direction: Direction) -> snapshot::GetGainersLosersSnapshotParamsBuilder {
        let path = format!("/v2/snapshot/locale/{locale}/markets/{market_type}/{direction}");
        snapshot::GetGainersLosersSnapshotParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the snapshot of an option contract for a stock equity.
    pub async fn get_option_contract_snapshot(&self, underlying_asset: &str, option_contract: &str) -> snapshot::GetOptionContractSnapshotParamsBuilder {
        let path = format!("/v3/snapshot/options/{underlying_asset}/{option_contract}");
        snapshot::GetOptionContractSnapshotParamsBuilder(self.builder(Method::GET, &path))
    }

    pub async fn get_indices_snapshot(&self) -> snapshot::GetIndicesSnapshotParamsBuilder {
        let path = "/v3/snapshot/indicies";
        snapshot::GetIndicesSnapshotParamsBuilder{
            request_builder: self.builder(Method::GET, &path),
            ticker_any_of: "".into()
        }
    }

    pub async fn list_universal_snapshots(&self) -> snapshot::ListUniversalSnapshotParamsBuilder {
        let path = "/v3/snapshot";
        snapshot::ListUniversalSnapshotParamsBuilder{
            request_builder: self.builder(Method::GET, &path),
            ticker_any_of: "".into()
        }
    }
}
