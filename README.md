![discloud and ferris](https://github.com/jackskelt/discloud-rs/assets/86922268/42249ccb-4e20-4c60-a463-7dfd06466c32)

# discloud-rs


A wrapper for [Discloud's API](https://docs.discloudbot.com/api/usar-a-api) made in **Rust** 🦀.

The crate has [tracing](https://crates.io/crates/tracing) for debug.

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

