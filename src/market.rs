use reqwest::Method;

use super::PolygonClient;
use crate::models::markets;

impl PolygonClient {
    /// Get upcoming market holidays and their open/close times.
    pub fn market_holidays(&self) -> markets::MarketHolidaysParamsBuilder {
        let path = "/v1/marketstatus/upcoming";
        markets::MarketHolidaysParamsBuilder(self.builder(Method::GET, &path))
    }

    /// Get the current trading status of the exchanges and overall financial markets.
    pub fn market_status(&self) -> markets::MarketStatusParamsBuilder {
        let path = "/v1/marketstatus/now";
        markets::MarketStatusParamsBuilder(self.builder(Method::GET, &path))
    }
}
