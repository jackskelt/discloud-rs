extern crate discloud_rs;
use std::env;

use discloud_rs::AppStartError;
use dotenvy::dotenv;
#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let client = discloud_rs::Discloud::new(&env::var("DISCLOUD_TOKEN").unwrap());

    let apps = client.get_all_apps().await.unwrap();

    println!("All apps fetched: {apps:?}");

    match apps.first() {
        Some(app) => {
            match app.start(&client).await {
                // Or client.start_app(&a.id);
                Ok(_) => println!("Started app: {app:#?}"),
                Err(error) => match error {
                    AppStartError::AlreadyOnline => println!("App already online"),
                    AppStartError::Other(e) => println!("Other error: {e:#?}"),
                },
            };
        }
        None => {
            println!("No app to start");
        }
    }
}
