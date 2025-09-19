use serde::Deserialize;

use crate::{AppBackup, AppLogs, AppRamError, AppStartError, AppStartStatus, AppStatus, AppStopError, Discloud, Error};

#[derive(Deserialize, Debug, Clone)]
pub struct TeamAppsResponseUnique {
    pub status: String,
    pub message: String,
    pub apps: Vec<TeamApp>
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamApp {
    pub id: String,
    pub name: String,
    pub online: bool,
    pub ram: u32,
    pub ram_killed: bool,
    pub exit_code: i32,
    pub lang: String,
    pub perms: Vec<String>,
    pub r#type: i32
}

impl TeamApp {
    pub async fn start(&self, client: &Discloud) -> Result<AppStartStatus, AppStartError> {
        client.start_team_app(&self.id).await
    }

    pub async fn restart(&self, client: &Discloud) -> Result<(), Error> {
        client.restart_team_app(&self.id).await
    }

    pub async fn stop(&self, client: &Discloud) -> Result<(), AppStopError> {
        client.stop_team_app(&self.id).await
    }

    pub async fn commit(&self, filepath: &str, client: &Discloud) -> Result<(), Error> {
        client.commit_team_app(&self.id, filepath).await
    }

    pub async fn get_backup(&self, client: &Discloud) -> Result<AppBackup, Error> {
        client.get_team_app_backup(&self.id).await
    }

    pub async fn get_logs(&self, client: &Discloud) -> Result<AppLogs, Error> {
        client.get_team_app_logs(&self.id).await
    }

    pub async fn set_ram(&self, quantity: u32, client: &Discloud) -> Result<(), AppRamError> {
        client.set_team_app_ram(&self.id, quantity).await
    }

    pub async fn get_status(&self, client: &Discloud) -> Result<AppStatus, Error> {
        client.get_team_app_status(&self.id).await
    }
}