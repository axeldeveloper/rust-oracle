use minio_rsc::{provider::StaticProvider, Minio};
use std::env;
use dotenv::dotenv;


pub fn get_client_minio() -> Minio {
  dotenv().ok(); // Load the .env file

  let minio_key = env::var("MINIO_ACCESS_KEY_FILE").expect("You've not set the MINIO_ACCESS_KEY_FILE");
  let minio_secret = env::var("MINIO_SECRET_KEY_FILE").expect("You've not set the MINIO_SECRET_KEY_FILE");

  let provider = StaticProvider::new(minio_key, minio_secret, None);
  Minio::builder()
      .host("localhost:9000")
      .provider(provider)
      .secure(false)
      .build()
      .unwrap()
}