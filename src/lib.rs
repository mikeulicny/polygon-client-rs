pub mod models {
    pub mod types;
    pub mod aggs;
    pub mod conditions;
    pub mod contracts;
    pub mod dividends;
    pub mod exchanges;
    pub mod financials;
    pub mod indicators;
    pub mod markets;
    pub mod quotes;
    pub mod snapshot;
    pub mod splits;
    pub mod summaries;
    pub mod tickers;
    pub mod trades;
}

pub mod aggs;
pub mod indicators;
pub mod market;
pub mod quotes;
pub mod reference;
pub mod snapshot;
pub mod trades;
pub mod vx;
pub mod error;

mod macros;

use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct BaseResponse {
    pub status: Option<String>,
    pub request_id: Option<String>,
    pub count: Option<i64>,
    pub message: Option<String>,
    pub error: Option<String>,
    // Only supported on structs with pagination
    pub next_url: Option<String>
}

pub struct PolygonClient {
    client: reqwest::Client,
    api_key: String,
}

impl PolygonClient {
    const BASE_URL: &'static str = "https://api.polygon.io";

    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key
        }
    }

    fn builder(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        self.client.request(method, format!("{}{}", Self::BASE_URL, path))
            .query(&[("apiKey", &self.api_key)])
    }
}

#[cfg(test)]
mod tests {
}
