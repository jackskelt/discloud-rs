use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppRestartAll {
    pub already_in_process: Vec<String>,
    pub already_offline: Vec<String>,
    pub restarted: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppRestartResponseAll {
    pub message: String,
    pub status: String,
    pub apps: AppRestartAll,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppRestartResponseUnique {
    pub message: String,
    pub status: String,
}
