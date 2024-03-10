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
            let logs = app.get_logs(&client).await.unwrap(); // or client.get_app_logs(&app.id)
            println!("Fetched app logs: {logs:#?}");
        }
        None => {
            println!("No app to fetch logs");
        }
    }
}
