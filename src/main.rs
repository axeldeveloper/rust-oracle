
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
use oracle::{Connection, Result, Version};

fn main() -> Result<()> {



    let key = "DB_URL";
    match env::var_os(key) {
        Some(val) => println!("{key}: {val:?}"),
        None => println!("{key} is not defined in the environment.")
    }



    // "oracle://user:secret@127.0.0.1/MY_DB"
    // let conn = Connection::connect("system", "oracle", svc)?;
    // Connect to a database.
    let conn = Connection::connect("system", "oracle", "localhost:49161/XE")?;

    //let sql = "select * from COUNTRIES where COUNTRY_ID = :1";


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