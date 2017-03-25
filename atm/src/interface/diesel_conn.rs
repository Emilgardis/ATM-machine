use diesel;
use diesel::prelude::*;
use dotenv::dotenv;
use account::{NewAccount, Account, Owner};
use transaction::{Transaction, NewTransaction};
use std::env;
use diesel::pg::PgConnection;
use error::*;
use interface::schemas::accounts::{dsl as acc_dsl, table as acc_table};
use interface::schemas::transactions::{table as trans_table, dsl as trans_dsl};

pub fn establish_connection() -> Result<PgConnection> {
    dotenv().chain_err(|| "While setting up dotenv")?;

    let database_url = env::var("DATABASE_URL").chain_err(|| "While getting env var DATABASE_URL")?;
    PgConnection::establish(&database_url).map_err::<Error, _>(|e| e.into()).chain_err(|| "Couldn't establish connection")
}

pub fn add_account(conn: &PgConnection, account: NewAccount) -> Result<Account> {
    diesel::insert(&account).into(acc_table)
        .execute(conn)
        .chain_err(|| "While trying to execute insert")?;
    acc_table.find(account.id()).first(conn).map_err::<Error, _>(|e| e.into()).chain_err(|| "Couldn't find newly added accout")
}

pub fn execute_transaction(conn: &PgConnection, ntrans: NewTransaction) -> Result<Transaction> {
    conn.transaction::<Transaction, Error, _>(|| {
            diesel::insert(&ntrans).into(trans_table)
                .execute(conn)
                .chain_err(|| "While trying to execute insert")?;
            // Do stuff on accounts if we do this.
            trans_table.order(trans_dsl::serial.desc()).first(conn).map_err(|e| e.into())
    })

}

pub fn all_accounts(conn: &PgConnection) -> Result<Vec<Account>> {
    acc_table.load(conn).map_err(|e| e.into())
}

pub fn all_transactions(conn: &PgConnection) -> Result<Vec<Transaction>> {
    trans_table.load(conn).map_err(|e| e.into())
}

pub fn accounts_by_owner(conn: &PgConnection, owner: &Owner) -> Result<Vec<Account>> {
    acc_table.filter(acc_dsl::owner_id.eq(owner.id())).get_results(conn).map_err(|e| e.into())
}

pub fn transactions_from(conn: &PgConnection, account: &Account) -> Result<Vec<Transaction>> {
    trans_table
        .filter(
            trans_dsl::sender.eq(account.id())
            .or(trans_dsl::recipient.eq(account.id())))
        .get_results(conn).map_err(|e| e.into())
}
