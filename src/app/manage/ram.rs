use serde::{Deserialize, Serialize};

use crate::Error;

#[derive(Deserialize, Debug, Clone)]
pub struct AppRamResponse {
    pub message: String,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct AppRamBody {
    #[serde(rename = "ramMB")]
    pub ram: u32,
}

#[derive(Debug)]
pub enum AppRamError {
    ForbiddenQuantity(String),
    Other(Error),
}

impl From<Error> for AppRamError {
    fn from(error: Error) -> Self {
        Self::Other(error)
    }
}
