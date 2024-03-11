use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppLogsResponseAll {
    pub apps: Vec<AppLogs>,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppLogsResponseUnique {
    pub apps: AppLogs,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppLogs {
    pub id: String,
    pub terminal: AppLogsTerminal,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppLogsTerminal {
    pub big: Option<String>,
    pub small: Option<String>,
    pub url: String,
}
