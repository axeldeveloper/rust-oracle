use std::result::Result;
use std::error::Error;
use minio_rsc::types::{Bucket};
use minio_rsc::types::args::{ObjectArgs, ListObjectsArgs};
use minio_rsc::errors::Result as MResult;
use minio_rsc::types::response::ListBucketResult;
use reqwest::Response;
use crate::common::get_client_minio;


//use minio_rsc::errors;
//use crate::errors::{Error, Result, XmlError} as a,b,c ;
//use minio_rsc::client::Minio;
//use minio_rsc::provider::StaticProvider;
//use minio_rsc::errors::{Result, XmlError, MinioError};


pub async fn async_buckets_print() {
    println!("------------------------------------------------------------------------------");
    let minio = get_client_minio();
    let (buckets, owner) = minio.list_buckets().await.unwrap();
    println!(" Service Owner {}", owner.display_name );
    for obj in buckets {
        println!("{}", obj.creation_date );
        println!("{}", obj.name );
    }
}

pub async fn list_buckets() -> Result<Vec<Bucket>, Box<dyn Error>>  {
    println!("------------------------------------------------------------------------------");
    let minio = get_client_minio();
    let (buckets, _owner) = minio.list_buckets().await.unwrap();
    Ok(buckets)
}

pub async fn get_object() -> MResult<Response>  {
    println!("------------------------------------------------------------------------------"); 
    let minio = get_client_minio();
    let response: Response = minio.get_object(
        ObjectArgs::new("pasta-virtual", "mpdf.pdf")
            //.version_id(Some("cdabf31a-9752-4265-b137-6b3961fbaf9b".to_string()))
        ).await?;

    Ok(response)
}



/*


pub async fn list_objects() -> ListBucketResult {
    println!("------------------------------------------------------------------------------");
    let minio = get_client_minio();

    let args = ListObjectsArgs::new("pasta-virtual")
    .max_keys(10);

    Ok(minio.list_objects(args).await);
}



*/



//fn make_error() -> Result<(), String> { Err("error".to_string()) }