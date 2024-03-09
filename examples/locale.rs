extern crate discloud_rs;

use dotenvy::dotenv;
use std::env;

use discloud_rs::Locale;

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let client = discloud_rs::Discloud::new(&env::var("DISCLOUD_TOKEN").unwrap());

    client.set_locale(Locale::PtBR).await.unwrap();
}
