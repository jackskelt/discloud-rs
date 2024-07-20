use std::fmt::Debug;

use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, trace};

use crate::{config::Config, Error};

pub fn default_url(path: &str) -> String {
    format!("https://api.discloud.app/v2/{path}")
}

pub async fn make_request<T: DeserializeOwned + Debug>(
    config: &Config,
    method: Method,
    path: &str,
) -> Result<T, Error> {
    let url = default_url(path);

    debug!("Creating request client");
    let client = Client::builder().user_agent(&config.user_agent).build()?;

    debug!(url = url, "Making request");
    let response = client
        .request(method, url)
        .header("api-token", &config.token)
        .send()
        .await?;

    let response_status = response.status();

    if !response_status.is_success() {
        debug!(
            status = response_status.as_u16(),
            response_body = format!("{:?}", response.text().await),
            "Request failed"
        );
        if response_status.is_server_error() {
            return Err(Error::ServerError);
        }

        return Err(match response_status.as_u16() {
            403 => Error::Forbidden,
            401 => Error::InvalidToken,
            429 => Error::Ratelimited,
            404 => Error::NotFound,
            _ => Error::Unknown,
        });
    }

    let body = response.json::<T>().await?;
    debug!(response_body = format!("{body:?}"), "Request succeed");
    Ok(body)
}

pub async fn make_request_with_body<T: DeserializeOwned + Debug, B: Serialize + Debug>(
    config: &Config,
    method: Method,
    path: &str,
    body: B,
) -> Result<T, Error> {
    let url = default_url(path);

    debug!("Creating request client");
    let client = Client::builder().user_agent(&config.user_agent).build()?;

    debug!(url = url, "Making request");
    let response = client
        .request(method, url)
        .json(&body)
        .header("api-token", &config.token)
        .send()
        .await?;

    let response_status = response.status();

    if !response_status.is_success() {
        trace!(?body);
        debug!(
            status = response_status.as_u16(),
            response_body = format!("{:?}", response.text().await),
            "Request failed"
        );
        if response_status.is_server_error() {
            return Err(Error::ServerError);
        }

        return Err(match response_status.as_u16() {
            401 => Error::InvalidToken,
            403 => Error::Forbidden,
            429 => Error::Ratelimited,
            404 => Error::NotFound,
            _ => Error::Unknown,
        });
    }

    let body = response.json::<T>().await?;
    debug!(response_body = format!("{body:?}"), "Request succeed");
    Ok(body)
}
