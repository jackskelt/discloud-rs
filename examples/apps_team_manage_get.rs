extern crate discloud_rs;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let client = discloud_rs::Discloud::new(&env::var("DISCLOUD_TOKEN").unwrap());

    let apps = client.get_all_apps().await.unwrap();

    if let Some(app) = apps.first() {
        if let Ok(team) = app.get_team(&client).await {
            println!("Fetched app team: {team:#?}");
        } else {
            println!("Failed to fetch app team");
        }
    } else {
        println!("No app to fetch team");
    }
}
