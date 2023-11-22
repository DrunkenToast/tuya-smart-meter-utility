use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::{macros::format_description, util::days_in_year_month, Date, Month, OffsetDateTime};

use crate::util::pretty_string::PrettyString;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct YearMonthDay(Date);

impl YearMonthDay {
    pub fn new(year: i32, month: Month, day: u8) -> Result<Self, &'static str> {
        let str = format!("{:04}{:02}{:02}", year, month, day);
        Self::try_from(str)
    }

    pub fn year(&self) -> i32 {
        self.0.year()
    }

    pub fn month(&self) -> Month {
        self.0.month()
    }

    pub fn day(&self) -> u8 {
        self.0.day()
    }

    pub fn as_string(&self) -> String {
        format!(
            "{:04}{:02}{:02}",
            self.year(),
            self.month() as u8,
            self.day()
        )
    }

    pub fn first_day_current_month() -> Self {
        let t = OffsetDateTime::now_utc();
        let t = t.replace_day(1).expect("First day of the month is valid");
        Self(t.date())
    }

    pub fn last_day_current_month() -> Self {
        let t = OffsetDateTime::now_utc();
        let t = t
            .replace_day(days_in_year_month(t.year(), t.month()))
            .expect("Last day of same month is valid");
        Self(t.date())
    }
}

impl PrettyString for YearMonthDay {
    fn as_pretty_string(&self) -> String {
        format!("{} {} {}", self.day(), self.month(), self.year())
    }
}

impl Default for YearMonthDay {
    fn default() -> Self {
        let t = OffsetDateTime::now_utc().date();
        Self(t)
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
        write!(f, "{}", self.as_string())
    }
}

impl TryFrom<&str> for YearMonthDay {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let str = format!("{}", s);
        let f = format_description!("[year][month][day]");
        let t =
            Date::parse(&str, f).map_err(|_| "Failed to parse date, expects format 'yyyymmdd'")?;

        Ok(Self(t))
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
