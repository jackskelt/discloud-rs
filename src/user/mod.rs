use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub message: String,
    pub user: User,
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "userID")]
    pub user_id: String,
    #[serde(rename = "totalRamMb")]
    pub total_ram: u32,
    #[serde(rename = "ramUsedMb")]
    pub used_ram: u32,
    pub subdomains: Vec<String>,
    #[serde(rename = "customdomains")]
    pub custom_domains: Vec<String>,
    pub apps: Vec<String>,
    pub plan: String,
    pub locale: String,
    #[serde(rename = "planDataEnd")]
    pub plan_date_end: String,
}

#[derive(Deserialize, Debug)]
pub struct LocaleResponse {
    pub status: String,
    pub locale: Locale,
}

#[derive(Deserialize, Debug, Clone)]
pub enum Locale {
    #[serde(rename = "pt-BR")]
    PtBR,
    #[serde(rename = "en-US")]
    EnUS,
}

impl Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PtBR => write!(f, "pt-BR"),
            Self::EnUS => write!(f, "en-US"),
        }
    }
}
