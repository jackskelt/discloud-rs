use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppDeleteAll {
    pub already_in_process: Vec<String>,
    pub already_offline: Vec<String>,
    pub removealled: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppDeleteResponseAll {
    pub message: String,
    pub status: String,
    pub apps: AppDeleteAll,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppDeleteResponseUnique {
    pub message: String,
    pub status: String,
}
