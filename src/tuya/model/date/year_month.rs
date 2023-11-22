use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use time::{error, macros::format_description, Date, Month, OffsetDateTime};

use crate::util::pretty_string::PrettyString;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct YearMonth(Date);

impl YearMonth {
    pub fn new(year: i32, month: Month) -> Result<Self, error::Parse> {
        let d = Date::parse(
            (format!("{:04}{:02}", year, month) + "01").as_str(),
            format_description!("[year][month][day]"),
        )?;
        Ok(Self(d))
    }

    pub fn as_string(&self) -> String {
        format!("{:04}{:02}", self.0.year(), self.0.month() as u8)
    }

    pub fn year(&self) -> i32 {
        self.0.year()
    }

    pub fn month(&self) -> Month {
        self.0.month()
    }
}

impl PrettyString for YearMonth {
    fn as_pretty_string(&self) -> String {
        format!("{0} {1}", self.0.year(), self.0.month())
    }
}

impl Default for YearMonth {
    fn default() -> Self {
        let t = OffsetDateTime::now_utc().date();
        Self(t)
    }
}

impl Into<String> for &YearMonth {
    fn into(self) -> String {
        self.as_string()
    }
}

impl Into<Date> for &YearMonth {
    fn into(self) -> Date {
        self.0
    }
}

impl Display for YearMonth {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl TryFrom<&str> for YearMonth {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let str = format!("{}01", s);
        let f = format_description!("[year][month][day]");
        let t =
            Date::parse(&str, f).map_err(|_| "Failed to parse date, expects format 'yyyymm'")?;

        Ok(Self(t))
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
