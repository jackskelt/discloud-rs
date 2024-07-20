#![allow(warnings)]

mod app;
mod client;
mod config;
mod error;
mod team_manager;
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
