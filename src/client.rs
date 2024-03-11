#![allow(unused)]

use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, Method, Request, StatusCode,
};
use serde::de::DeserializeOwned;

use crate::{
    app::{
        backup::*,
        logs::*,
        status::*,
        App, AppResponseAll, AppResponseUnique,
    },
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

    pub async fn get_app(&self, id: &str) -> Result<App, Error> {
        if id == "all" {
            return Err(Error::InvalidRequest(
                "Don't use all with that function. Use get_all_apps method instead.",
            ));
        }

        let body: AppResponseUnique =
            make_request(&self.config, Method::GET, &format!("app/{id}")).await?;

        Ok(body.apps)
    }

    pub async fn get_all_apps(&self) -> Result<Vec<App>, Error> {
        let body: AppResponseAll = make_request(&self.config, Method::GET, "app/all").await?;

        Ok(body.apps)
    }

    pub async fn get_all_apps_status(&self) -> Result<Vec<AppStatus>, Error> {
        let body: AppStatusResponseAll =
            make_request(&self.config, Method::GET, "app/all/status").await?;

        Ok(body.apps)
    }

    pub async fn get_app_status(&self, id: &str) -> Result<AppStatus, Error> {
        if id == "all" {
            return Err(Error::InvalidRequest(
                "Don't use all with that function. Use get_all_apps_status method instead.",
            ));
        }

        let body: AppStatusResponseUnique =
            make_request(&self.config, Method::GET, &format!("app/{id}/status")).await?;

        Ok(body.apps)
    }

    pub async fn get_app_logs(&self, id: &str) -> Result<AppLogs, Error> {
        if id == "all" {
            return Err(Error::InvalidRequest(
                "Don't use all with that function. Use get_all_apps_logs method instead.",
            ));
        }

        let body: AppLogsResponseUnique =
            make_request(&self.config, Method::GET, &format!("app/{id}/logs")).await?;

        Ok(body.apps)
    }

    pub async fn get_all_apps_logs(&self) -> Result<Vec<AppLogs>, Error> {
        let body: AppLogsResponseAll =
            make_request(&self.config, Method::GET, "app/all/logs").await?;

        Ok(body.apps)
    }

    pub async fn get_app_backup(&self, id: &str) -> Result<AppBackup, Error> {
        if id == "all" {
            return Err(Error::InvalidRequest(
                "Don't use all with that function. Use get_all_apps_backup method instead.",
            ));
        }

        let body: AppBackupResponseUnique =
            make_request(&self.config, Method::GET, &format!("app/{id}/backup")).await?;

        Ok(body.backups)
    }

    pub async fn get_all_apps_backup(&self) -> Result<Vec<AppBackup>, Error> {
        let body: AppBackupResponseAll =
            make_request(&self.config, Method::GET, "app/all/backup").await?;

        Ok(body.backups)
    }
}
