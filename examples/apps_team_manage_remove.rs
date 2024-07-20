extern crate discloud_rs;
use dotenvy::dotenv;
use std::env;

use discloud_rs::Discloud;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let client = Discloud::new(&env::var("DISCLOUD_TOKEN").unwrap());

    let apps = client.get_all_apps().await.unwrap();

    if let Some(app) = apps.first() {
        let mods = app.get_team(&client).await.unwrap();

        if let Some(app_mod) = mods.first() {
            app_mod.remove(&client).await.unwrap();
        }
    } else {
        println!("No app to fetch team");
    }
}
