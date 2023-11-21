use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::{macros::format_description, Date, Month, OffsetDateTime};

use crate::util::pretty_string::PrettyString;

#[derive(Debug, Hash, Eq, Clone)]
pub struct YearMonth {
    pub year: i32,
    pub month: Month,
}

impl YearMonth {
    pub fn new(year: i32, month: Month) -> Self {
        Self { year, month }
    }

    pub fn as_string(&self) -> String {
        format!("{:04}{:02}", self.year, self.month as u8)
    }
}

impl PrettyString for YearMonth {
    fn as_pretty_string(&self) -> String {
        format!("{0} {1}", self.year, self.month)
    }
}

impl Default for YearMonth {
    fn default() -> Self {
        let t = OffsetDateTime::now_utc();
        Self {
            year: t.year(),
            month: t.month(),
        }
    }
}

impl Into<String> for &YearMonth {
    fn into(self) -> String {
        self.as_string()
    }
}

impl Into<Date> for &YearMonth {
    fn into(self) -> Date {
        Date::parse(
            &(self.as_string() + "01"),
            format_description!("[year][month][day]"),
        )
        .expect("Self contained format")
    }
}

impl Display for YearMonth {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{0}{1}", self.year, self.month as u8)
    }
}

impl PartialEq for YearMonth {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month
    }
}

impl Ord for YearMonth {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let d1: Date = self.into();
        let d2: Date = other.into();
        d1.cmp(&d2)
    }
}

impl PartialOrd for YearMonth {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<&str> for YearMonth {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let str = format!("{}01", s);
        let f = format_description!("[year][month][day]");
        let t =
            Date::parse(&str, f).map_err(|_| "Failed to parse date, expects format 'yyyymm'")?;

        Ok(Self {
            year: t.year(),
            month: t.month(),
        })
    }
}

impl TryFrom<String> for YearMonth {
    type Error = &'static str;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

// Serde
impl Serialize for YearMonth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let str: String = self.into();
        serializer.serialize_str(str.as_str())
    }
}

impl<'de> Deserialize<'de> for YearMonth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;

        YearMonth::try_from(s).map_err(serde::de::Error::custom)
    }
}
