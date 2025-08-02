#![allow(warnings)]

mod app;
mod client;
mod config;
mod error;
mod team_manager;
mod team;
mod upload;
mod user;
mod util;

pub use app::{
    backup::AppBackup,
    logs::{AppLogs, AppLogsTerminal},
    manage::{
        AppDeleteAll, AppRamError, AppRestartAll, AppStartAll, AppStartError, AppStartStatus,
        AppStopAll, AppStopError,
    },
    status::{AppStatus, NetIO},
    App,
};
pub use client::Discloud;
pub use error::Error;
pub use team_manager::{APITeamMember, TeamManager, TeamMember, TeamPerms};
pub use user::{Locale, User};

#[cfg(test)]
mod test {
    use std::env;

    use dotenvy::dotenv;
    use tokio::test;

    use crate::Discloud;

    #[tokio::test]
    async fn test() {
        dotenv().ok();
        let client = Discloud::new(&env::var("DISCLOUD_TOKEN").expect("Discloud token is required"));
        if let Err(err) = client.commit_app("1753838917006", "App.zip").await {
            panic!("{:?}", err);
        }
        println!("Ok");
    }
}