use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppBackupResponseUnique {
    pub backups: AppBackup,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppBackupResponseAll {
    pub backups: Vec<AppBackup>,
    pub message: String,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppBackup {
    pub id: String,
    pub url: String,
}
