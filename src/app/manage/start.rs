use serde::Deserialize;

use crate::Error;

#[derive(Debug)]
pub enum AppStartError {
    AlreadyOnline,
    Other(Error),
}

impl From<Error> for AppStartError {
    fn from(error: Error) -> Self {
        Self::Other(error)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppStartStatus {
    pub exit_code: i32,
    pub online: bool,
    pub ram_killed: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppStartAll {
    pub already_in_process: Vec<String>,
    pub already_online: Vec<String>,
    pub started: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppStartResponseUnique {
    pub message: String,
    pub status: String,
    pub app_status: Option<AppStartStatus>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppStartResponseAll {
    pub message: String,
    pub status: String,
    pub apps: AppStartAll,
}
