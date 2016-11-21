//! All the account and bank/money functions, handles things.
use std::str;
use std::iter;
use std::hash;
use std::collections::HashMap;
//use std::io::{Error as IOError, IOErrorKind};

use crypto::scrypt;

use base64;

use chrono;
use uuid::Uuid;

use currency::Money;
use transaction::Transaction;

/// Basic representation of rscrypt, params are always 14, 8 and 1
#[derive(Debug, Serialize, Deserialize)]
pub struct Scrypt {
    pub salt: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Scrypt {
    pub fn new<T: AsRef<str>>(source: T) -> Result<Scrypt, &'static str> {
        // Code mainly copied from crypto::scrypt::scrypt_check
        static ERR_STR: &'static str = "Hash is not in Rust Scrypt format.";
        
        let mut iter = source.as_ref().split('$');

        match iter.next() {
            Some(x) => if x != "" { return Err(ERR_STR) },
            None => return Err(ERR_STR),
        }

        match iter.next() {
            Some(x) =>if x != "rscrypt" {return Err(ERR_STR)},
            None => return Err(ERR_STR),
        }

        iter.next();
        iter.next();

        let salt = base64::decode(iter.next().unwrap()).unwrap();
        let hash = base64::decode(iter.next().unwrap()).unwrap();

        Ok(Scrypt {
            salt: salt,
            hash: hash,
        })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
/// Stores everything one have to know about the account.
pub struct Account {
    // FIXME Make Transaction with Currency instead of Build
    pub transactions: Vec<Transaction>,
}

impl Account {
    // TODO: Make initial_funds generic with C: Currency
    pub fn new(initial_funds: Option<Money>, owner_id: Uuid) -> Account {
        let mut transactions = Vec::new();
        if initial_funds.is_some() {
            transactions.push(Transaction::Deposit {
                from: owner_id,
                date: chrono::UTC::now(),
                amount: initial_funds.unwrap(),
            });
        }
        Account {
            transactions: transactions,
        }
    }

}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    /// An end-user
    pub id: Uuid, // Id of owner.
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

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredAccount {
    /// A stored Account.
    account: Account,
    pub pending_transactions: Vec<Transaction>,
    // pub path: Path,
    pub owner: Owner,
    scrypt: String,
    pub id: Uuid, // Same as Account.id, not same as owner.id
    pub created: chrono::DateTime<chrono::UTC>,
    pub last_updated: chrono::DateTime<chrono::UTC>,
}

impl hash::Hash for StoredAccount {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        self.id.hash(state)
    }
}

impl StoredAccount {
    // FIXME: Should we take account? Or just borrow?
    fn new<T: AsRef<str>>(owner: Owner, funds: Option<Money>, password: T) -> StoredAccount {
        #[cfg(debug_assertions)]
        println!("WARNING! Please note that currently all accounts are using plaintext passwords\n\
                  Build in --release to use scrypt");
        let id = Uuid::new_v4();

         #[cfg(not(debug_assertions))]
        let scrypt: String = {
            let s_params = scrypt::ScryptParams::new(14, 8, 1);
            scrypt::scrypt_simple(password.as_ref(), &s_params).unwrap()
        };
        #[cfg(debug_assertions)]
        let scrypt: String = String::from(password.as_ref());

        let account = Account::new(funds, owner.id);
        let created = chrono::UTC::now();

        StoredAccount {
            account: account,
            pending_transactions: Vec::new(),
            id: id,
            scrypt: scrypt,
            owner: owner,
            created: created,
            last_updated: created,
        }
    }
    

    pub fn open<T: AsRef<str>>(&mut self, password: T) -> Result<&mut Account, ()> {
        #[cfg(not(debug_assertions))]
        let password_matches = 
            scrypt::scrypt_check(password.as_ref(), &self.scrypt).unwrap();
        #[cfg(debug_assertions)]
        let password_matches = {
            password.as_ref() == self.scrypt
        };

        if password_matches {
            return Ok(&mut self.account);
        }
        return Err(());
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccountStorage{
    _accounts: HashMap<Uuid, StoredAccount>, // FIXME: This should actually be a unloaded StoredAccount, less memory.
}

impl AccountStorage {
    pub fn fetch(&mut self, id: &Uuid) -> Result<&mut StoredAccount, ()> {
        self._accounts.get_mut(id).ok_or(())
    }
}

#[cfg(test)]
mod bank_tests {
    use super::*;
    use currency::{Currency, Money};
    
    #[test]
    fn secure_account_and_decrypt() {
        let owner = Owner::new("John Doe");
        let mut sec_account = StoredAccount::new(owner, Some(Money::new(Currency::SEK, 100.0)), "hunter1");

        println!("{:#?}", sec_account);
        let open_account = sec_account.open("hunter1").unwrap();
        println!("{:#?}", open_account);
    }

    #[test]
    #[should_panic]
    fn open_with_wrong_password() {
        let owner = Owner::new("John Doe");
        let mut sec_account = StoredAccount::new(owner, None, "hunter1");

        //println!("{:?}", sec_account);
        let open_account = sec_account.open("wrongpass").expect("Fail means success");
    }
}