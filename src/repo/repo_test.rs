use dotenv::dotenv;
use oracle::{Connection, Result};


pub fn display_db_info() -> Result<()> {
    // NOTE: Available on crate feature *clock* only.
    let db_dsn: String = std::env::var("DB_DSN").expect("DB_DSN must be set.");
    let db_user = std::env::var("DB_USER").expect("DB_USER must be set.");
    let db_pwd = std::env::var("DB_PWD").expect("DB_PWD must be set.");
    let heart_eyed_cat = '😻';

    // Connect to a database.
    let conn = Connection::connect(db_user, db_pwd, db_dsn)?;
    let (server_ver, banner) = conn.server_version()?;

    /* 
    println!(
        "* Database Server Oracle Version: {}",
        style(server_ver).magenta()
    );
    println!("* Server Banner: {}", style(banner).magenta());
    println!("* Conected: {}", heart_eyed_cat);
    */
    conn.close()?;

    Ok(())
}