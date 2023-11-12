use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// https://developer.tuya.com/en/docs/cloud/734e8088a6?id=Kcspwthd1f5tb
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceResponse {
    #[serde(rename = "id")]
    pub device_id: String,
    #[serde(rename = "productName")]
    pub product_name: String,
    #[serde(rename = "customName")]
    pub custom_name: String,
    #[serde(rename = "isOnline")]
    pub is_online: bool,
}
pub type DevicesResponse = Vec<DeviceResponse>;

// https://developer.tuya.com/en/docs/cloud/734e8088a6?id=Kcspwthd1f5tb
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceMonthlyStatistics {
    // TODO: Use YearMonth type here
    pub months: HashMap<String, String>,
}
