pub mod auth;
pub mod device;
pub mod request;

pub struct TuyaClient {
    host: String,
    client_id: String,
    client_secret: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expiration_time: Option<u128>,
}

impl TuyaClient {
    pub fn new(host: &str, client_id: &str, client_secret: &str) -> Self {
        Self {
            host: host.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            access_token: None,
            refresh_token: None,
            expiration_time: None,
        }
    }
}
