use chrono::{DateTime, Utc};
use dotenv::dotenv;
use oracle::{Connection, Result};


#[derive(Debug)]
pub struct EmsAnexo {
    pub pfis_numero: i32,
    pub tdoc_cod: i32,
    pub dt_mov:  DateTime<Utc>,
    pub extensao: String,
    pub seq: i32,
    pub arquivo: Option<Vec<u8>>,
}

pub fn list_attachments( numero: i32) -> Result<Vec<EmsAnexo>>  {

    dotenv().ok();

    let dsn: String = std::env::var("DB_DSN").expect("DB_DSN must be set.");
    let user = std::env::var("DB_USER").expect("DB_USER must be set.");
    let pwd = std::env::var("DB_PWD").expect("DB_PWD must be set.");

    let db = Connection::connect(user, pwd, dsn)?;

    let sql: &str = "select * from EMS_DOC_ANEXOS where PFIS_NUMERO = :P  ";
    let mut stmt = db.statement(sql).build()?;
    let rows = stmt.query(&[&numero])?;
    let mut lists: Vec<EmsAnexo> = Vec::new();

    for row_result in rows {

        let row = row_result?;

        let some = EmsAnexo {
            pfis_numero: row.get("PFIS_NUMERO")?,
            tdoc_cod: row.get("TDOC_COD")?,
            dt_mov: row.get("DT_MOV")?,
            extensao: row.get("EXTENSAO")?,
            seq: row.get("SEQ")?,
            arquivo:row.get("ARQUIVO")?,
        };
        lists.push(some);
    }
    println!(" Total de Registros {}", lists.len());
    //for job in list_job {
    //    println!("Found job {:?}", job.matricula);
    //}
    db.close()?;
    Ok(lists)
}

