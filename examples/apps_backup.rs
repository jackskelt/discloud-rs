extern crate discloud_rs;
use std::env;

use dotenvy::dotenv;
#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let client = discloud_rs::Discloud::new(&env::var("DISCLOUD_TOKEN").unwrap());

    let apps = client.get_all_apps().await.unwrap();

    match apps.first() {
        Some(app) => {
            let backup = app.get_backup(&client).await.unwrap(); // or client.get_app_backup(&app.id)
            println!("Fetched app backup: {backup:#?}");
        }
        None => {
            println!("No app to fetch backup");
        }
    }
}
