//! All the account and bank/money functions, handles things.
use std::iter;
use std::hash;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

use rand::{OsRng, Rng}; 
#[cfg(not(debug_assertions))]
use argon2;

use chrono;
use uuid::Uuid;

use currency::Money;
use steel_cent;
use transaction::{Transaction, PendingTransaction};
use error::*;
use diesel::prelude::*;
use diesel;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable)]
pub struct Owner {
    /// An end-user
    #[serde(rename="OwnerId")]
    pub id: Uuid, // Id of owner.
    #[serde(rename="Name")]
    pub name: String,
}

impl Owner {
    pub fn new<T: AsRef<str>>(name: T) -> Owner {
        Owner {
            id: Uuid::new_v4(),
            name: name.as_ref().into(),
        }
    }
}

use schema::accounts;

#[derive(Debug, Insertable)]
#[table_name="accounts"]
pub struct NewAccount {
    id: Uuid,
    owner_id: Uuid,
    pw_hash: String,
    pub created: chrono::DateTime<chrono::UTC>,
    pub last_updated: chrono::DateTime<chrono::UTC>,
}


impl NewAccount {
    // FIXME: Should we take account? Or just borrow?
    pub fn new<T: AsRef<str>, F: Into<Option<Money>>>(owner: &Owner, funds: F, password: T) -> Result<NewAccount> {
        #[cfg(all(debug_assertions, not(test)))] // Disable this print on test, but enable otherwise when in debug
        println!("WARNING! Please note that currently all accounts are using plaintext passwords\n\
                  Build in --release to use scrypt");
        let id = Uuid::new_v4();

         #[cfg(not(debug_assertions))]
        let pw_hash: String = {
            let mut rng = OsRng::new()?;

            let salt: Vec<u8> = rng.gen_iter::<u8>().take(16).collect();
            let pw = password.as_ref().as_bytes();
            let config = argon2::Config::new();
            argon2::hash_encoded(pw, salt, &config)?

        };
        #[cfg(debug_assertions)]
        let pw_hash: String = String::from(password.as_ref());

        let created = chrono::UTC::now();
        Ok(
            NewAccount {
                id: id,
                pw_hash: pw_hash,
                owner_id: owner.id,
                created: created,
                last_updated: created,
            }
        )
    }
}
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct Account {
    /// A stored Account.
    //pub account: Account,
    // pub path: Path,
    pub owner_id: Uuid,
    pw_hash: String,
    #[primary_key]
    pub id: Uuid, // Not the same as owner.id, used to track transactions.
    pub created: chrono::DateTime<chrono::UTC>,
    pub last_updated: chrono::DateTime<chrono::UTC>,
}

impl hash::Hash for Account {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        self.id.hash(state)
    }
}

impl Account {
    pub fn save(&mut self, conn: diesel::pg::PgConnection) -> Result<()> {
        self.save_changes::<Account>(&conn).chain_err(|| "While saving")?;
        Ok(())
    }

    pub fn open<T: AsRef<str>>(&mut self, password: T) -> Result<()> {
        #[cfg(not(debug_assertions))]
        let password_matches = 
            argon2::verify_encoded(self.pw_hash, password.as_ref().as_bytes()).chain_err(|| format!("Failed to check password for {}.", self.owner));
        #[cfg(debug_assertions)]
        let password_matches = {
            password.as_ref() == self.pw_hash
        };

        if password_matches {
            // return Ok(&mut self.account); FIXME: Make account.transactions locked behind crypto.
            return Ok(())
        }
        bail!("Password didn't match!")
    }

    pub fn funds(&self) -> HashMap<steel_cent::currency::Currency, i64> {
        //let mut map = HashMap::new();
        //for trans in &self.account.transactions {
        //    if let Some(money) = trans.get_change(&self.id) {
        //        *map.entry(money.currency).or_insert(0) += money.minor_amount()
        //    }
        //}
        //map
        //for trans in &self.account.transactions {
        //    if let Some((curr, amount)) = trans.get_change(&self.id){
        //        *map.entry(curr).or_insert(0.0) += amount;
        //    }
        //}
        // map.into_iter().map(|(curr, amount)| Money::new(curr, amount)).collect()
        //map
        unimplemented!()
    } 
}
#[cfg(test)]
mod account_tests {
    use super::*;
    use currency::{currency as scc, Money};
    use super::super::uuid::Uuid;
    use transaction::Transaction;
    
    #[test]
    fn secure_account_and_decrypt() {
        let owner = Owner::new("John Doe");
        let mut sec_account = StoredAccount::new(owner, Money::of_major(scc::SEK, 100), "hunter1").unwrap();

        println!("{:#?}", sec_account);
        sec_account.open("hunter1").unwrap();
    }

    #[test]
    #[should_panic]
    fn open_with_wrong_password() {
        let owner = Owner::new("John Doe");
        let mut sec_account = NewAccount::new(owner, None, "hunter1").unwrap();

        //println!("{:?}", sec_account);
        let open_account = sec_account.open("wrongpass").expect("Fail means success");
    }

    #[test]
    fn check_funds() { 
        use std::collections::HashMap;
        let owner = Owner::new("John Doe");
        let mut sec_account = NewAccount::new(owner, Money::of_major(scc::SEK, 100), "hunter1").unwrap();
        let other_owner = Owner::new("Jane Doe");
        let mut other_sec_account = StoredAccount::new(other_owner, Money::of_major(scc::JPY, 100), "password").unwrap();
        sec_account.account.transactions.push(Transaction::deposit(
            sec_account.id, Money::of_major(scc::SEK, 100)));
        sec_account.account.transactions.push(Transaction::withdrawal(
            sec_account.id, Money::of_major(scc::ISK, 40)));
        sec_account.account.transactions.push(Transaction::payment(
            other_sec_account.id, sec_account.id, Money::of_major(scc::USD, 30)));
        other_sec_account.account.transactions.push(Transaction::payment(
            other_sec_account.id, sec_account.id, Money::of_major(scc::USD, 30)));
        let funds = sec_account.funds();
        let checks = {
            let mut checks = HashMap::new();
            checks.insert(scc::SEK, 20000);
            checks.insert(scc::ISK, -40);
            checks.insert(scc::USD, 3000);
            checks
        };

        assert_eq!(&funds.len(), &checks.len());
        for (curr, amount) in funds.iter() {
            assert_eq!(amount, checks.get(curr).unwrap());
        }
        // FIXME: Add check for other account
    }
}
