//! All the account and bank/money functions, handles things.

use argon2;

use chrono;

use currency::Money;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use error::*;
use interface::diesel_conn;
use rand::{OsRng, Rng};
use std::collections::HashMap;
use std::hash;
use steel_cent;
use uuid::Uuid;

mod owner;
use interface::schemas::accounts;
pub use self::owner::{NewOwner, Owner};
use transaction::{NewTransaction, Transaction};

#[derive(Debug, Insertable)]
#[table_name="accounts"]
pub struct NewAccount {
    id: Uuid,
    owner_id: Uuid,
    pub created: chrono::DateTime<chrono::UTC>,
    pub last_updated: chrono::DateTime<chrono::UTC>,
    pw_hash: String,
}


impl NewAccount {
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn new<T: AsRef<str>, F: Into<Option<Money>>>(owner: &Owner,
                                                      funds: F,
                                                      password: T)
                                                      -> Result<NewAccount> {
        let id = Uuid::new_v4();
        let pw_hash: String = {
            let mut rng = OsRng::new().chain_err(|| "While making random generator")?;

            let salt: Vec<u8> = rng.gen_iter::<u8>().take(16).collect();
            let pw = password.as_ref().as_bytes();
            let config = argon2::Config::default();
            argon2::hash_encoded(pw, salt.as_slice(), &config)
                .chain_err(|| "While making encoded hash of password")?
                .to_owned()

        };
        //#[cfg(debug_assertions)]
        //#let pw_hash: String = String::from(password.as_ref());

        let created = chrono::UTC::now();
        Ok(NewAccount {
            id: id,
            pw_hash: pw_hash,
            owner_id: owner.id(),
            created: created,
            last_updated: created,
        })
    }
}
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
pub struct Account {
    /// A stored Account.
    // pub account: Account,
    // pub path: Path,
    #[primary_key]
    pub id: Uuid, // Not the same as owner.id, used to track transactions.
    pub owner_id: Uuid,
    pub created: chrono::DateTime<chrono::UTC>,
    pub last_updated: chrono::DateTime<chrono::UTC>,
    pw_hash: String,
}

impl hash::Hash for Account {
    fn hash<H>(&self, state: &mut H)
        where H: hash::Hasher
    {
        self.id.hash(state)
    }
}

impl Account {
    pub fn save(&mut self, conn: &diesel::pg::PgConnection) -> Result<()> {
        self.save_changes::<Account>(conn).chain_err(|| "While saving")?;
        Ok(())
    }

    pub fn open<T: AsRef<str>>(&mut self, password: T) -> Result<()> {
        //#[cfg(not(debug_assertions))]
        let password_matches = argon2::verify_encoded(
            self.pw_hash.as_str(), password.as_ref().as_bytes()
            )
            .chain_err(|| format!("Failed to check password for {}.", self.id()))?;
        //#[cfg(debug_assertions)]
        //#let password_matches = {
        //# password.as_ref() == self.pw_hash
        //#;

        if password_matches {
            return Ok(());
        }
        bail!("Password didn't match!")
    }

    pub fn funds(&self,
                 conn: &PgConnection)
                 -> Result<HashMap<steel_cent::currency::Currency, i64>> {
        // TODO: Should be stored as a vec of all their specific transactions, and maybe
        // optimised so that we neer really do 20+ searches.
        let mut map = HashMap::new();
        // Carrier on for and if let is wierd...
        for trans in diesel_conn::transactions_from(conn, self).chain_err(|| {
                format!("While trying to get transactions affecting account {:?}",
                        self.id())
            })? {
            if let Some(money) = trans.get_change(&self.id)
                .chain_err(|| {
                    format!("While calculating value of transaction id: {}",
                            trans.serial())
                })? {
                *map.entry(money.currency).or_insert(0) += money.minor_amount()
            }
        }
        Ok(map)
        // for trans in &self.account.transactions {
        //    if let Some((curr, amount)) = trans.get_change(&self.id){
        //        *map.entry(curr).or_insert(0.0) += amount;
        //    }
        //
        // map.into_iter().map(|(curr, amount)| Money::new(curr, amount)).collect()
        // map
    }

    pub fn transfer(&self,
                    conn: &PgConnection,
                    other: &mut Account,
                    amount: Money)
                    -> Result<Transaction> {
        let trans = NewTransaction::transfer(self.id().clone(), other.id().clone(), amount);
        diesel_conn::execute_transaction(conn, trans).chain_err(|| "Transaction failed")

    }
}
//#[cfg(test)]
//#mod account_tests {
//# use super::*;
//# use currency::{currency as scc, Money};
//# use super::super::uuid::Uuid;
//# use transaction::Transaction;
//
//# #[test]
//# fn secure_account_and_decrypt() {
//#     let owner = Owner::new("John Doe");
//#     let mut sec_account = StoredAccount::new(owner, Money::of_major(scc::SEK, 100), "hunter1")
//#         .unwrap();
//
//#     println!("{:#?}", sec_account);
//#     sec_account.open("hunter1").unwrap();
//# }
//
//# #[test]
//# #[should_panic]
//# fn open_with_wrong_password() {
//#     let owner = Owner::new("John Doe");
//#     let mut sec_account = NewAccount::new(owner, None, "hunter1").unwrap();
//
//#     // println!("{:?}", sec_account);
//#     let open_account = sec_account.open("wrongpass").expect("Fail means success");
//# }
//
//# #[test]
//# fn check_funds() {
//#     use std::collections::HashMap;
//#     let owner = Owner::new("John Doe");
//#     let mut sec_account = NewAccount::new(owner, Money::of_major(scc::SEK, 100), "hunter1")
//#         .unwrap();
//#     let other_owner = Owner::new("Jane Doe");
//#     let mut other_sec_account =
//#         StoredAccount::new(other_owner, Money::of_major(scc::JPY, 100), "password").unwrap();
//#     sec_account.account
//#         .transactions
//#         .push(Transaction::deposit(sec_account.id, Money::of_major(scc::SEK, 100)));
//#     sec_account.account
//#         .transactions
//#         .push(Transaction::withdrawal(sec_account.id, Money::of_major(scc::ISK, 40)));
//#     sec_account.account.transactions.push(Transaction::payment(other_sec_account.id,
//#                                                                sec_account.id,
//#                                                                Money::of_major(scc::USD, 30)));
//#     other_sec_account.account
//#         .transactions
//#         .push(Transaction::payment(other_sec_account.id,
//#                                    sec_account.id,
//#                                    Money::of_major(scc::USD, 30)));
//#     let funds = sec_account.funds();
//#     let checks = {
//#         let mut checks = HashMap::new();
//#         checks.insert(scc::SEK, 20000);
//#         checks.insert(scc::ISK, -40);
//#         checks.insert(scc::USD, 3000);
//#         checks
//#     };
//
//#     assert_eq!(&funds.len(), &checks.len());
//#     for (curr, amount) in funds.iter() {
//#         assert_eq!(amount, checks.get(curr).unwrap());
//#     }
//#     // FIXME: Add check for other account
//# }
//
