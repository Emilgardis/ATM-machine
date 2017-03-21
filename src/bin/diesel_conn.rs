use diesel;
use diesel::prelude::*;
use super::errors::*;
use dotenv::dotenv;
use atm_machine::account::*;
use atm_machine::schema;
use std::env;
use diesel::Connection;
pub fn establish_connection() -> Result<diesel::pg::PgConnection> {
    dotenv().chain_err(|| "While setting up dotenv")?;

    let database_url = env::var("DATABASE_URL").chain_err(|| "While getting env var DATABASE_URL")?;
    diesel::pg::PgConnection::establish(&database_url).map_err(|e| e.into())
}

pub fn add_account(conn: &diesel::pg::PgConnection, account: NewAccount) -> Result<()> {
    diesel::insert(&account).into(schema::accounts::table).execute(conn).chain_err(|| "While trying to execute add")?;
    Ok(())
    //unimplemented!()
}
