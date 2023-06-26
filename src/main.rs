extern crate dotenv;
use dotenv::dotenv;
use oracle::{Connection, Result};
use console::style;
use console::Emoji;

use crate::repo_job::Job;

use futures::executor::block_on;


#[path = "repo/repo_job.rs"] mod repo_job;
#[path = "repo/repo_anexo.rs"] mod repo_anexo;
#[path = "services/services_minio.rs"] mod service_minio;


static BARRA_STR: &'static str = "---------------------------------------------------------------------------------------------";

//const DATE_FORMAT_STR: &'static str = "%Y-%m-%d %H:%M:%S";
const DATE_FORMAT_STR: &'static str = "%d/%m/%Y %H:%M:%S";
#[derive(Hash)]
struct Config {
    user: String,
    pwd: String,
    dsn: String,
}


fn main() -> Result<()> {
    dotenv().ok();

    //service_minio::list_buckets();

    let future = service_minio::list_buckets(); // Nothing is printed
    block_on(future); //

    let dt_current = chrono::offset::Utc::now();

    println!("* {}", style(BARRA_STR).green());
    println!("* {}", style(BARRA_STR).green());
    println!("[* {}]", style("Importador do arquivo para MINIO").green());

    let _ = display_db_info();

    println!("* Data Inicio: {}", dt_current.format(DATE_FORMAT_STR).to_string());

    println!("* ESCOLHA UMA DAS OPÇÕES {}  ", style("(INFORMAR APENAS NUMEROS entre (1, 2, 3)) ").on_green());
    println!("1 -  {}  ", style("INFORMAR EMPRESA - IMPORTAR").yellow());
    println!("2 -  {}  ", style("RESETAR DOCUMENTOS DO STOREAGE").red());
    println!("3 -  {}  ", style("INFORMAR NUMERO - IMPORTAR POS PESSOA FISICA").green());
    println!("{}", style(BARRA_STR).green());
    println!("* {}",  style("Informe o código ?").magenta() );

    let input = read_string();
        match input.trim() {
            "1"=>option_import_emp(input)?,
            "2"=>option_reset_person(input)?,
            "3"=>option_import_person(input)?,
            _=>println!("Rest of the number"),

        }
    let dt_current_end = chrono::offset::Utc::now();
    println!("* Data Inicio: {}", dt_current_end.format(DATE_FORMAT_STR).to_string());
    println!("[4/4] {} Finishing processes Done!", Emoji("✨", ":-)"));
    Ok(())
}


fn display_db_info() -> Result<()> {
     // NOTE: Available on crate feature *clock* only.
     let db_dsn: String = std::env::var("DB_DSN").expect("DB_DSN must be set.");
     let db_user = std::env::var("DB_USER").expect("DB_USER must be set.");
     let db_pwd = std::env::var("DB_PWD").expect("DB_PWD must be set.");
     let heart_eyed_cat = '😻';
 
     // Connect to a database.
     let conn = Connection::connect(db_user, db_pwd, db_dsn)?;
     let (server_ver, banner) = conn.server_version()?;

    println!("* Database Server Oracle Version: {}",  style(server_ver).magenta() );
    println!("* Server Banner: {}",  style(banner).magenta() );
    println!("* Conected: {}", heart_eyed_cat);
    conn.close()?;

    Ok(())
}

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("erro ao carregar os parametros");
    input
}

fn convert_str_to_int(value: String) -> i32{
    let input: i32 = value
    .trim()
    .parse()
    .expect("Wanted a number");
    input
}

fn option_import_emp(input: String) -> Result<()> {
    println!("* Opção Selecionado {}",  style(input).green() );
    println!("* {}",  style("Informe o empresa valida ?").green() );
    let input_emp: String = read_string();
    let emp = convert_str_to_int(input_emp.clone());
    println!("* Empresa Informada {}",  style(emp.to_string() ).green() );
    let result: std::result::Result<Vec<Job>, oracle::Error> = repo_job::list_jobs(emp);
    println!("Resposta {:?}", result.unwrap().len() );
    /*for job in result.unwrap() {
        println!("Found job {:?}", job.matricula);
    }*/

    println!("[1/3] {} Finishing processes Done!", Emoji("✨", ":-)"));
    Ok(())
}


fn option_reset_person(input: String) -> Result<()> {
    println!("* Opção Selecionado {}",  style(input).green() );
    println!("* {}",  style("Informe o numero de pessoa fisica valido ?").green() );
    Ok(())
}

fn option_import_person(input: String) -> Result<()> {
    println!("* Opção Selecionado {}",  style(input).green() );
    println!("* {}",  style("Informe o numero de pessoa fisica valido ?").green() );
    let input_emp: String = read_string();
    let numero = convert_str_to_int(input_emp.clone());
    println!("* Empresa Informada {}",  style(numero.to_string() ).green() );
    let result: std::result::Result<Vec<repo_anexo::EmsAnexo>, oracle::Error> = repo_anexo::list_attachments(numero);
    println!("Resposta {:?}", result.unwrap().len() );
    /*for job in result.unwrap() {
        println!("Found job {:?}", job.matricula);
    }*/

    println!("[1/3] {} Finishing processes Done!", Emoji("✨", ":-)"));
    Ok(())
}




/* 


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

//for job in result.unwrap() {
    //    println!("Found job {:?}", job.matricula);
    //}
    
    match result {
        Ok(val) => {
            println!("Found job {:?}", val);
        },
        Err(err) => {
            println!("Found job {:?}", err);
        }
    }

#[derive(Debug)]
struct Job2 {
    matricula: i32,
    emp: i32,
    //data: Option<Vec<u8>>,
}


fn print_vars(){
    dotenv().ok();
    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}

fn read_number() -> Option<u8> {
    let input = read_string();
    u8::from_str(&input).ok()
}

*/
