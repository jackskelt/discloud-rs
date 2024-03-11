extern crate discloud_rs;
use std::env;

use discloud_rs::AppStopError;
use dotenvy::dotenv;
#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let client = discloud_rs::Discloud::new(&env::var("DISCLOUD_TOKEN").unwrap());

    let apps = client.get_all_apps().await.unwrap();

    println!("All apps fetched: {apps:?}");

    match apps.first() {
        Some(app) => {
            match app.stop(&client).await {
                // Or client.stop_app(&a.id);
                Ok(_) => println!("Stopped app: {app:#?}"),
                Err(error) => match error {
                    AppStopError::AlreadyOffline => println!("App is already offline"),
                    AppStopError::Other(e) => println!("Other error: {e:#?}"),
                },
            };
        }
        None => {
            println!("No app to stop");
        }
    }
}
