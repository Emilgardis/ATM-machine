extern crate atm_lib;
#[macro_use]
extern crate error_chain;
extern crate diesel;
extern crate dotenv;
extern crate steel_cent;
extern crate chrono;
// mod custom_views;
use atm_lib as atm;

use atm::account::{NewAccount, Owner};
use atm::currency::{Money, currency};
use atm::interface::diesel_conn;
use atm::error::*;

use diesel::prelude::*;

use chrono::TimeZone;
quick_main!(run);

fn run() -> Result<()> {
    println!("Input password of new user");
    let password_1 = String::from("hunter1");
    let password_2 = String::from("hunter2");
    
    let nowner_1 = Owner::new("Joe John").set_email("junk@gmail.com");
    let nowner_2 = Owner::new("Jane Doe").set_date_of_birth(chrono::UTC.ymd(1985, 9, 8).and_hms(0,0,0));
    let funds_1 = Money::of_major(currency::SEK, 100);
    let funds_2 = Money::of_major(currency::SEK, 100);
   
    let conn = diesel_conn::establish_connection(None).chain_err(|| "Failed to establish connection")?;
    println!("All users are: {:?}", diesel_conn::all_accounts(&conn));


    let owner_1 = diesel_conn::add_owner(&conn, nowner_1).chain_err(|| "Failed to add new owner to database")?;
    let nacc_1 =
        NewAccount::new(&owner_1, funds_1, password_1).chain_err(|| "Failed to create new account")?;
    let owner_2 = diesel_conn::add_owner(&conn, nowner_2).chain_err(|| "Failed to add new owner to database")?;
    let nacc_2 =
        NewAccount::new(&owner_2, funds_2, password_2).chain_err(|| "Failed to create new account")?;
    let mut acc_1 = diesel_conn::add_account(&conn, nacc_1).chain_err(|| "Failed to add new account to database")?;
    let mut acc_2 = diesel_conn::add_account(&conn, nacc_2).chain_err(|| "Failed to add new account to database")?;
    acc_1.save(&conn)?;
    acc_2.save(&conn)?;
    let funds_transfer = Money::of_major(currency::SEK, 100);
    // Should it be a function on Account or from diesel_conn?
    acc_1.transfer(&conn, &mut acc_2, funds_transfer.clone())?;
    println!("Funds of {:?} is:", acc_1.id());
    for (c, a) in acc_1.funds(&conn)? {
        println!("{}", Money::of_minor(c, a));
    }
    println!("Funds of {:?} is:", acc_2.id());
    for (c, a) in acc_2.funds(&conn)? {
        println!("{}", Money::of_minor(c, a));
    }
    Ok(())
}
