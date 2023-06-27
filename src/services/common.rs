use minio_rsc::{provider::StaticProvider, Minio};
use std::env;
use dotenv::dotenv;


pub fn get_client_minio() -> Minio {
  dotenv().ok(); // Load the .env file

  println!("https://console.minio.terra.open:800/");

  let minio_key = env::var("MINIO_ACCESS_KEY_FILE").expect("You've not set the MINIO_ACCESS_KEY_FILE");
  let minio_secret = env::var("MINIO_SECRET_KEY_FILE").expect("You've not set the MINIO_SECRET_KEY_FILE");

  println!("{minio_key} - {minio_secret}");

  let provider = StaticProvider::new(minio_key, minio_secret, None);
  Minio::builder()
      .host("https://console.minio.terra.open:80")
      .provider(provider)
      .secure(false)
      .build()
      .unwrap()
}




/*
fn print_vars(){
    dotenv().ok();
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}

fn read_number() -> Option<u8> {
    let input = read_string();
    u8::from_str(&input).ok()
}

*/
