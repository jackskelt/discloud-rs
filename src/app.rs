use serde::Deserialize;

use crate::{Discloud, Error};

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
    pub exit_code: i32,
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

#[derive(Deserialize, Debug, Clone)]
pub struct AppStatusResponseUnique {
    pub apps: AppStatus,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppStatusResponseAll {
    pub apps: Vec<AppStatus>,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppStatus {
    pub container: String,
    pub cpu: String,
    pub id: String,
    pub last_restart: String,
    pub memory: String,
    #[serde(rename = "netIO")]
    pub net_io: NetIO,
    pub ssd: String,
    #[serde(rename = "startedAt")]
    pub started_at: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NetIO {
    pub down: String,
    pub up: String,
}

impl App {
    pub async fn get_status(&self, client: &Discloud) -> Result<AppStatus, Error> {
        client.get_app_status(&self.id).await
    }
}
