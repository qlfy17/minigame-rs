use std::time::Duration;

use reqwest::{header, RequestBuilder};
use serde::de::DeserializeOwned;

use crate::{IntoRequest, MiniGameResult};

static BASE_URL: &str = "https://api.weixin.qq.com/cgi-bin";

#[allow(dead_code)]
#[derive(Debug)]
pub struct MiniGameClient {
    // 微信小程序平台API基础地址 https://api.weixin.qq.com/cgi-bin
    pub base_url: String,
    pub client: reqwest::Client,
}

impl Default for MiniGameClient {
    fn default() -> Self {
        Self {
            base_url: BASE_URL.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[allow(dead_code)]
impl MiniGameClient {
    pub fn new(client: Option<reqwest::Client>) -> Self {
        match client {
            Some(client) => Self {
                base_url: BASE_URL.into(),
                client,
            },
            None => Self::default(),
        }
    }

    pub fn new_request(&self, req: impl IntoRequest) -> RequestBuilder {
        let req = req.into_request(self.client.clone());
        // req.bearer_auth(&self.token);
        req.header(header::ACCEPT, "application/vnd.github.v3+json")
            .header(header::USER_AGENT, "crates.io (https://crates.io)")
            .timeout(Duration::from_secs(10))
    }

    pub async fn send<T>(&self, req: RequestBuilder) -> MiniGameResult<T>
    where
        T: DeserializeOwned,
    {
        req.send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await
            .map_err(Into::into)
    }
}
