use serde::Deserialize;

use crate::Discloud;

use super::TeamPerms;

#[derive(Debug, Deserialize)]
pub struct APITeamMember {
    #[serde(rename = "modID")]
    pub id: String,
    pub perms: Vec<TeamPerms>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TeamMember {
    #[serde(rename = "modID")]
    pub mod_id: String,
    #[serde(rename = "appID")]
    pub app_id: String,
    pub perms: Vec<TeamPerms>,
}

impl TeamMember {
    pub async fn remove(&self, client: &Discloud) -> Result<(), crate::Error> {
        client.remove_app_mod(&self.app_id, &self.mod_id).await
    }

    pub async fn edit_perms_mut(
        &mut self,
        client: &Discloud,
        perms: Vec<TeamPerms>,
    ) -> Result<(), crate::Error> {
        let res = client
            .edit_app_mod(&self.app_id, &self.mod_id, perms)
            .await?;

        *self = res;

        Ok(())
    }

    pub async fn edit_perms(
        &self,
        client: &Discloud,
        perms: Vec<TeamPerms>,
    ) -> Result<TeamMember, crate::Error> {
        client.edit_app_mod(&self.app_id, &self.mod_id, perms).await
    }

    pub async fn add_perms_mut(
        &mut self,
        client: &Discloud,
        perms: Vec<TeamPerms>,
    ) -> Result<(), crate::Error> {
        let mut new_perms = self.perms.clone();
        new_perms.extend(perms);
        new_perms.sort_unstable();
        Vec::dedup(&mut new_perms);

        let res = client
            .edit_app_mod(&self.app_id, &self.mod_id, new_perms)
            .await?;
        *self = res;

        Ok(())
    }

    pub async fn add_perms(
        &self,
        client: &Discloud,
        perms: Vec<TeamPerms>,
    ) -> Result<TeamMember, crate::Error> {
        let mut new_perms = self.perms.clone();
        new_perms.extend(perms);
        new_perms.sort_unstable();
        Vec::dedup(&mut new_perms);
        client
            .edit_app_mod(&self.app_id, &self.mod_id, new_perms)
            .await
    }
}
