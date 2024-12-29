use reqwest::Method;
use super::PolygonClient;
use crate::models::financials;

const FINANCIALS_PATH: &'static str = "/vX/reference/financials";

impl PolygonClient {
    pub fn stock_financials(&self) -> financials::StockFinancialsParamsBuilder {
        let path = FINANCIALS_PATH;
        return financials::StockFinancialsParamsBuilder(self.builder(Method::GET, &path));
    }
}
