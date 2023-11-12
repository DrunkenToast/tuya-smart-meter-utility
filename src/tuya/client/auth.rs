use super::TuyaClient;
use crate::{
    tuya::model::{auth::TokenResponse, model::TuyaResult},
    util::time::get_time,
};
use reqwest::Method;

impl TuyaClient {
    pub async fn get_access_token(&mut self) -> TuyaResult<()> {
        let current_time = get_time();
        if let Some(_) = &self.access_token {
            if let Some(expire_time) = self.expiration_time {
                if expire_time > current_time {
                    // Our access_token is still valid
                    return Ok(());
                }
            }
        }

        let res: TokenResponse;
        if let Some(token) = &self.refresh_token {
            res = self
                .make_request_token(
                    Method::GET,
                    format!("/v1.0/token/{0}", token).as_str(),
                    Some(&[("grant_type", "1")]),
                )
                .await?;
        } else {
            res = self
                .make_request_token(Method::GET, "/v1.0/token", Some(&[("grant_type", "1")]))
                .await?;
        }

        self.access_token = Some(res.access_token);
        self.refresh_token = Some(res.refresh_token);
        self.expiration_time = Some(current_time + res.expire_time);

        Ok(())
    }
}
