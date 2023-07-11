extern crate dotenv;

use crate::repo_job::Job;
use console::style;
use console::Emoji;
// use std::result::Result;
// use std::error::Error;
use dotenv::dotenv;
use s3::creds::Credentials;
use s3::error::S3Error;
use tokio;
use s3::Bucket;
//use s3::creds::Credentials;


#[path = "repo/repo_anexo.rs"] mod repo_anexo;
#[path = "repo/repo_job.rs"] mod repo_job;
#[path = "repo/repo_test.rs"] mod repo_test;
#[path = "services/services_minio.rs"] mod services_minio;
#[path = "services/services_s3.rs"] mod services_s3;
#[path = "services/services_rusoto_s3.rs"] mod services_rusoto_s3;


static BARRA_STR: &'static str =
    "---------------------------------------------------------------------------------------------";

const DATE_FORMAT_STR: &'static str = "%d/%m/%Y %H:%M:%S";


#[tokio::main]
async fn main() -> Result<(), S3Error> {
    println!("* {}", style(BARRA_STR).green());
    
    services_minio::async_buckets_print().await;

    //let ts = services_s3::test_list();

    //let tss = services_rusoto_s3::test_list().await;

    //services_rusoto_s3::get_object("/dados-pesoais/04869305135/27_1.pdf");

    //services_rusoto_s3::bucket_obj_bytes("dados-pesoais".to_string(), "04869305135/27_1.pdf".to_string()).await;

    Ok(())
}



//#[tokio::main]
async fn main2()  {
    dotenv().ok();

    let dt_current = chrono::offset::Utc::now();
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
    println!("* Data Fim: {}",dt_current_end.format(DATE_FORMAT_STR).to_string());
    println!("[4/4] {} Finishing processes Done!", Emoji("✨", ":-)"));

    noa_faz_nada().await

}


async fn noa_faz_nada(){
    services_minio::buckets_print().await;

    services_minio::async_buckets_print().await;
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
    println!("* {}", style("Informe o numero de pessoa fisica valido ?").green() );
    let input_numero: String = read_string();
    let numero = convert_str_to_int(input_numero.clone());
    println!("* Pessoa Informada {}", style(numero.to_string()).green());
}

fn option_import_person(input: String)  {
    println!("* Opção Selecionado {}", style(input).green());
    println!("* {}",style("Informe o numero de pessoa fisica valido ?").green());
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
