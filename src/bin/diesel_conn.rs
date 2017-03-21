use diesel;
use super::errors::*;
use dotenv::dotenv;
use atm_machine::account::*;
use std::env;
use diesel::Connection;
pub fn establish_connection() -> Result<diesel::pg::PgConnection> {
    dotenv()?;

    let database_url = env::var("DATABASE_URL")?;
    diesel::pg::PgConnection::establish(&database_url).map_err(|e| e.into())
}

pub fn add_account(account: StoredAccount) -> Result<()> {
    diesel::insert(&account);
    unimplemented!()
}
