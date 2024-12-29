use reqwest::Method;
use super::PolygonClient;
use crate::models::financials;

const FINANCIALS_PATH: &'static str = "/vX/reference/financials";

impl PolygonClient {
    /// Get historical financial data for a stock ticker
    pub fn stock_financials(&self) -> financials::StockFinancialsParamsBuilder {
        let path = FINANCIALS_PATH;
        return financials::StockFinancialsParamsBuilder(self.builder(Method::GET, &path));
    }
}
