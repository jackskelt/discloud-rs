# discloud-rs

A wrapper for [Discloud's API](https://docs.discloudbot.com/api/usar-a-api) made in **Rust** 🦀.

The crate have [tracing](https://crates.io/crates/tracing) for debug.

## [Routes](https://discloud.github.io/apidoc/)

- [x] User
  - [x] Get user info
  - [x] Set locale
- [ ] Upload
- [ ] App
  - [ ] Info
  - [ ] Status
  - [ ] Logs
  - [ ] Backup
  - [ ] Manage (start, restart, stop, ram, commit, delete)
- [ ] Team Manager
- [ ] Team
  - [ ] Manage (start, restart, stop, commit, status)
  - [ ] Backup
  - [ ] Logs
  - [ ] Status

## Usage

### Get user info
```rs
extern crate discloud_rs;
#[tokio::main]
async fn main() {
    let client = discloud_rs::Discloud::new("TOKEN");

    let user = client.get_user_info().await.unwrap();
}
```

### Set locale
```rs
extern crate discloud_rs;

use discloud_rs::Locale;

#[tokio::main]
async fn main() {
    let client = discloud_rs::Discloud::new("TOKEN");

    client.set_locale(Locale::PtBR).await.unwrap(); // Set language to Portuguese from Brazil
}
```

