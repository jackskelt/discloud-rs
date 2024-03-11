use serde::Deserialize;

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
