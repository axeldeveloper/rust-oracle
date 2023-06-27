use std::result::Result;
use std::error::Error;
use minio_rsc::types::response::ListBucketResult;
use minio_rsc::types::{Bucket};

use reqwest::Response;
use minio_rsc::types::args::{ObjectArgs, ListObjectsArgs};
use minio_rsc::errors::Result as MResult;
use std::env;
use dotenv::dotenv;

mod common;

pub async fn buckets_print() {
    dotenv().ok(); // Load the .env file

    let minio_url = env::var("MINIO_URL").expect("You've not set the MINIO_ACCESS_KEY_FILE");
    let minio_key = env::var("MINIO_ACCESS_KEY_FILE").expect("You've not set the MINIO_ACCESS_KEY_FILE");
    let minio_secret = env::var("MINIO_SECRET_KEY_FILE").expect("You've not set the MINIO_SECRET_KEY_FILE");

    println!( "{:?}", minio_url );
    println!( "{:?}", minio_key );
    println!( "{:?}", minio_secret );
}


pub async fn async_buckets_print() {
    println!("------------------------------------------------------------------------------");
    let minio = common::get_client_minio();
    let (buckets, owner) = minio.list_buckets().await.unwrap();
    println!(" Service Owner {}", owner.display_name );
    for obj in buckets {
        println!("{}", obj.creation_date );
        println!("{}", obj.name );
    }
}



/*
pub async fn list_buckets() -> Result<Vec<Bucket>, Box<dyn Error>>  {
    println!("------------------------------------------------------------------------------");
    let minio = get_client_minio();
    let (buckets, _owner) = minio.list_buckets().await.unwrap();
    Ok(buckets)
}


pub async fn get_buckets_object() -> MResult<Response>  {
    println!("------------------------------------------------------------------------------");
    let minio = get_client_minio();
    let response: Response = minio.get_object(
        ObjectArgs::new("pasta-virtual", "mpdf.pdf")
            //.version_id(Some("cdabf31a-9752-4265-b137-6b3961fbaf9b".to_string()))
        ).await?;

    Ok(response)
}



pub async fn list_objects() -> ListBucketResult {
    println!("------------------------------------------------------------------------------");
    let minio = get_client_minio();

    let args = ListObjectsArgs::new("pasta-virtual")
    .max_keys(10);

    Ok(minio.list_objects(args).await);
}



*/



//fn make_error() -> Result<(), String> { Err("error".to_string()) }