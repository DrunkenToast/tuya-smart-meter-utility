use super::TuyaClient;
use crate::{
    tuya::model::{
        error::TuyaError,
        model::{Headers, HmacSha256, TuyaResponse, TuyaResult},
    },
    util::time::get_time,
};
use hmac::Mac;
use reqwest::header::HeaderMap;
use serde::de;
use sha2::{Digest, Sha256};
use url::Url;

impl TuyaClient {
    pub async fn make_request_business<T: de::DeserializeOwned>(
        &mut self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> TuyaResult<T> {
        self.get_access_token().await?;
        self.make_request(method, endpoint, params, true).await
    }

    pub async fn make_request_token<T: de::DeserializeOwned>(
        &mut self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> TuyaResult<T> {
        self.make_request(method, endpoint, params, false).await
    }

    async fn make_request<T: de::DeserializeOwned>(
        &mut self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
        business: bool,
    ) -> TuyaResult<T> {
        let t = get_time();

        let url;
        if let Some(params) = params {
            url = Url::parse_with_params(format!("{0}{endpoint}", self.host).as_str(), params)
        } else {
            url = Url::parse(format!("{0}{endpoint}", self.host).as_str());
        }
        let url: Url = url.map_err(|e| TuyaError::HostUrlParse(e))?;

        let res = reqwest::Client::new()
            .request(method.clone(), url.clone())
            .headers(self.create_headers(&t, &method, &url, business))
            .send()
            .await?
            .json::<TuyaResponse<T>>()
            .await?;

        if res.success {
            Ok(res.result.expect("Should be succesful"))
        } else {
            Err(TuyaError::RequestError {
                msg: res.msg.expect("contains message on failure"),
                code: res.code.expect("contains code on failure"),
                t: res.t,
                tid: res.tid,
            })
        }
    }

    fn string_to_sign(&self, method: &str, content: &str, _headers: Headers, url: &str) -> String {
        // let headers: String = headers.into();
        let headers = "";
        let mut hasher = Sha256::new();
        hasher.update(content);
        let content_sha256 = hasher.finalize();

        let res = format!("{method}\n{:x}\n{headers}\n{url}", content_sha256);
        res
    }

    fn sign(&self, t: &u128, string_to_sign: &str, nonce: &str, business: bool) -> String {
        let str: String;
        if business {
            str = format!(
                "{0}{1}{t}{nonce}{string_to_sign}",
                self.client_id,
                self.access_token
                    .clone()
                    .expect("Access token should get retrieved first!")
            );
        } else {
            str = format!("{0}{t}{nonce}{string_to_sign}", self.client_id);
        }

        let mut mac = HmacSha256::new_from_slice(self.client_secret.as_bytes())
            .expect("HMAC can take key of any size");

        mac.update(str.as_bytes());

        let res = mac.finalize().into_bytes();
        let res = format!("{:X}", res);

        res
    }

    fn create_headers(
        &self,
        t: &u128,
        method: &reqwest::Method,
        url: &Url,
        business: bool,
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();

        if business {
            if let Some(token) = &self.access_token {
                headers.insert("access_token", token.parse().unwrap());
            }
        }

        headers.insert("client_id", self.client_id.parse().unwrap());
        headers.insert("sign_method", "HMAC-SHA256".parse().unwrap());
        headers.insert("t", format!("{t}").parse().unwrap());
        headers.insert("nonce", "".parse().unwrap());

        let url = match url.query() {
            Some(query) => {
                format!("{0}?{1}", url.path(), query)
            }
            None => url.path().into(),
        };

        let signed = self.sign(
            &t,
            self.string_to_sign(method.as_str(), "", Headers(headers.clone()), url.as_str())
                .as_str(),
            "",
            business,
        );

        headers.insert("sign", signed.parse().unwrap());

        headers
    }
}
