#![allow(unused)]

use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, Method, Request, StatusCode,
};
use serde::de::DeserializeOwned;

use crate::{
    config::Config,
    user::{Locale, LocaleResponse, User, UserResponse},
    util::{self, make_request},
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
        let body: UserResponse = make_request(&self.config, Method::GET, "user").await?;

        Ok(body.user)
    }

    pub async fn set_locale(&self, locale: Locale) -> Result<(), Error> {
        let _: LocaleResponse =
            make_request(&self.config, Method::PUT, &format!("locale/{locale}")).await?;

        Ok(())
    }
}
