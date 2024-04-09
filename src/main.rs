extern crate dotenv;


use console::style;
use console::Emoji;
use dotenv::dotenv;
use tokio;

mod repo;
use repo::repository::OracleRepository;
use repo::repo_anexo;
use repo::repo_emprego;
use crate::repo::repo_emprego::Job;

static BARRA_STR: &'static str =
    "---------------------------------------------------------------------------------------------";

const DATE_FORMAT_STR: &'static str = "%d/%m/%Y %H:%M:%S";


#[tokio::main]
async fn main()  {
    dotenv().ok();
    println!("* {}", style(BARRA_STR).green());
    let dt_current = chrono::offset::Utc::now();
    println!("* {} ", style("Importador COGESP").green());
    println!("* Data Inicio: {}", dt_current.format(DATE_FORMAT_STR).to_string());
    println!("* ESCOLHA UMA DAS OPÇÕES {}  ",style("(INFORMAR APENAS NUMEROS entre (1, 2, 3)) ").on_green());
    println!("0 -  {}  ", style("[PESSOAS_FISICAS] INFORMAR PESSOA - IMPORTAR").blue());
    println!("1 -  {}  ", style("[REG_EMPREGOS] INFORMAR EMPRESA - IMPORTAR").yellow());
    println!("2 -  {}  ", style("RESETAR DOCUMENTOS DO STOREAGE").red());
    println!("3 -  {}  ", style("[EMS_DOC_ANEXOS] INFORMAR NUMERO - RETORNA EMS_DOC_ANEXOS").green());
    println!("4 -  {}  ", style("[PESSOAS_FISICAS] INFORMAR CPF - RETORNAR PESSOA FISICA").blue());
    println!("{}", style(BARRA_STR).green());
    println!("* {}", style(" Menu => Informe o código ?").magenta());

    let input = read_string();

    match input.trim() {
        "0" => option_import_person_v0(input),
        "1" => option_import_emp(input),
        "2" => option_reset_person(input),
        "3" => option_import_person(input),
        "4" => option_cpf_person(input),
        _ => println!("Rest of the number"),
    }
    let dt_current_end = chrono::offset::Utc::now();
    println!("* Data Fim: {}",dt_current_end.format(DATE_FORMAT_STR).to_string());

    println!("[4/4] {} Finishing processes Done!", Emoji("✨", ":-)"));

    noa_faz_nada().await

}


async fn noa_faz_nada(){
    println!("* fim do Processo {}", style(" - ").green());
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

fn option_import_person_v0(input: String){

    println!("* Opção Selecionado {}", style(input).green());
    println!("* {}", style("Informe o pfis numero  ?").green());

    let input_emp: String = read_string();
    let numero = convert_str_to_int(input_emp.clone());

    match OracleRepository::new("connection_string") {
        Ok(repository) => {
            // Crie uma variável para armazenar o resultado da consulta
           // ID do dado a ser buscado
            match repository.get_data_by_id(numero) {
                Ok(data) => {
                    match data {
                        Some(value) => println!("Data found: {}", value),
                        None => println!("Data not found."),
                    }
                }
                Err(err) => {
                    eprintln!("Erro ao buscar dados: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Erro ao conectar ao Oracle: {}", err);
        }
    }
}


fn option_cpf_person(input: String){
    println!("* Opção Selecionado {}", style(input).green());
    println!("* {}", style("Informe o cpf valida ?").green());
    let input_emp: String = read_string();
    let cpf = convert_str_to_int(input_emp.clone());
    //let cpf = input_emp.clone();
    println!("* CPF Informada {}", style(cpf.to_string()).green());

    match OracleRepository::new("connection_string") {
        Ok(repository) => {
            // Crie uma variável para armazenar o resultado da consulta
            // ID do dado a ser buscado
            match repository.ger_cpf(cpf) {
                Ok(data) => {
                    match data {
                        Some(value) => println!("Data found: {}", value.name),
                        None => println!("Data not found."),
                    }
                }
                Err(err) => {
                    eprintln!("Erro ao buscar dados: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Erro ao conectar ao Oracle: {}", err);
        }
    }

}

fn option_import_emp(input: String)  {
    println!("* Opção Selecionado {}", style(input).green());
    println!("* {}", style("Informe o empresa valida ?").green());
    let input_emp: String = read_string();
    let emp = convert_str_to_int(input_emp.clone());
    println!("* Empresa Informada {}", style(emp.to_string()).green());

    let result: Result<Vec<Job>, oracle::Error> = repo_emprego::list_jobs(emp);

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

    println!("* EMS_DOC_ANEXOS Informada {}", style(numero.to_string()).green());

    let result: Result<Vec<repo_anexo::EmsAnexo>, oracle::Error> = repo_anexo::list_attachments(numero);

    println!("Resposta {:?}", result.unwrap().len());
    /*for job in result.unwrap() {
        println!("Found job {:?}", job.matricula);
    }*/

    println!("[1/3] {} Finishing processes Done!", Emoji("✨", ":-)"));
}
