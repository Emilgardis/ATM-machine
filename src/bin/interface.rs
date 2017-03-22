extern crate atm_machine;
extern crate uuid;
#[macro_use]
extern crate error_chain;
extern crate cursive;
extern crate steel_cent;
extern crate diesel;
extern crate dotenv;
// mod custom_views;
use atm_machine as atm;
use atm::account::{NewAccount, Owner};
use steel_cent::{Money, currency};
use std::collections::HashMap;
use std::path::Path;
use std::io::{self, Read};
mod diesel_conn;
//mod custom_views;

pub mod errors {
	use atm_machine as atm;
    use dotenv;
    use diesel;

    error_chain! {
		links {
			Atm(atm::error::Error, atm::error::ErrorKind);
		}

        foreign_links {
            DotEnv(dotenv::DotenvError);
            VarErr(::std::env::VarError);
            DieselConn(diesel::ConnectionError);
            Diesel(diesel::result::Error);
        }
	}
}

use errors::*;
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
    let mut password = String::new();
    let stdin = io::stdin().read_to_string(&mut password);
    println!("Got stdin");
    let conn = diesel_conn::establish_connection().chain_err(|| "Failed to establish connection")?;
    println!("All users are: {:?}", diesel_conn::all(&conn));
    let owner_1 = Owner::new("Joe John");
    let funds_1 = Money::of_major(currency::SEK, 100);
    let acc_1 = NewAccount::new(&owner_1, funds_1, password).chain_err(|| "Failed to create new account")?;
    let mut acc = diesel_conn::add_account(&conn, acc_1).chain_err(|| "Failed to add new account to database")?;
    println!("{:#?}", acc);
	acc.save(&conn)?;
	Ok(())
}


