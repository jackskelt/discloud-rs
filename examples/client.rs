extern crate discloud_rs;
use std::env;

use dotenvy::dotenv;
#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let client = discloud_rs::Discloud::new(&env::var("DISCLOUD_TOKEN").unwrap());

    let user = client.get_user_info().await.unwrap();

    println!("{user:?}");
}
