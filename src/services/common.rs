use dotenv::dotenv;
use minio_rsc::{provider::StaticProvider, Minio};
use std::env;

pub fn get_client_minio() -> Minio {
    dotenv().ok(); // Load the .env file

    let minio_url: String = env::var("MINIO_URL").expect("You've not set the MINIO_URL");
    let minio_key =
        env::var("MINIO_ACCESS_KEY_FILE").expect("You've not set the MINIO_ACCESS_KEY_FILE");
    let minio_secret =
        env::var("MINIO_SECRET_KEY_FILE").expect("You've not set the MINIO_SECRET_KEY_FILE");

    println!("{} ", minio_url);
    println!("{minio_key} - {minio_secret}");

}
