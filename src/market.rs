use reqwest::Method;

use super::PolygonClient;
use crate::models::markets;

impl PolygonClient {
    pub fn market_holidays(&self) -> markets::MarketHolidaysParamsBuilder {
        let path = "/v1/marketstatus/upcoming";
        markets::MarketHolidaysParamsBuilder(self.builder(Method::GET, &path))
    }

    pub fn market_status(&self) -> markets::MarketStatusParamsBuilder {
        let path = "/v1/marketstatus/now";
        markets::MarketStatusParamsBuilder(self.builder(Method::GET, &path))
    }
}
