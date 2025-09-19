use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UploadOk {
    pub status: String,
    pub status_code: i32,
    pub message: String,
    pub app: UploadApp,
    pub logs: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UploadApp {
    pub added_at_timestamp: u64,
    pub auto_restart: bool,
    pub avatar_url: String,
    pub id: String,
    pub lang: String,
    pub main_file: String,
    pub name: String,
    pub ram: u32,
    pub r#type: i32,
    pub version: String,
}