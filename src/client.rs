#![allow(unused)]

use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, Method, Request, StatusCode,
};
use serde::de::DeserializeOwned;

use crate::{
    config::Config,
    user::{Locale, LocaleResponse, User, UserResponse},
    util,
};
use tracing::{debug, trace};

use super::error::Error;

#[derive(Clone)]
pub struct Discloud {
    config: Config,
}

impl Discloud {
    pub fn new(token: &str) -> Self {
        trace!("Creating new client");
        Self {
            config: Config::new(
                token,
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
            ),
        }
    }

    pub async fn get_user_info(&self) -> Result<User, Error> {
        let url = util::default_url("user");

        debug!("Creating request client");
        let client = Client::builder()
            .user_agent(&self.config.user_agent)
            .build()?;

        debug!(url = url, "Making request");
        let response = client
            .get(url)
            .header("api-token", &self.config.token)
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
                401 => Error::InvalidToken,
                429 => Error::Ratelimited,
                404 => Error::NotFound,
                _ => Error::Unknown,
            });
        }

        let body = response.json::<UserResponse>().await?;
        debug!(response_body = format!("{body:?}"), "Request succeed");

        Ok(body.user)
    }

    pub async fn set_locale(&self, locale: Locale) -> Result<(), Error> {
        let url = util::default_url(&format!("locale/{locale}"));

        debug!("Creating request client");
        let client = Client::builder()
            .user_agent(&self.config.user_agent)
            .build()?;

        debug!(url = url, "Making request");
        let response = client
            .put(url)
            .header("api-token", &self.config.token)
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
                401 => Error::InvalidToken,
                429 => Error::Ratelimited,
                404 => Error::NotFound,
                _ => Error::Unknown,
            });
        }

        let body = response.json::<LocaleResponse>().await?;

        debug!(response_body = format!("{body:?}"), "Request succeed");

        Ok(())
    }
}
