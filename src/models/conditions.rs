use serde::{Serialize, Deserialize};
use crate::BaseResponse;
use crate::models::types::{AssetClass, SIP, DataType, Sort, Order};
use crate::macros::{
    impl_limit_method,
    impl_order_method,
    impl_sort_method, impl_asset_class_method, impl_send_method
};

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateRule {
    pub updates_high_low: bool,
    pub updates_open_close: bool,
    pub updates_volume: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateRules {
    pub consolidated: UpdateRule,
    pub market_center: UpdateRule,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Condition {
    pub abbreviation: Option<String>,
    pub asset_class: String,
    pub data_types: Vec<String>,
    pub description: Option<String>,
    pub exchange: Option<i64>,
    pub id: i64,
    // legacy: bool,
    pub name: String,
    pub sip_mapping: String,
    #[serde(rename = "type")]
    pub data_type: Option<String>,
    pub update_rules: Option<UpdateRules>,
}

#[derive(Serialize)]
pub struct ConditionsParams {
    asset_class: Option<AssetClass>,
    data_type: Option<DataType>,
    id: Option<i64>,
    sip: Option<SIP>,
    order: Option<Order>,
    limit: u32, // 10 default
    sort: Option<Sort>
}

pub struct ConditionsParamsBuilder(pub(crate) reqwest::RequestBuilder);

impl ConditionsParamsBuilder {
    impl_asset_class_method!();

    pub fn data_type(mut self, data_type: DataType) -> Self {
        self.0 = self.0.query(&[("data_type", data_type)]);
        self
    }
    
    pub fn id(mut self, id: i64) -> Self {
        self.0 = self.0.query(&[("id", id)]);
        self
    }

    pub fn sip(mut self, sip: SIP) -> Self {
        self.0 = self.0.query(&[("sip", sip)]);
        self
    }

    impl_limit_method!();
    impl_order_method!();
    impl_sort_method!();
    impl_send_method!(ConditionsResponse);
}

#[derive(Deserialize)]
pub struct ConditionsResponse {
    #[serde(flatten)]
    pub base_response: BaseResponse,
    pub results: Vec<Condition>
}
