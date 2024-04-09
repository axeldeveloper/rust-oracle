use std::env;
use console::style;
use dotenv::dotenv;
use oracle::Connection;


pub struct Pessoa {
    pub social_name: Option<String>,
    pub name: String,
    pub cpf: i32,
    pub number: i32,
}



pub struct OracleRepository {
    connection: Connection,
}

impl OracleRepository {
    pub fn new(connection_string: &str) -> Result<Self, oracle::Error> {
        dotenv().ok();
        println!("\nDatabase Server Version: {}", connection_string);
        // Load the .env file
        let db_url = env::var("DB_URL").expect("You've not set the DB_URL");
        let db_user = env::var("DB_USER").expect("You've not set the DB_USER");
        let db_pwd = env::var("DB_PWD").expect("You've not set the DB_PWD");

        // Connect to a database.
        let connection = Connection::connect(db_user, db_pwd, db_url)?;

        // let connection = Connection::connect(connection_string, oracle::Credentials::default())?;

        Ok(Self { connection })
    }

    pub fn ger_cpf(&self, cpf: i32) -> Result<Option<Pessoa>, oracle::Error> {
        let sql: &str = " select * from PESSOAS_FISICAS WHERE CPF = :cpf ";
        let mut stmt = self.connection.statement(sql).build()?;
        let rows = stmt.query(&[&cpf])?;
        for row_result in rows {
            // println!("* CPF Informada {:?}",  row_result  );
            let row = row_result?;
            let social: Option<String> = row.get("NOME_SOCIAL")?;
            let nome: String = row.get("NOME")?;
            let cpf: i32 = row.get("CPF")?;
            let number = row.get("NUMERO")?;
            let pessoa = Pessoa {
                social_name: social,
                name: nome,
                cpf: cpf,
                number: number,
            };
            return Ok(Some(pessoa));
        }
        Ok(None)
    }

    pub fn get_data_by_id(&self, id: i32) -> Result<Option<String>, oracle::Error> {

        println!("* {} ", style("Repositorio recebendo valor ?").green());

        let sql: &str = "SELECT NUMERO FROM PESSOAS_FISICAS WHERE numero = :1";

        // let mut stmt = conn.statement("insert into emp(empno, ename) values (:id, :name)").build()?

        let mut statement = self.connection.statement(sql).build()?;
        let rows = statement.query(&[&id])?;
        let mut result: Option<String> = None;

        for row_result in rows {
            let row = row_result?;
            let data: String = row.get(0)?;
            result = Some(data);
        }

        Ok(result)
    }

    // Adicione métodos para realizar operações no banco de dados Oracle, como insert, update, delete, etc.
}
