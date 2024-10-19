use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;

mod access_token;
mod client;
mod error;

#[allow(dead_code)]

trait IntoRequest {
    fn into_request(self, client: reqwest::Client) -> RequestBuilder;
}

pub type MiniGameResult<T> = Result<T, error::Error>;
