use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TeamPerms {
    #[serde(rename = "start_app")]
    Start,
    #[serde(rename = "stop_app")]
    Stop,
    #[serde(rename = "restart_app")]
    Restart,
    #[serde(rename = "logs_app")]
    Logs,
    #[serde(rename = "commit_app")]
    Commit,
    #[serde(rename = "edit_ram")]
    Ram,
    #[serde(rename = "backup_app")]
    Backup,
    #[serde(rename = "status_app")]
    Status,
}
