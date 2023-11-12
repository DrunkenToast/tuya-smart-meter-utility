use super::TuyaClient;
use crate::tuya::model::{
    device::{DeviceMonthlyStatistics, DevicesResponse},
    model::TuyaResult,
};
use reqwest::Method;
use serde_json::Value;

impl TuyaClient {
    pub async fn get_device_info(&mut self, device_id: &str) -> TuyaResult<Value> {
        self.make_request_business(
            Method::GET,
            format!("/v2.0/cloud/thing/{device_id}").as_str(),
            None,
        )
        .await
    }

    pub async fn get_device_statistics(
        &mut self,
        device_id: &str,
    ) -> TuyaResult<DeviceMonthlyStatistics> {
        let res = self
            .make_request_business(
                Method::GET,
                format!("/v1.0/devices/{device_id}/statistics/months").as_str(),
                Some(&[
                    ("code", "add_ele"),
                    ("end_month", "202311"),
                    ("start_month", "202311"),
                ]),
            )
            .await;
        res
    }

    pub async fn get_devices(&mut self) -> TuyaResult<DevicesResponse> {
        self.make_request_business(
            Method::GET,
            "/v2.0/cloud/thing/device",
            Some(&[("page_size", "20")]),
        )
        .await
    }
}
