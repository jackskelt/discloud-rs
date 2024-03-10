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
  - [ ] Manage (start, restart, stop, ram, commit, delete)
- [ ] Team Manager
- [ ] Team
  - [ ] Manage (start, restart, stop, commit, status)
  - [ ] Backup
  - [ ] Logs
  - [ ] Status

## Usage

## User

### Get user info
```rs
use discloud_rs::Discloud;

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    let user = client.get_user_info().await.unwrap();
}
```

### Set locale
```rs
use discloud_rs::{ Discloud, Locale };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    client.set_locale(Locale::PtBR).await.unwrap(); // Set language to Brazilian Portuguese
}
```

## App

### Get app
```rs
use discloud_rs::{ Discloud };

#[tokio::main]
async fn main() {
    let client = Discloud::new("TOKEN");

    client.get_all_apps().await.unwrap(); // Get all apps

    client.get_app("APP_ID").await.unwrap(); // Get app by id
}
```

### Get app status
```rs
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
```rs
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
```rs
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