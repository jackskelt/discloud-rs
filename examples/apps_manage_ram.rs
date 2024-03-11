extern crate discloud_rs;
use std::env;

use discloud_rs::AppRamError;
use dotenvy::dotenv;
#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let client = discloud_rs::Discloud::new(&env::var("DISCLOUD_TOKEN").unwrap());

    let apps = client.get_all_apps().await.unwrap();

    println!("All apps fetched: {apps:?}");

    match apps.first() {
        Some(app) => {
            match app.set_ram(&client, 100).await {
                // Or client.set_app_ram(&a.id);
                Ok(_) => println!("Changed app ram: {app:#?}"),
                Err(error) => match error {
                    AppRamError::ForbiddenQuantity(message) => println!("{message}"),
                    AppRamError::Other(_e) => {}
                },
            };
        }
        None => {
            println!("No app to change ram");
        }
    }
}
