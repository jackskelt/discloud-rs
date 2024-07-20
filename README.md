![discloud and ferris](https://github.com/jackskelt/discloud-rs/assets/86922268/42249ccb-4e20-4c60-a463-7dfd06466c32)

# discloud-rs

A wrapper for [Discloud's API](https://docs.discloudbot.com/api/usar-a-api) made in **Rust** 🦀.

The crate has [tracing](https://crates.io/crates/tracing) for debug.

## [Routes](https://discloud.github.io/apidoc/)

*Click for usage example*

- [x] [User](#user)
  - [x] [Get user info](#get-user-info)
  - [x] [Set locale](#set-locale)
- [ ] Upload
- [ ] [App](#app)
  - [x] [Info](#get-app)
  - [x] [Status](#get-app-status)
  - [x] [Logs](#get-app-logs)
  - [x] [Backup](#get-app-backup)
  - [ ] [Manage](#manage-app) ([start](#start), [restart](#restart), [stop](#stop), [ram](#set-ram), commit, [delete](#delete))
- [x] [Team Manager](#team-manage)
- [ ] Team
  - [ ] Manage (start, restart, stop, commit, status)
  - [ ] Backup
  - [ ] Logs
  - [ ] Status

## Usage

## User

### Get user info

```rust,no_run
use discloud_rs::Discloud;

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    let user = client.get_user_info().await.unwrap();
}
```

### Set locale

```rust,no_run
use discloud_rs::{ Discloud, Locale };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    client.set_locale(Locale::PtBR).await.unwrap(); // Set language to Brazilian Portuguese
}
```

## App

### Get app

```rust,no_run
use discloud_rs::{ Discloud };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    client.get_all_apps().await.unwrap(); // Get all apps

    client.get_app("APP_ID").await.unwrap(); // Get app by id
}
```

### Get app status

```rust,no_run
use discloud_rs::{ Discloud };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    client.get_all_apps_status().await.unwrap(); // Get all apps status

    client.get_app_status("APP_ID").await.unwrap(); // Get app status by id

    // Get status from app
    let app = client.get_app("APP_ID").await.unwrap();
    app.get_status(&client).await.unwrap();
}
```

### Get app logs

```rust,no_run
use discloud_rs::{ Discloud };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    client.get_all_apps_logs().await.unwrap(); // Get all apps logs (Be careful if you have many apps)

    client.get_app_logs("APP_ID").await.unwrap(); // Get app logs by id

    // Get logs from app
    let app = client.get_app("APP_ID").await.unwrap();
    app.get_logs(&client).await.unwrap();
}
```

### Get app backup

```rust,no_run
use discloud_rs::{ Discloud };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    client.get_all_apps_backup().await.unwrap(); // Get all apps backup (This may take a while)

    client.get_app_backup("APP_ID").await.unwrap(); // Get app logs by id

    // Get logs from app
    let app = client.get_app("APP_ID").await.unwrap();
    app.get_backup(&client).await.unwrap();
}
```

## Manage App

### Start

```rust,no_run
use discloud_rs::{ Discloud, AppStartError };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    // Start all of your apps
    let response = client.start_all_apps().await.unwrap();

    // Start app by id
    match client.start_app("APP_ID").await {
      Ok(_) => { }, // App was started.
      Err(error) => {
          match error {
              AppStartError::AlreadyStarted => { }, // App is already online
              AppStartError::Other(e) => { } // discloud_rs::Error
          }
      }
    };

    // Start from app
    let app = client.get_app("APP_ID").await.unwrap();
    match app.start(&client).await {
      Ok(_) => { }, // App was started.
      Err(error) => {
          match error {
              AppStartError::AlreadyStarted => { }, // App is already online
              AppStartError::Other(e) => { } // discloud_rs::Error
          }
      }
    };
}
```

### Restart

```rust,no_run
use discloud_rs::{ Discloud };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    client.restart_all_apps().await.unwrap(); // Restart all of your apps

    client.restart_app("APP_ID").await.unwrap(); // Restart app by id

    // Restart from app
    let app = client.get_app("APP_ID").await.unwrap();
    app.restart(&client).await.unwrap();
}
```

### Stop

```rust,no_run
use discloud_rs::{ Discloud, AppStopError };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    // Stop all of your apps
    client.stop_all_apps().await.unwrap();

    // Stop app by id
    match client.stop_app("APP_ID").await {
      Ok(_) => { }, // App was stopped.
      Err(error) => {
          match error {
              AppStopError::AlreadyStopped => { }, // App is already stopped
              AppStopError::Other(e) => { } // discloud_rs::Error
          }
      }
    };

    // Stop from app
    let app = client.get_app("APP_ID").await.unwrap();
    match app.stop(&client).await {
      Ok(_) => { }, // App was stopped.
      Err(error) => {
          match error {
              AppStopError::AlreadyStopped => { }, // App is already stopped
              AppStopError::Other(e) => { } // discloud_rs::Error
          }
      }
    };
}
```

### Set RAM

```rust,no_run
use discloud_rs::{ Discloud, AppRamError };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    // Set app ram (in MB)
    match client.set_app_ram("APP_ID", 100).await {
      Ok(_) => { }, // App ram was changed
      Err(error) => {
          match error {
              AppRamError::ForbiddenQuantity(message) => { }, // Forbidden quantity (Not enough memory or minimum required)
              AppRamError::Other(e) => { } // discloud_rs::Error
          }
      }
    };

    // Set ram from app
    let app = client.get_app("APP_ID").await.unwrap();
    match app.set_ram(&client, 100).await {
      Ok(_) => { }, // App ram was changed
      Err(error) => {
          match error {
              AppRamError::ForbiddenQuantity(message) => { }, // Forbidden quantity (Not enough memory or minimum required
              AppRamError::Other(e) => { } // discloud_rs::Error
          }
      }
    };
}
```

### Delete

```rust,no_run
use discloud_rs::Discloud;

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    // Delete all apps
    client.delete_all_apps().await.unwrap();

    // Delete app
    client.delete_app("APP_ID").await.unwrap();

    // Delete from app
    let app = client.get_app("APP_ID").await.unwrap();
    app.delete(&client).await.unwrap();
}
```

## Team Manage

### Get app team

```rust,no_run
use discloud_rs::Discloud;

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    // Get app team using client
    client.get_app_team("APP_ID").await.unwrap();

    // Get app team using app
    let app = client.get_app("APP_ID").await.unwrap();
    app.get_team(&client).await.unwrap();
}
```

### Add app mod

```rust,no_run
use discloud_rs::{Discloud, TeamPerms};

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    // Add app mod using client
    client.add_app_mod("APP_ID", "MOD_ID", vec![TeamPerms::Start, TeamPerms::Stop]).await.unwrap();

    // Add app mod using app
    let app = client.get_app("APP_ID").await.unwrap();
    app.add_mod(&client, "MOD_ID", vec![TeamPerms::Start, TeamPerms::Stop]).await.unwrap();
}
```

### Edit app mod

```rust,no_run
use discloud_rs::{Discloud, TeamPerms};

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    // Edit mod permissions using client
    client.edit_app_mod("APP_ID", "MOD_ID", vec![TeamPerms::Start, TeamPerms::Stop]).await.unwrap();

    // Edit mod permissions using app
    let app = client.get_app("APP_ID").await.unwrap();
    app.edit_mod(&client, "MOD_ID", vec![TeamPerms::Start, TeamPerms::Stop]).await.unwrap();

    // Edit mod permissions from mod
    if let Some(app_mod) = app.get_team(&client).await.unwrap().first() {
      app_mod.edit_perms(&client, vec![TeamPerms::Start, TeamPerms::Stop]).await.unwrap();

      // Or add permissions to mod
      app_mod.add_perms(&client, vec![TeamPerms::Start, TeamPerms::Stop]).await.unwrap();
  };
}
```

### Remove app mod

```rust,no_run
use discloud_rs::{Discloud, TeamPerms};

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    // Remove mod using client
    client.remove_app_mod("APP_ID", "MOD_ID").await.unwrap();

    // Remove mod using app
    let app = client.get_app("APP_ID").await.unwrap();
    app.remove_mod(&client, "MOD_ID").await.unwrap();

    // Remove from mod
    if let Some(app_mod) = app.get_team(&client).await.unwrap().first() {
      app_mod.remove(&client).await.unwrap();
  };
}
```
