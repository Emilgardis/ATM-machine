pub mod diesel_conn;
pub mod schemas;

use dotenv::dotenv;
use error::*;
use std::env;

pub fn get_database_url() -> Result<String> {
    dotenv().chain_err(|| "While setting up dotenv")?;
    env::var("DATABASE_URL").chain_err(|| "While getting env var DATABASE_URL")
}
