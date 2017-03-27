use atm_lib::account;
use atm_lib::currency;
use atm_lib::interface;
use atm_lib::transaction;
use chrono;
use num;

use pool;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Failure, Flash, Redirect};
use rocket_contrib::UUID;
use super::AdminUser;
pub fn pg_epoch() -> chrono::NaiveDateTime {
    chrono::NaiveDate::from_ymd(2000, 1, 1).and_hms(0, 0, 0)
}
pub fn create_datetime_from_pg(offset: i64) -> chrono::DateTime<chrono::UTC> {
    chrono::DateTime::from_utc(pg_epoch()
                                   .checked_add_signed(chrono::Duration::microseconds(offset))
                                   .unwrap(),
                               chrono::UTC)
}

#[derive(FromForm)]
pub struct NewTransactionFromForm {
    // TODO: Better name please.
    pub sender: UUID,
    pub recipient: Option<UUID>,
    pub trans_type: i32,
    pub amount: i64,
    pub currency: String,
    pub date: Option<i64>, // This should probably be a string, but for now do the offset
}

impl From<NewTransactionFromForm> for transaction::NewTransaction {
    fn from(trans: NewTransactionFromForm) -> transaction::NewTransaction {
        transaction::NewTransaction {
            sender: trans.sender.into_inner(),
            recipient: trans.recipient.map(|id| id.into_inner()),
            trans_type: num::FromPrimitive::from_i32(trans.trans_type).unwrap(), // Must crash :/
            amount: trans.amount,
            currency: trans.currency,
            date: trans.date
                .map(|offset| create_datetime_from_pg(offset))
                .unwrap_or(chrono::UTC::now()), // Not recommended but better than crash :-)
        }
    }
}

#[post("/admin-panel/post-transaction", data="<transaction>")]
pub fn post_transaction(_admin: AdminUser,
                        conn: pool::Conn,
                        transaction: Form<NewTransactionFromForm>)
                        -> Result<Redirect, Flash<Redirect>> {

    unimplemented!()
}
