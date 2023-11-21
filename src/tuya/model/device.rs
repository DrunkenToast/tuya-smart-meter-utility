use crate::util::pretty_string::PrettyString;

use super::date::{year_month::YearMonth, year_month_day::YearMonthDay};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type DevicesResponse = Vec<DeviceResponse>;

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

impl DeviceResponse {
    pub fn get_name(&self) -> &str {
        return if self.custom_name.len() == 0 {
            &self.product_name
        } else {
            &self.custom_name
        };
    }

    pub fn print(&self, index: Option<usize>, to_print: impl PrettyString) {
        println!(
            "[{}] {}:\n{}",
            index.unwrap_or(0) + 1,
            self.get_name(),
            to_print.as_pretty_string()
        );
    }
}

// https://developer.tuya.com/en/docs/cloud/734e8088a6?id=Kcspwthd1f5tb
#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceMonthlyStatistics {
    pub months: BTreeMap<YearMonth, String>,
}

impl PrettyString for DeviceMonthlyStatistics {
    fn as_pretty_string(&self) -> String {
        let padding = self
            .months
            .iter()
            .map(|m| m.0.as_pretty_string().len())
            .max();

        return if let Some(padding) = padding {
            let s = self
                .months
                .iter()
                .map(|m| {
                    format!(
                        "* {: <width$} {} kWh",
                        m.0.as_pretty_string() + ":",
                        m.1,
                        width = padding + 1,
                    )
                })
                .collect::<Vec<String>>();
            s.join("\n")
        } else {
            "".into()
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceDailyStatistics {
    pub days: BTreeMap<YearMonthDay, String>,
}

impl PrettyString for DeviceDailyStatistics {
    fn as_pretty_string(&self) -> String {
        let padding = self.days.iter().map(|d| d.0.as_pretty_string().len()).max();

        return if let Some(padding) = padding {
            let s = self
                .days
                .iter()
                .map(|d| {
                    format!(
                        "* {: <width$} {} kWh",
                        d.0.as_pretty_string() + ":",
                        d.1,
                        width = padding + 1,
                    )
                })
                .collect::<Vec<String>>();
            s.join("\n")
        } else {
            "".into()
        };
    }
}
