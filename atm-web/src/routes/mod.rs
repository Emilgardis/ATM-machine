
use pool;
use rocket_contrib::Template;
use rocket_contrib::UUID;
use atm_lib::account::Account;
use atm_lib::transaction;
use atm_lib::interface;
use atm_lib::currency;
use uuid::Uuid;

use error::*;
// Move to appropriate mod later
#[derive(Serialize)]
pub struct TemplateContext {
    accounts: Vec<Account>,
}
#[get("/admin-panel")]
pub fn admin_show_accounts(conn: pool::Conn) -> Result<Template> {
    let context = TemplateContext {
        accounts: interface::diesel_conn::all_accounts(&conn)?,
    };
    Ok(Template::render("accounts_view", &context))
}

// Move to appropriate
#[derive(Serialize)]
struct AccountView {
    funds: Vec<String>,
    account: Account,
    transactions: Vec<TransactionOfUser>
}

#[derive(Serialize)]
struct TransactionOfUser {
    serial: i32,
    amount: String,
    trans_type: transaction::TransactionType,
    sender: Uuid,
    recipient: Option<Uuid>,
}

#[derive(FromForm)]
pub struct AccountQuery {
    pub id: UUID,
}
#[get("/admin-panel/accounts/account?<account_query>")]
pub fn admin_show_account(conn: pool::Conn, account_query: AccountQuery) -> Result<Template> {
    let account = interface::diesel_conn::get_account(&conn, &account_query.id)?;
    let transactions = interface::diesel_conn::transactions_from(&conn, &account)?
        .into_iter().filter_map( // FIXME: Handle error properly
            |trans| 
                 Some(
                    TransactionOfUser {
                    serial: trans.serial(),
                    amount: format!("{}", match trans.amount_as_money() {
                        Ok(s) => s,
                        Err(e) => {
                                super::print_error_to_stderr(e);
								return None;
							},
                        }
                    ),
                    trans_type: trans.trans_type,
                    sender: trans.sender,
                    recipient: trans.recipient,
                 })
            )
        .collect();
    let context = AccountView {
        funds: currency::convert_map_to_money(account.funds(&conn)?).into_iter().map(|elem| format!("{}", elem)).collect(),
        account: account,
        transactions: transactions,
    };
    Ok(Template::render("account_view", &context))
}
