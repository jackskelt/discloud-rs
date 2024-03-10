extern crate discloud_rs;
use std::env;

use dotenvy::dotenv;
#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let client = discloud_rs::Discloud::new(&env::var("DISCLOUD_TOKEN").unwrap());

    let apps = client.get_all_apps().await.unwrap();

    println!("All apps fetched: {apps:?}");

    match apps.first() {
        Some(a) => {
            let app = client.get_app(&a.id).await.unwrap();
            println!("Fetched unique app: {app:#?}");
        }
        None => {
            println!("No app to fetch unique");
        }
    }
}
