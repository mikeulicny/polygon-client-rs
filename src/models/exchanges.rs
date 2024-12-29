use serde::Deserialize;
use crate::BaseResponse;
use crate::macros::{impl_asset_class_method, impl_locale_method};
use crate::models::types::{AssetClass, MarketLocale};

pub struct ExchangesParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl ExchangesParamsBuilder {
    impl_asset_class_method!();
    impl_locale_method!();
}

#[derive(Deserialize, Debug, Clone)]
pub struct Exchange {
    pub acronym: Option<String>,
    pub asset_class: String,
    pub id: i64,
    pub locale: String,
    pub mic: Option<String>,
    pub name: String,
    pub operating_mic: Option<String>,
    pub participant_id: Option<String>,
    #[serde(rename = "type")]
    pub exchange_type: String,
    pub url: Option<String>,
}

#[derive(Deserialize)]
pub struct ExchangesResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Option<Exchange>
}
