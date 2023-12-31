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

    // TODO: Implement body content signing, currently not needed
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
            // Reqwest seems to internally sort the query params alphabetically.
            // This will invalidate the signed token, so we sort it manually.
            // (.build().url() does not work)
            let mut params = params.to_vec();
            params.sort_by(|a, b| a.0.cmp(b.0));
            url = Url::parse_with_params(format!("{0}{endpoint}", self.host).as_str(), params)
        } else {
            url = Url::parse(format!("{0}{endpoint}", self.host).as_str());
        }
        let url: Url = url.map_err(|e| TuyaError::HostUrlParse(e))?;

        let res: TuyaResponse<T> = self
            .request_client
            .request(method.clone(), url.clone())
            .headers(self.create_headers(&t, &method, &url, "", business))
            .send()
            .await?
            .json()
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

    // TODO: Implement specified header signing, currently not needed
    fn string_to_sign(&self, method: &str, content: &str, _headers: Headers, url: &str) -> String {
        let headers = "";
        let mut hasher = Sha256::new();
        hasher.update(content);
        let content_sha256 = hasher.finalize();

        let res = format!("{method}\n{:x}\n{headers}\n{url}", content_sha256);
        res
    }

    fn sign(&self, t: &u128, string_to_sign: &str, nonce: &u32, business: bool) -> String {
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
        body_content: &str,
        business: bool,
    ) -> HeaderMap {
        let nonce = rand::random::<u32>();

        let mut headers = HeaderMap::new();

        if business {
            if let Some(token) = &self.access_token {
                headers.insert("access_token", token.parse().unwrap());
            }
        }

        headers.insert("client_id", self.client_id.parse().unwrap());
        headers.insert("sign_method", "HMAC-SHA256".parse().unwrap());
        headers.insert("t", format!("{t}").parse().unwrap());
        headers.insert("nonce", nonce.into());

        let url = match url.query() {
            Some(query) => {
                format!("{0}?{1}", url.path(), query)
            }
            None => url.path().into(),
        };

        let signed = self.sign(
            &t,
            self.string_to_sign(
                method.as_str(),
                body_content,
                Headers(headers.clone()),
                url.as_str(),
            )
            .as_str(),
            &nonce,
            business,
        );

        headers.insert("sign", signed.parse().unwrap());

        headers
    }
}
