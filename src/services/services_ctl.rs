

/*

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





fn list_users(emp: i32, conn: &Connection) -> Result<Vec<Job>>  {
    let sql: &str = "SELECT EMP_COD AS EMPRESA, CHAPA AS MATRICULA FROM REG_EMPREGOS
                    WHERE EMP_COD = :EMP_COD
                    AND ( SELECT MAX(DT_FIM_VIG)
                        FROM VALORES_DIVERSOS
                        WHERE TVD_COD = 901
                        AND DATA IS NULL) BETWEEN DT_ADMISSAO AND FIM_DOS_TEMPOS(LEAST(DT_DESAT, DT_RESCISAO))  ";
    //let rows = conn.query(sql, &[&30])?;
    let mut stmt = conn.statement(sql).build()?;

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

    Ok(list_job)
}



fn display_rows(conn: &Connection) -> Result<()> {
    let sql: &str = "select * from PESSOAS_FISICAS where CPF is not null and rownum <= 10  ";
    let mut stmt = conn.statement(sql).build()?;
    let rows = stmt.query(&[])?;

    // Get the column names
    //for info in rows.column_info() {
    //    print!("{} ", info.name())
    //}

    for row_result in rows {
        let row = row_result?;
        // get a column value by position (0-based)
        // get a column by name (case-insensitive)
        let ename: f32  = row.get(0)?;
        let sal: String = row.get("NOME")?;
        let cpf: String = row.get("CPF")?;

        println!( " {:10}| {:60}| {:>14}|",ename,sal,cpf);
    }
    println!("{}", style(BARRA_STR).green());

    Ok(())
}

*/