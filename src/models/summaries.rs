use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Session {
    pub change: f64,
    pub change_percent: f64,
    pub close: f64,
    pub early_trading_change: Option<f64>,
    pub early_trading_change_percent: Option<f64>,
    pub high: f64,
    pub late_trading_change: Option<f64>,
    pub late_trading_change_percent: Option<f64>,
    pub low: f64,
    pub open: f64,
    pub previous_close: f64,
    pub price: Option<f64>,
    pub volume: Option<f64>,
}
