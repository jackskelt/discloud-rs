use serde::Deserialize;

use crate::Error;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppStopAll {
    pub already_in_process: Vec<String>,
    pub already_offline: Vec<String>,
    pub stopped: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppStopResponseUnique {
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppStopResponseAll {
    pub message: String,
    pub status: String,
    pub apps: AppStopAll,
}

#[derive(Debug)]
pub enum AppStopError {
    AlreadyOffline,
    Other(Error),
}

impl From<Error> for AppStopError {
    fn from(error: Error) -> Self {
        Self::Other(error)
    }
}
