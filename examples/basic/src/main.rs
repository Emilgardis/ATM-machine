extern crate atm_lib;
#[macro_use]
extern crate error_chain;
extern crate diesel;
extern crate dotenv;
extern crate steel_cent;
// mod custom_views;
use atm_lib as atm;
use std::io::{self, Read};

use atm::account::{NewAccount, Owner};
use atm::currency::{Money, currency};
use atm::interface::diesel_conn;
use atm::error::*;

use diesel::prelude::*;

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("Input password of new user");
    let mut password_1 = String::from("hunter1");
    let mut password_2 = String::from("hunter2");
    let conn = diesel_conn::establish_connection().chain_err(|| "Failed to establish connection")?;
    println!("All users are: {:?}", diesel_conn::all_accounts(&conn));
    let owner_1 = Owner::new("Joe John");
    let owner_2 = Owner::new("Joe John");
    let funds_1 = Money::of_major(currency::SEK, 100);
    let funds_2 = Money::of_major(currency::SEK, 100);
    let nacc_1 =
        NewAccount::new(&owner_1, funds_1, password_1).chain_err(|| "Failed to create new account")?;
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
