use serde::Deserialize;

pub mod backup;
pub mod logs;
pub mod manage;
pub mod status;

use crate::{Discloud, Error, TeamMember, TeamPerms};
use self::{
    backup::AppBackup,
    logs::AppLogs,
    manage::{AppRamError, AppStartError, AppStartStatus, AppStopError},
    status::AppStatus,
};

#[derive(Deserialize, Debug, Clone)]
pub struct AppResponseAll {
    pub apps: Vec<App>,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppResponseUnique {
    pub apps: App,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub auto_deploy_git: String,
    pub auto_restart: bool,
    #[serde(rename = "avatarURL")]
    pub avatar_url: String,
    pub exit_code: Option<i32>,
    pub id: String,
    pub lang: String,
    pub main_file: String,
    pub mods: Vec<String>,
    pub name: String,
    pub online: bool,
    pub ram: i32,
    pub ram_killed: bool,
    pub r#type: i32,
}

impl App {
    pub async fn get_status(&self, client: &Discloud) -> Result<AppStatus, Error> {
        client.get_app_status(&self.id).await
    }

    pub async fn get_logs(&self, client: &Discloud) -> Result<AppLogs, Error> {
        client.get_app_logs(&self.id).await
    }

    pub async fn get_backup(&self, client: &Discloud) -> Result<AppBackup, Error> {
        client.get_app_backup(&self.id).await
    }

    pub async fn start(&self, client: &Discloud) -> Result<AppStartStatus, AppStartError> {
        client.start_app(&self.id).await
    }

    pub async fn stop(&self, client: &Discloud) -> Result<(), AppStopError> {
        client.stop_app(&self.id).await
    }

    pub async fn restart(&self, client: &Discloud) -> Result<(), Error> {
        client.restart_app(&self.id).await
    }

    pub async fn set_ram(&self, client: &Discloud, quantity: u32) -> Result<(), AppRamError> {
        client.set_app_ram(&self.id, quantity).await
    }

    pub async fn commit(&self, client: &Discloud, id: &str, filepath: &str) -> Result<(), Error> {
        client.commit_app(&self.id, filepath).await
    }

    pub async fn delete(&self, client: &Discloud) -> Result<(), Error> {
        client.delete_app(&self.id).await
    }

    pub async fn get_team(&self, client: &Discloud) -> Result<Vec<TeamMember>, Error> {
        client.get_app_team(&self.id).await
    }

    pub async fn add_mod(
        &self,
        client: &Discloud,
        mod_id: &str,
        perms: Vec<TeamPerms>,
    ) -> Result<TeamMember, Error> {
        client.add_app_mod(&self.id, mod_id, perms).await
    }

    pub async fn edit_mod(
        &self,
        client: &Discloud,
        mod_id: &str,
        perms: Vec<TeamPerms>,
    ) -> Result<TeamMember, Error> {
        client.edit_app_mod(&self.id, mod_id, perms).await
    }

    pub async fn remove_mod(&self, client: &Discloud, mod_id: &str) -> Result<(), Error> {
        client.remove_app_mod(&self.id, mod_id).await
    }
}
