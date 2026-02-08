use dotenv::dotenv;
use std::process;

pub fn init() {
    if let Err(err) = dotenv() {
        eprintln!("Erro lendo as variaveis de ambiente: {}", err);
        process::exit(1);
    }
}