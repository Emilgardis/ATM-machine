extern crate atm_machine;
extern crate uuid;
#[macro_use]
extern crate error_chain;

// mod custom_views;
use atm_machine as atm;
use atm::account::{StoredAccount, AccountStorage, Account, Owner};
use atm::currency::{Money, Currency};

use std::collections::HashMap;
use std::path::Path;
use std::io::{self, Read};


mod errors {
	use atm_machine as atm;
    error_chain! {
		links {
			Atm(atm::error::Error, atm::error::ErrorKind);
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
    //let mut password = String::new();
    //let stdin = io::stdin().read_to_string(&mut password);
    //println!("Got stdin");
    let owner_1 = Owner::new("Joe John");
    let funds_1 = Money::new(Currency::SEK, 100.0);
    //let acc_1 = StoredAccount::new(owner_1, funds_1, password);
    let mut storage = AccountStorage::from(env!("PWD"))?;
    //storage.store(acc_1)?;
    // FIXME: Load accounts
    let mut accounts = storage.get_accounts();
    println!("{:#?}", accounts);
	Ok(())
}


