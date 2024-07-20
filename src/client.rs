#![allow(unused)]

use reqwest::{
    header::{HeaderMap, HeaderName},
    Client, Method, Request, StatusCode,
};
use serde::de::DeserializeOwned;

use crate::{
    app::{backup::*, logs::*, manage::*, status::*, App, AppResponseAll, AppResponseUnique},
    config::Config,
    team_manager::{
        APITeamMember, AddTeamMemberResponse, GetTeamManagerResponse, TeamMember, TeamMemberBody,
        TeamPerms,
    },
    user::{Locale, LocaleResponse, User, UserResponse},
    util::{make_request, make_request_with_body, DiscloudDefaultResponse},
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

    pub async fn remove_app_mod(&self, app_id: &str, mod_id: &str) -> Result<(), Error> {
        let _res: DiscloudDefaultResponse = make_request(
            &self.config,
            Method::DELETE,
            &format!("app/{app_id}/team/{mod_id}"),
        )
        .await?;

        Ok(())
    }

    pub async fn edit_app_mod(
        &self,
        app_id: &str,
        mod_id: &str,
        perms: Vec<TeamPerms>,
    ) -> Result<TeamMember, Error> {
        let res: AddTeamMemberResponse = make_request_with_body(
            &self.config,
            Method::PUT,
            &format!("app/{app_id}/team"),
            TeamMemberBody {
                mod_id: mod_id.to_string(),
                perms,
            },
        )
        .await?;
        Ok(res.app)
    }

    pub async fn add_app_mod(
        &self,
        app_id: &str,
        mod_id: &str,
        perms: Vec<TeamPerms>,
    ) -> Result<TeamMember, Error> {
        let res: AddTeamMemberResponse = make_request_with_body(
            &self.config,
            Method::POST,
            &format!("app/{app_id}/team"),
            TeamMemberBody {
                mod_id: mod_id.to_string(),
                perms,
            },
        )
        .await?;

        Ok(res.app)
    }

    pub async fn get_app_team(&self, id: &str) -> Result<Vec<TeamMember>, Error> {
        let res: GetTeamManagerResponse =
            make_request(&self.config, Method::GET, &format!("app/{id}/team")).await?;

        let team_members = res
            .team
            .iter()
            .map(|v| TeamMember {
                mod_id: v.id.clone(),
                app_id: id.to_owned(),
                perms: v.perms.clone(),
            })
            .collect::<Vec<_>>();

        Ok(team_members)
    }

    pub async fn get_user_info(&self) -> Result<User, Error> {
        let res: UserResponse = make_request(&self.config, Method::GET, "user").await?;

        Ok(res.user)
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

        let res: AppResponseUnique =
            make_request(&self.config, Method::GET, &format!("app/{id}")).await?;

        Ok(res.apps)
    }

    pub async fn get_all_apps(&self) -> Result<Vec<App>, Error> {
        let res: AppResponseAll = make_request(&self.config, Method::GET, "app/all").await?;

        Ok(res.apps)
    }

    pub async fn get_all_apps_status(&self) -> Result<Vec<AppStatus>, Error> {
        let res: AppStatusResponseAll =
            make_request(&self.config, Method::GET, "app/all/status").await?;

        Ok(res.apps)
    }

    pub async fn get_app_status(&self, id: &str) -> Result<AppStatus, Error> {
        if id == "all" {
            return Err(Error::InvalidRequest(
                "Don't use all with that function. Use get_all_apps_status method instead.",
            ));
        }

        let res: AppStatusResponseUnique =
            make_request(&self.config, Method::GET, &format!("app/{id}/status")).await?;

        Ok(res.apps)
    }

    pub async fn get_app_logs(&self, id: &str) -> Result<AppLogs, Error> {
        if id == "all" {
            return Err(Error::InvalidRequest(
                "Don't use all with that function. Use get_all_apps_logs method instead.",
            ));
        }

        let res: AppLogsResponseUnique =
            make_request(&self.config, Method::GET, &format!("app/{id}/logs")).await?;

        Ok(res.apps)
    }

    pub async fn get_all_apps_logs(&self) -> Result<Vec<AppLogs>, Error> {
        let res: AppLogsResponseAll =
            make_request(&self.config, Method::GET, "app/all/logs").await?;

        Ok(res.apps)
    }

    pub async fn get_app_backup(&self, id: &str) -> Result<AppBackup, Error> {
        if id == "all" {
            return Err(Error::InvalidRequest(
                "Don't use all with that function. Use get_all_apps_backup method instead.",
            ));
        }

        let res: AppBackupResponseUnique =
            make_request(&self.config, Method::GET, &format!("app/{id}/backup")).await?;

        Ok(res.backups)
    }

    pub async fn get_all_apps_backup(&self) -> Result<Vec<AppBackup>, Error> {
        let res: AppBackupResponseAll =
            make_request(&self.config, Method::GET, "app/all/backup").await?;

        Ok(res.backups)
    }

    pub async fn start_app(&self, id: &str) -> Result<AppStartStatus, AppStartError> {
        if id == "all" {
            return Err(AppStartError::Other(Error::InvalidRequest(
                "Don't use all with that function. Use start_all_apps method instead.",
            )));
        }

        let res: AppStartResponseUnique =
            make_request(&self.config, Method::PUT, &format!("app/{id}/start")).await?;

        if res.status == "error" {
            return Err(AppStartError::AlreadyOnline);
        }

        res.app_status.ok_or(AppStartError::Other(Error::Unknown))
    }

    pub async fn start_all_apps(&self) -> Result<AppStartAll, Error> {
        let res: AppStartResponseAll =
            make_request(&self.config, Method::PUT, "app/all/start").await?;

        Ok(res.apps)
    }

    pub async fn stop_app(&self, id: &str) -> Result<(), AppStopError> {
        if id == "all" {
            return Err(AppStopError::Other(Error::InvalidRequest(
                "Don't use all with that function. Use stop_all_apps method instead.",
            )));
        }

        let res: AppStartResponseUnique =
            make_request(&self.config, Method::PUT, &format!("app/{id}/stop")).await?;

        if res.status == "error" {
            return Err(AppStopError::AlreadyOffline);
        }

        Ok(())
    }

    pub async fn stop_all_apps(&self) -> Result<AppStopAll, Error> {
        let res: AppStopResponseAll =
            make_request(&self.config, Method::PUT, "app/all/stop").await?;

        Ok(res.apps)
    }

    pub async fn restart_app(&self, id: &str) -> Result<(), Error> {
        if id == "all" {
            return Err(Error::InvalidRequest(
                "Don't use all with that function. Use restart_all_apps method instead.",
            ));
        }

        let res: AppRestartResponseUnique =
            make_request(&self.config, Method::PUT, &format!("app/{id}/restart")).await?;

        if res.status == "error" {
            return Err(Error::Unknown);
        }

        Ok(())
    }

    pub async fn restart_all_apps(&self) -> Result<AppRestartAll, Error> {
        let res: AppRestartResponseAll =
            make_request(&self.config, Method::PUT, "app/all/restart").await?;

        Ok(res.apps)
    }

    pub async fn set_app_ram(&self, id: &str, quantity: u32) -> Result<(), AppRamError> {
        let res: AppRamResponse = make_request_with_body(
            &self.config,
            Method::PUT,
            &format!("app/{id}/ram"),
            AppRamBody { ram: quantity },
        )
        .await?;

        if res.status == "error" {
            return Err(AppRamError::ForbiddenQuantity(res.message));
        }

        Ok(())
    }

    pub async fn commit_app(&self) {
        todo!()
    }

    pub async fn delete_app(&self, id: &str) -> Result<(), Error> {
        if id == "all" {
            return Err(Error::InvalidRequest(
                "Don't use all with that function. Use delete_all_apps method instead.",
            ));
        }

        let res: AppDeleteResponseUnique =
            make_request(&self.config, Method::DELETE, &format!("app/{id}/delete")).await?;

        if res.status == "error" {
            return Err(Error::Unknown);
        }

        Ok(())
    }

    pub async fn delete_all_apps(&self) -> Result<AppDeleteAll, Error> {
        let res: AppDeleteResponseAll =
            make_request(&self.config, Method::DELETE, "app/all/delete").await?;

        if res.status == "error" {
            return Err(Error::Unknown);
        }

        Ok(res.apps)
    }
}
