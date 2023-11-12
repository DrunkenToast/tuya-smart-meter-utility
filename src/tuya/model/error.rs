use std::fmt::Display;

use url::ParseError;

#[derive(Debug)]
pub enum TuyaError {
    HostUrlParse(ParseError),
    RequestError {
        msg: String,
        code: i32,
        t: u64,
        tid: String,
    },
    RequestFailure(reqwest::Error),
}

impl From<reqwest::Error> for TuyaError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestFailure(value)
    }
}

impl Display for TuyaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = match self {
            Self::HostUrlParse(e) => format!("Host URL failed to parse: {}", e.to_string()),
            Self::RequestError { msg, code, t, tid } => {
                format!(
                    "Tuya request was unsuccesful:\n\
                \tmsg: {0}\n\
                \tcode: {1}\n\
                \tt: {2}\n\
                \ttid: {3}\n\
                See https://developer.tuya.com/en/docs/iot/error-code?id=K989ruxx88swc for more information",
                    msg, code, t, tid
                )
            }
            Self::RequestFailure(e) => e.to_string(),
        };
        writeln!(f, "{}", str)
    }
}
