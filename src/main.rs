
// Import diesel
//use diesel::prelude::*;
// Import the oracle connection type
//use diesel_oci::OciConnection;


/* 
table! {
  countries {
    id -> Integer,
    country_name -> Text,
    region_id -> Integer
  }
}
*/

// https://github.com/kubo/rust-oracle

use std::env;
use dotenv::dotenv;
//use minio_rsc::errors::MinioError;
use oracle::{Connection, Result};
use tokio;

#[path = "minio/services_minio.rs"] mod services_minio;
#[path = "minio/common.rs"] mod common;
use common::get_client_minio;


#[tokio::main]
async fn main() -> Result<()> {

    let minio = get_client_minio();

    services_minio::async_buckets_print().await;

    println!("{:?}", minio.list_buckets().await);
    println!("====== begin test tagging");
   
    let r7 = services_minio::list_buckets().await;
    match r7 {
        Ok(bucket) => println!("Bucket is {:?}!!", bucket),
        
        Err(e) => eprintln!("Oh noes, we don't know which era we're in! :( \n  {}", e),
      }

   
    println!("====== begin oracle ");

    let result = ler_ora();
    
    let response = match result {
        Ok(res) => println!("res is {:?}!!", res) ,
        Err(err) => return Err(err),
    };

    println!("to response {:?}!!", response);


    let r8 = services_minio::get_object().await;

    let resp = match r8 {
        Ok(res) => println!("res is {:?}!!", res.url().path()) ,
        
        //Err(error) => return S3Error(error) 
        
        Err(error) => {
            panic!("=> There was a problem opening the file: {:?}", error)
        },
    };

    println!("to resp {:?}!!", resp);

    Ok(())

}



fn ler_ora() -> Result<()> {
    dotenv().ok(); // Load the .env file
    println!("Oarecle data");
  
    let db_url = env::var("DB_URL").expect("You've not set the DB_URL");
    let db_user = env::var("DB_USER").expect("You've not set the DB_USER");
    let db_pwd = env::var("DB_PWD").expect("You've not set the DB_PWD");


    // Connect to a database.
    let conn = Connection::connect(db_user, db_pwd, db_url)?;
    let (server_ver, banner) = conn.server_version()?;
    println!("\nDatabase Server Version: {}", server_ver);
    println!("\nServer Banner: {}\n", banner);
    let sql = "select * from HR.COUNTRIES ";
    let mut stmt = conn.statement(sql).build()?;
    let rows = stmt.query(&[])?;

    // Get the column names
    for info in rows.column_info() {
       print!("{} ", info.name())
    }
    println!("");

    // Display the resultset
    for row_result in rows {
        // print column values
        for (idx, val) in row_result?.sql_values().iter().enumerate() {
            if idx != 0 {
                print!(",");
            }
            print!("{}", val);
        }
        println!();
    }
    conn.close()?;

    println!("\nBye");
    Ok(())
}