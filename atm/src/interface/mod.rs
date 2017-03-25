pub mod diesel_conn;
pub mod schemas;

use error::*;
use std::env;
use dotenv::dotenv;

pub fn get_database_url() -> Result<String> {  
    dotenv().chain_err(|| "While setting up dotenv")?;
    env::var("DATABASE_URL").chain_err(|| "While getting env var DATABASE_URL")
}
