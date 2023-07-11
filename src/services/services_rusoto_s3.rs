use std::{error::Error, io::{ErrorKind, Read}};

use futures::TryStreamExt;
use log::warn;
use rusoto_core::{Region, RusotoError, credential::StaticProvider, Client, HttpClient};
use rusoto_s3::{S3Client, ListObjectsV2Request, S3, GetObjectRequest};

use std::future::Future;
use tokio::runtime::Runtime;
use bytes::BytesMut;



pub async fn test_list() {

    println!( " -------------------------------------------------" );

    let credentials = StaticProvider::new(
        "0PHbZF2utbnk2f0iA9Pc".to_owned(),
        "L382deOPcTKoYUqAJ54Y3lixkjS4cs0C".to_owned(),
        None,
        None,
    );

    let client = Client::new_with(credentials, HttpClient::new().unwrap());
    let region = Region::Custom {
        name: String::from(""),  // region name in endpoint
        endpoint: String::from("https://minio.sad.ms.gov.br"),
    };

    //let s3 = S3Client::new();


    let s3 = S3Client::new_with_client(client, region.clone());

    println!( " -------------------------------------------------" );

    let mut request = ListObjectsV2Request {
        bucket: String::from("pasta-virtual"),
        ..Default::default()
    };

    let result = s3.list_objects_v2(request.clone()).await;

    match result {
        Ok(bucket) => {
            println!( " Bucket 2 =>  {}", bucket.name.unwrap()  );
            //let results = bucket.list("/".to_string(), Some("/".to_string())).await?;
            //println!("{:?}", results);
            //let results = bucket.list("dados-pesoais".to_string(), Some("/".to_string())  ).await?;
            //for result in results {
            //   println!("Found job {:?}", result);
            //
        },

        Err(e) => {println!("{}", e)},
    }

    println!( " Fim 2",  );



}


pub async fn bucket_obj_bytes( _prefix: String, object: String) {
    let bucket: String = "pasta-virtual".to_owned();
    let credentials = StaticProvider::new(
        "0PHbZF2utbnk2f0iA9Pc".to_owned(), "L382deOPcTKoYUqAJ54Y3lixkjS4cs0C".to_owned(),None, None, );

    let region = Region::Custom { name: String::from(""), endpoint: String::from("https://minio.sad.ms.gov.br") };

    let client = Client::new_with(credentials, HttpClient::new().unwrap());
    let s3_client = S3Client::new_with_client(client, region.clone());


    let get_req = GetObjectRequest {
        bucket,
        key: object,
        ..Default::default()
    };

    let result = s3_client.get_object(get_req)
        .await
        .expect("Couldn't GET object");
    println!("get object result: {:#?}", result);

    let stream = result.body.unwrap();
    let body = stream.map_ok(|b| BytesMut::from(&b[..])).try_concat().await.unwrap();

    assert!(body.len() > 0);
    dbg!(body);
}



pub fn get_object(object_key: &str)  {
    let bucket_name = "pasta-virtual";
    let credentials = StaticProvider::new(
        "0PHbZF2utbnk2f0iA9Pc".to_owned(), "L382deOPcTKoYUqAJ54Y3lixkjS4cs0C".to_owned(),None, None, );

    let region = Region::Custom { name: String::from(""), endpoint: String::from("https://minio.sad.ms.gov.br") };

    let client = Client::new_with(credentials, HttpClient::new().unwrap());
    let s3_client = S3Client::new_with_client(client, region.clone());

    let mut or = GetObjectRequest::default();
    or.bucket = bucket_name.to_string();
    or.key = object_key.to_string();
    //let _ = env_logger::try_init();
    let get_object_output = s3_client.get_object(or);
    let r = async_run(get_object_output);

    /*let _err = Error::new(
        ErrorKind::Other,
        format!(
            "something goes wrong while getting object {} in the S3 bucket {}",
            object_key, bucket_name
        ),
    );*/

    match r {
        Err(err) => {
            warn!("{}", err);
            //Err(_err)
        }

        Ok(x) => {
            let mut s = String::new();
            x.body.unwrap().into_blocking_read().read_to_string(&mut s);

            if s.is_empty() {
                // It looks like we receive sometimes empty content from s3. This is a quick and dirty patch, is there another better way ? Like using s3 Hash ?
                /*
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "file content is empty - which is not the expected content - what's wrong?",
                ));
                */
            }

            //Ok(s)
        }
    }

}

pub fn async_run<F: Future>(future: F) -> F::Output {
    // TODO improve - is it efficient to create a Runtime at each exec?
    let mut runtime = Runtime::new().expect("unable to create a tokio runtime");
    runtime.block_on(future)
}
