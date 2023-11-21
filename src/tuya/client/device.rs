use super::TuyaClient;
use crate::tuya::model::{
    date::{year_month::YearMonth, year_month_day::YearMonthDay},
    device::{DeviceDailyStatistics, DeviceMonthlyStatistics, DeviceResponse, DevicesResponse},
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

    pub async fn get_device_properties(&mut self, device_id: &str) -> TuyaResult<Value> {
        self.make_request_business(
            Method::GET,
            format!("/v2.0/cloud/thing/{device_id}/shadow/properties").as_str(),
            None,
        )
        .await
    }

    pub async fn get_monthly_device_statistics(
        &mut self,
        device_id: &str,
        start: &YearMonth,
        end: &YearMonth,
    ) -> TuyaResult<DeviceMonthlyStatistics> {
        let start: String = start.as_string();
        let end: String = end.as_string();

        let res = self
            .make_request_business(
                Method::GET,
                format!("/v1.0/devices/{device_id}/statistics/months").as_str(),
                Some(&[
                    ("code", "add_ele"),
                    ("end_month", &end),
                    ("start_month", &start),
                ]),
            )
            .await;
        res
    }

    pub async fn get_daily_device_statistics(
        &mut self,
        device_id: &str,
        start: &YearMonthDay,
        end: &YearMonthDay,
    ) -> TuyaResult<DeviceDailyStatistics> {
        let start: String = start.as_string();
        let end: String = end.as_string();

        let res = self
            .make_request_business(
                Method::GET,
                format!("/v1.0/devices/{device_id}/statistics/days").as_str(),
                Some(&[
                    ("start_day", &start),
                    ("end_day", &end),
                    ("code", "add_ele"),
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
