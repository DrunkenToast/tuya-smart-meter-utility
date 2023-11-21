use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::{
    macros::format_description, util::days_in_year_month, Date, Month, OffsetDateTime, Time,
};

use crate::util::pretty_string::PrettyString;

#[derive(Debug, Hash, Eq, Clone)]
pub struct YearMonthDay {
    pub year: i32,
    pub month: Month,
    pub day: u8,
}

impl YearMonthDay {
    pub fn new(year: i32, month: Month, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn as_string(&self) -> String {
        format!("{:04}{:02}{:02}", self.year, self.month as u8, self.day)
    }

    pub fn first_day_current_month() -> Self {
        let t = OffsetDateTime::now_utc();
        Self {
            year: t.year(),
            month: t.month(),
            day: 1,
        }
    }

    pub fn last_day_current_month() -> Self {
        let t = OffsetDateTime::now_utc();
        Self {
            year: t.year(),
            month: t.month(),
            day: days_in_year_month(t.year(), t.month()),
        }
    }
}

impl PrettyString for YearMonthDay {
    fn as_pretty_string(&self) -> String {
        format!("{} {} {}", self.day, self.month, self.year)
    }
}

impl Default for YearMonthDay {
    fn default() -> Self {
        let t = OffsetDateTime::now_utc();
        Self {
            year: t.year(),
            month: t.month(),
            day: t.day(),
        }
    }
}

impl Into<String> for &YearMonthDay {
    fn into(self) -> String {
        self.as_string()
    }
}

impl Into<Date> for &YearMonthDay {
    fn into(self) -> Date {
        Date::parse(&self.as_string(), format_description!("[year][month][day]"))
            .expect("Self contained format")
    }
}

impl Display for YearMonthDay {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{0:04}{1:02}{2:02}",
            self.year, self.month as u8, self.day
        )
    }
}

impl PartialEq for YearMonthDay {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month
    }
}

impl Ord for YearMonthDay {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let d1: Date = self.into();
        let d2: Date = other.into();
        d1.cmp(&d2)
    }
}

impl PartialOrd for YearMonthDay {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<&str> for YearMonthDay {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let str = format!("{}", s);
        let f = format_description!("[year][month][day]");
        let t =
            Date::parse(&str, f).map_err(|_| "Failed to parse date, expects format 'yyyymmdd'")?;

        Ok(Self {
            year: t.year(),
            month: t.month(),
            day: t.day(),
        })
    }
}

impl TryFrom<String> for YearMonthDay {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

// Serde
impl Serialize for YearMonthDay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str: String = self.into();
        serializer.serialize_str(str.as_str())
    }
}

impl<'de> Deserialize<'de> for YearMonthDay {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        YearMonthDay::try_from(s).map_err(serde::de::Error::custom)
    }
}
