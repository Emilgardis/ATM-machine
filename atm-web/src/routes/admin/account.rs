use pool;
use rocket_contrib::Template;
use rocket_contrib::UUID;
use atm_lib::account::Account;
use atm_lib::transaction;
use atm_lib::interface;
use atm_lib::currency;
use uuid::Uuid;
use super::AdminUser;

use error::*;
// Move to appropriate mod later
#[derive(Serialize)]
struct AccountsContext {
    accounts: Vec<Account>,
}


#[get("/admin-panel/accounts")]
fn show_accounts(_admin: AdminUser, conn: pool::Conn) -> Result<Template> {
    let context = AccountsContext {
        accounts: interface::diesel_conn::all_accounts(&conn)?,
    };
    Ok(Template::render("accounts_view", &context))
}

// Move to appropriate
#[derive(Serialize)]
struct AccountContext {
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

fn make_account_context(conn: &pool::Conn, account_id: &Uuid) -> Result<AccountContext> {
    let account = interface::diesel_conn::get_account(&conn, &account_id)?;
    let transactions = interface::diesel_conn::transactions_from(&conn, &account)?
        .into_iter().filter_map( // FIXME: Handle error properly
            |trans| 
                 Some(
                    TransactionOfUser {
                    serial: trans.serial(),
                    amount: format!("{}", match trans.amount_as_money() {
                        Ok(s) => s,
                        Err(e) => {
                                ::print_error_to_stderr(e);
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
    let context = AccountContext {
        funds: currency::convert_map_to_money(account.funds(&conn)?).into_iter().map(|elem| format!("{}", elem)).collect(),
        account: account,
        transactions: transactions,
    };
    Ok(context)
}

#[derive(FromForm)]
struct AccountQuery {
    pub id: UUID,
    pub opt: Option<String>
}
#[get("/admin-panel/accounts/account?<account_query>")]
fn show_account(_admin: AdminUser, conn: pool::Conn, account_query: AccountQuery) -> Result<Template> {
    let context = make_account_context(&conn, &account_query.id)?;
    println!("Passed in: {:?}", account_query.opt);
    Ok(Template::render("account_view", &context))
}
