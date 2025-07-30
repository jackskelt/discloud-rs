use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppCommitResponseUnique {
    pub status: String,
    pub status_code: i32,
    pub message: String,
}