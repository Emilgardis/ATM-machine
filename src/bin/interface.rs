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
use atm::account::{StoredAccount, Account, Owner};
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
            Diesel(diesel::ConnectionError);
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
    let mut password = String::new();
    let stdin = io::stdin().read_to_string(&mut password);
    //println!("Got stdin");
    let owner_1 = Owner::new("Joe John");
    let funds_1 = Money::of_major(currency::SEK, 100);
    let acc_1 = StoredAccount::new(owner_1, funds_1, password)?;
    diesel_conn::establish_connection()?;
    diesel_conn::add_account(acc_1)?;
    //println!("{:#?}", acc_1);
	Ok(())
}


