use crate::Discloud;
use serde::{Deserialize, Serialize};

pub use perms::*;
pub use team_member::*;

mod perms;
mod team_member;

#[derive(Debug, Clone)]
pub struct TeamManager {
    pub app_id: String,
}

#[derive(Debug, Deserialize)]
pub struct GetTeamManagerResponse {
    pub status: String,
    pub team: Vec<APITeamMember>,
}

#[derive(Debug, Deserialize)]
pub struct AddTeamMemberResponse {
    pub status: String,
    pub app: TeamMember,
}

#[derive(Debug, Serialize)]
pub struct TeamMemberBody {
    #[serde(rename = "modID")]
    pub mod_id: String,
    pub perms: Vec<TeamPerms>,
}

impl TeamManager {
    pub async fn get_app_team(&self, client: &Discloud) -> Result<Vec<TeamMember>, crate::Error> {
        client.get_app_team(&self.app_id).await
    }

    pub async fn add_app_mod(
        &self,
        client: &Discloud,
        mod_id: &str,
        perms: Vec<TeamPerms>,
    ) -> Result<TeamMember, crate::Error> {
        client.add_app_mod(&self.app_id, mod_id, perms).await
    }
}
