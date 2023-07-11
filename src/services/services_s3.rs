use std::env;
use dotenv::dotenv;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::Bucket;

pub fn test_list() -> Result<(), S3Error> {
    dotenv().ok(); // Load the .env file

    let minio_url:  &str = &env::var("MINIO_URL").unwrap();
    let access_key: &str = &env::var("MINIO_ACCESS_KEY_FILE").unwrap();
    let secret_key: &str = &env::var("MINIO_SECRET_KEY_FILE").unwrap();
    let bucket_name = "pasta-virtual";
    let credentials = Credentials::new(Some(access_key), Some(secret_key),None, None,None).unwrap();

    println!( " URL Bucket =>  {}", minio_url  );

    //let credentials = Credentials::default().unwrap();
    //let region = "us-east-1".parse().unwrap();
    //let region = "".parse()?; // your region
    let region = s3::Region::Custom {region: "".into(), endpoint: "minio.sad.ms.gov.br".into(),};


    // This requires a running minio server at localhost:9000
    let buckets = Bucket::new(bucket_name, region, credentials);

    match buckets {
        Ok(bucket) => {
            println!( " Bucket =>  {}", bucket.name  );
            //let results = bucket.list("/".to_string(), Some("/".to_string())).await?;
            //println!("{:?}", results);
            //let results = bucket.list("dados-pesoais".to_string(), Some("/".to_string())  ).await?;
            //for result in results {
            //   println!("Found job {:?}", result);
            //
        },

        Err(e) => {println!("{}", e)},
    }

    println!( " Fim ",  );
    Ok(())
}