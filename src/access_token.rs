use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{client::MiniGameClient, IntoRequest, MiniGameResult};

#[derive(Debug, Serialize)]
pub struct AccessTokenRequest {
    // 填写 client_credential
    pub grant_type: String,
    // 账号唯一凭证，即 AppID，可在「微信公众平台 - 设置 - 开发设置」页中获得。（需要已经成为开发者，且账号没有异常状态）
    pub appid: String,
    // 账号唯一凭证密钥，即 AppSecret，获取方式同 appid
    pub secret: String,
    // 默认使用 false。1. force_refresh = false 时为普通调用模式，access_token 有效期内重复调用该接口不会更新 access_token；2. 当force_refresh = true 时为强制刷新模式，会导致上次获取的 access_token 失效，并返回新的 access_token
    pub force_refresh: bool,
}

#[allow(dead_code)]
impl AccessTokenRequest {
    pub fn new(appid: &str, secret: &str, force_refresh: Option<bool>) -> Self {
        AccessTokenRequestBuilder::new()
            .appid(appid)
            .secret(secret)
            .force_refresh(force_refresh)
            .build()
    }
}

impl IntoRequest for AccessTokenRequest {
    fn into_request(self, client: Client) -> RequestBuilder {
        client
            .post("https://api.weixin.qq.com/cgi-bin/stable_token")
            .json(&self)
    }
}

#[allow(dead_code)]
impl MiniGameClient {
    pub async fn get_stable_access_token(
        &self,
        req: AccessTokenRequest,
    ) -> MiniGameResult<AccessTokenResponse> {
        let req = self.new_request(req);
        Ok(self.send(req).await?)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct AccessTokenRequestBuilder {
    // 填写 client_credential
    pub grant_type: String,
    // 账号唯一凭证，即 AppID，可在「微信公众平台 - 设置 - 开发设置」页中获得。（需要已经成为开发者，且账号没有异常状态）
    pub appid: String,
    // 账号唯一凭证密钥，即 AppSecret，获取方式同 appid
    pub secret: String,
    // 默认使用 false。1. force_refresh = false 时为普通调用模式，access_token 有效期内重复调用该接口不会更新 access_token；2. 当force_refresh = true 时为强制刷新模式，会导致上次获取的 access_token 失效，并返回新的 access_token
    pub force_refresh: Option<bool>,
}

impl Default for AccessTokenRequestBuilder {
    fn default() -> Self {
        Self {
            grant_type: String::from("client_credential"),
            appid: Default::default(),
            secret: Default::default(),
            force_refresh: Some(false),
        }
    }
}

#[allow(dead_code)]
impl AccessTokenRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn appid(mut self, appid: &str) -> Self {
        self.appid = appid.into();
        self
    }

    /// 账号唯一凭证密钥，即 AppSecret，获取方式同 appid
    pub fn secret(mut self, secret: &str) -> Self {
        self.secret = secret.into();
        self
    }

    /// 默认使用 false。1. force_refresh = false 时为普通调用模式，access_token 有效期内重复调用该接口不会更新 access_token；2. 当force_refresh = true 时为强制刷新模式，会导致上次获取的 access_token 失效，并返回新的 access_token
    pub fn force_refresh(mut self, force_refresh: Option<bool>) -> Self {
        self.force_refresh = force_refresh;
        self
    }

    pub fn build(self) -> AccessTokenRequest {
        AccessTokenRequest {
            grant_type: self.grant_type,
            appid: self.appid,
            secret: self.secret,
            force_refresh: self.force_refresh.is_some(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct AccessTokenResponse {
    // 获取到的凭证
    pub access_token: String,
    // 凭证有效时间，单位：秒。目前是7200秒之内的值。
    pub expires_in: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result};
    use dotenv::var;

    #[tokio::test]
    async fn get_stable_access_token_should_work() -> Result<()> {
        dotenv::dotenv().ok();
        let appid = String::from(var("APPID").unwrap());
        let secret = String::from(var("APP_SECRET").unwrap());
        let client = MiniGameClient::new(None);
        let req = AccessTokenRequestBuilder::new()
            .appid(&appid)
            .secret(&secret)
            .build();
        let res = client.get_stable_access_token(req).await?;
        println!("{:#?}", res);
        Ok(())
    }
}
