use diesel;
use diesel::prelude::*;
use super::errors::*;
use dotenv::dotenv;
use atm_machine::account::*;
use std::env;
use diesel::pg::PgConnection;
pub fn establish_connection() -> Result<PgConnection> {
    dotenv().chain_err(|| "While setting up dotenv")?;

    let database_url = env::var("DATABASE_URL").chain_err(|| "While getting env var DATABASE_URL")?;
    PgConnection::establish(&database_url).map_err(|e| e.into())
}

pub fn add_account(conn: &PgConnection, account: NewAccount) -> Result<Account> {
    diesel::insert(&account).into(schema::accounts::table)
        .execute(conn)
        .chain_err(|| "While trying to execute add")?;
    schema::accounts::table.find(account.id()).first(conn).map_err(|e| e.into())
}

pub fn all(conn: &PgConnection) -> Result<Vec<Account>> {
    schema::accounts::table.load(conn).map_err(|e| e.into())
}
