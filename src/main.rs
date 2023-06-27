extern crate dotenv;

use crate::repo_job::Job;
use console::style;
use console::Emoji;
use std::result::Result;
use std::error::Error;
use dotenv::dotenv;
//use std::env;
use tokio;

#[path = "repo/repo_anexo.rs"] mod repo_anexo;
#[path = "repo/repo_job.rs"] mod repo_job;
#[path = "repo/repo_test.rs"] mod repo_test;
#[path = "services/services_minio.rs"] mod services_minio;


static BARRA_STR: &'static str =
    "---------------------------------------------------------------------------------------------";

const DATE_FORMAT_STR: &'static str = "%d/%m/%Y %H:%M:%S";


#[tokio::main]
async fn main2()  {
    dotenv().ok();

    let dt_current = chrono::offset::Utc::now();
    //services_minio::async_buckets_print().await;
    println!("* {}", style(BARRA_STR).green());
    println!("* {}", style(BARRA_STR).green());
    println!("[* {}]", style("Importador do arquivo para MINIO").green());
    println!("* Data Inicio: {}", dt_current.format(DATE_FORMAT_STR).to_string());
    println!("* ESCOLHA UMA DAS OPÇÕES {}  ",style("(INFORMAR APENAS NUMEROS entre (1, 2, 3)) ").on_green());
    println!("1 -  {}  ", style("INFORMAR EMPRESA - IMPORTAR").yellow());
    println!("2 -  {}  ", style("RESETAR DOCUMENTOS DO STOREAGE").red());
    println!("3 -  {}  ", style("INFORMAR NUMERO - IMPORTARPESSOA FISICA").green());
    println!("{}", style(BARRA_STR).green());
    println!("* {}", style("Informe o código ?").magenta());

    let input = read_string();
    match input.trim() {
        "1" => option_import_emp(input),
        "2" => option_reset_person(input),
        "3" => option_import_person(input),
        _ => println!("Rest of the number"),
    }
    let dt_current_end = chrono::offset::Utc::now();
    println!("* Data Inicio: {}",dt_current_end.format(DATE_FORMAT_STR).to_string());
    println!("[4/4] {} Finishing processes Done!", Emoji("✨", ":-)"));

}

fn main()  {
    let dt_current = chrono::offset::Utc::now();
    println!("* {}", style("Informe o empresa valida ?").green());
    println!("* Data Inicio: {}", dt_current.format(DATE_FORMAT_STR).to_string());
    let emp = 25;
    println!("* Empresa Informada {}", style(emp.to_string()).green());
    let result: std::result::Result<Vec<Job>, oracle::Error> = repo_job::list_jobs(emp);
    println!("Resposta {:?}", result.unwrap().len());
    let dt_current_end = chrono::offset::Utc::now();
    println!("* Data Inicio: {}",dt_current_end.format(DATE_FORMAT_STR).to_string());
    /*for job in result.unwrap() {
        println!("Found job {:?}", job.matricula);
    }*/

}


fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("erro ao carregar os parametros");
    input
}

fn convert_str_to_int(value: String) -> i32 {
    let input: i32 = value.trim().parse().expect("Wanted a number");
    input
}

fn option_import_emp(input: String)  {
    println!("* Opção Selecionado {}", style(input).green());
    println!("* {}", style("Informe o empresa valida ?").green());
    let input_emp: String = read_string();
    let emp = convert_str_to_int(input_emp.clone());
    println!("* Empresa Informada {}", style(emp.to_string()).green());
    let result: std::result::Result<Vec<Job>, oracle::Error> = repo_job::list_jobs(emp);
    println!("Resposta {:?}", result.unwrap().len());
    /*for job in result.unwrap() {
        println!("Found job {:?}", job.matricula);
    }*/

    println!("[1/3] {} Finishing processes Done!", Emoji("✨", ":-)"));
}

fn option_reset_person(input: String)  {
    println!("* Opção Selecionado {}", style(input).green());
    println!(
        "* {}",
        style("Informe o numero de pessoa fisica valido ?").green()
    );
    //Ok(())
}

fn option_import_person(input: String)  {
    println!("* Opção Selecionado {}", style(input).green());
    println!(
        "* {}",
        style("Informe o numero de pessoa fisica valido ?").green()
    );
    let input_emp: String = read_string();
    let numero = convert_str_to_int(input_emp.clone());
    println!("* Empresa Informada {}", style(numero.to_string()).green());
    let result: std::result::Result<Vec<repo_anexo::EmsAnexo>, oracle::Error> =
        repo_anexo::list_attachments(numero);
    println!("Resposta {:?}", result.unwrap().len());
    /*for job in result.unwrap() {
        println!("Found job {:?}", job.matricula);
    }*/

    println!("[1/3] {} Finishing processes Done!", Emoji("✨", ":-)"));
}
