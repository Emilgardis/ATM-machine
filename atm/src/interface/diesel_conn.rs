use account::{Account, NewAccount, NewOwner, Owner};
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use error::*;
use interface::schemas::accounts::{dsl as acc_dsl, table as acc_table};
use interface::schemas::owners::{dsl as owner_dsl, table as owner_table};
use interface::schemas::transactions::{dsl as trans_dsl, table as trans_table};
use transaction::{NewTransaction, Transaction};
use uuid;


pub fn establish_connection<S>(db_url: S) -> Result<PgConnection>
    where S: Into<Option<String>>
{
    let database_url = match db_url.into() {
        Some(url) => url.to_string(),
        None => super::get_database_url()?,
    };
    PgConnection::establish(&database_url)
        .map_err::<Error, _>(|e| e.into())
        .chain_err(|| "Couldn't establish connection")
}

pub fn add_account(conn: &PgConnection, account: NewAccount) -> Result<Account> {
    // FIXME: Do check on owner id if it exists.
    diesel::insert(&account).into(acc_table)
        .execute(conn)
        .chain_err(|| "While trying to execute insert")?;
    // Do we do this or is order guaranteed?? i.e get last entry.
    get_account(conn, account.id()).chain_err(|| "Couldn't find newly added account.")
}

pub fn add_owner(conn: &PgConnection, owner: NewOwner) -> Result<Owner> {
    diesel::insert(&owner).into(owner_table)
        .execute(conn)
        .chain_err(|| "While trying to execute insert")?;
    get_owner(conn, owner.id()).chain_err(|| "Couldn't find newly added owner")
}

pub fn get_account(conn: &PgConnection, account_id: &uuid::Uuid) -> Result<Account> {
    acc_table.find(account_id)
        .get_result(conn)
        .map_err::<Error, _>(|e| e.into())
        .chain_err(|| format!("Couldn't find account with id {:?}", account_id))
}

pub fn get_owner(conn: &PgConnection, owner_id: &uuid::Uuid) -> Result<Owner> {
    owner_table.find(owner_id)
        .get_result(conn)
        .map_err::<Error, _>(|e| e.into())
        .chain_err(|| format!("Couldn't find owner with id {:?}", owner_id))
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
    trans_table.filter(trans_dsl::sender.eq(account.id())
            .or(trans_dsl::recipient.eq(account.id())))
        .get_results(conn)
        .map_err(|e| e.into())
}
