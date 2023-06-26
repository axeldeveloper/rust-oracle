use dotenv::dotenv;
use oracle::{Connection, Result};


#[derive(Debug)]
pub struct Job {
    pub matricula: i32,
    pub emp: i32,
}

pub fn list_jobs(emp: i32) -> Result<Vec<Job>>  {
    dotenv().ok();

    let dsn: String = std::env::var("DB_DSN").expect("DB_DSN must be set.");
    let user = std::env::var("DB_USER").expect("DB_USER must be set.");
    let pwd = std::env::var("DB_PWD").expect("DB_PWD must be set.");

    let db = Connection::connect(user, pwd, dsn)?;

    let sql: &str = "SELECT EMP_COD AS EMPRESA, CHAPA AS MATRICULA FROM REG_EMPREGOS
                    WHERE EMP_COD = :EMP_COD
                    AND ( SELECT MAX(DT_FIM_VIG)
                        FROM VALORES_DIVERSOS
                        WHERE TVD_COD = 901
                        AND DATA IS NULL) BETWEEN DT_ADMISSAO AND FIM_DOS_TEMPOS(LEAST(DT_DESAT, DT_RESCISAO))  ";
    //let rows = conn.query(sql, &[&30])?;
    let mut stmt = db.statement(sql).build()?;
    let rows = stmt.query(&[&emp])?;
    let mut list_job: Vec<Job> = Vec::new();
    for row_result in rows {
        let row = row_result?;
        let some = Job {matricula: row.get("MATRICULA")?, emp: row.get("EMPRESA")?};
        list_job.push(some);
    }

    println!( " {}", list_job.len());
    //for job in list_job {
    //    println!("Found job {:?}", job.matricula);
    //}
    db.close()?;
    Ok(list_job)
}

