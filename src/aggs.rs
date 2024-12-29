use super::PolygonClient;
use crate::models::{aggs, types::{MarketLocale, MarketType, Timespan}};
use reqwest::Method;

impl PolygonClient {
    /// Get aggregate bars for a stock over a given date range in custom time window sizes.
    /// For example, if timespan = 'minute' and multiplier = 5 then 5-minute bars will be returned.
    pub fn list_aggs(&self, ticker: String, multiplier: u32, timespan: Timespan, from: String, to: String) -> aggs::ListAggsParamsBuilder {
        let path = format!("/v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to}");
        aggs::ListAggsParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the daily open, high, low, and close (OHLC) for the entire stocks/equities markets.
    pub fn grouped_daily_aggs(&self, locale: MarketLocale, market_type: MarketType, date: &str) -> aggs::GroupedDailyAggsParamsBuilder {
        let path = format!("/v2/aggs/grouped/locale/{locale}/market/{market_type}/{date}");
        aggs::GroupedDailyAggsParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the open, close, and afterhours prices of a stock symbol on a certain date.
    pub fn daily_open_close_aggs(&self, ticker: &str, date: &str) -> aggs::DailyOpenCloseAggsParamsBuilder {
        let path = format!("/v1/open-close/{ticker}/{date}");
        aggs::DailyOpenCloseAggsParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the previous day's open, high, low, and close (OHLC) for the specified stock ticker.
    pub fn previous_close_aggs(&self, ticker: &str) -> aggs::PreviousCloseAggsParamsBuilder {
        let path = format!("/v2/aggs/ticker/{ticker}/prev");
        aggs::PreviousCloseAggsParamsBuilder(self.builder(Method::GET, &path))
    }
}
