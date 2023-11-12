use hmac::Hmac;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

pub struct Headers(pub HeaderMap);
pub type HmacSha256 = Hmac<Sha256>;
pub type TuyaResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct TuyaResponse<T> {
    pub success: bool,
    pub result: Option<T>,
    pub code: Option<i32>,
    pub t: u64,
    pub tid: String,
}
