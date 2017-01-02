//! All the account and bank/money functions, handles things.
use std::str;
use std::iter;
use std::hash;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

#[cfg(not(debug_assertions))]
use crypto::scrypt;

use base64;

use chrono;
use uuid::Uuid;

use serde_json;

use currency::{Currency, Money};
use transaction::{Transaction, PendingTransaction};

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
    #[serde(rename="Transactions")]
    pub transactions: Vec<Transaction>,
    #[serde(rename="PendingTransactions")]
    pub pending_transactions: Vec<PendingTransaction>,
}

impl Account {
    // TODO: Make initial_funds generic with C: Currency
    pub fn new(initial_funds: Option<Money>, id: Uuid) -> Account {
        let mut transactions = Vec::new();
        if initial_funds.is_some() {
            transactions.push(Transaction::Deposit {
                from: id,
                date: chrono::UTC::now(),
                amount: initial_funds.unwrap(),
            });
        }
        Account {
            transactions: transactions,
            pending_transactions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredAccount {
    /// A stored Account.
    pub account: Account,
    // pub path: Path,
    #[serde(rename="Owner")]
    pub owner: Owner,
    #[serde(rename="Hash")]
    scrypt: String,
    #[serde(rename="Id")]
    pub id: Uuid, // Same as Account.id, not same as owner.id
    #[serde(rename="Created")]
    pub created: chrono::DateTime<chrono::UTC>,
    #[serde(rename="LastUpdated")]
    pub last_updated: chrono::DateTime<chrono::UTC>,
}

impl hash::Hash for StoredAccount {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        self.id.hash(state)
    }
}

impl StoredAccount {
    // FIXME: Should we take account? Or just borrow?
    pub fn new<T: AsRef<str>, F: Into<Option<Money>>>(owner: Owner, funds: F, password: T) -> StoredAccount {
        #[cfg(all(debug_assertions, not(test)))] // Disable this print on test, but enable otherwise when in debug
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

        let account = Account::new(funds.into(), id);
        let created = chrono::UTC::now();

        StoredAccount {
            account: account,
            id: id,
            scrypt: scrypt,
            owner: owner,
            created: created,
            last_updated: created,
        }
    }
    

    pub fn open<T: AsRef<str>>(&mut self, password: T) -> Result<(), ()> {
        #[cfg(not(debug_assertions))]
        let password_matches = 
            scrypt::scrypt_check(password.as_ref(), &self.scrypt).unwrap();
        #[cfg(debug_assertions)]
        let password_matches = {
            password.as_ref() == self.scrypt
        };

        if password_matches {
            // return Ok(&mut self.account); FIXME: Make account.transactions locked behind crypto.
            return Ok(())
        }
        return Err(());
    }

    pub fn funds(&self) -> HashMap<Currency, f64> {
        let mut map: HashMap<Currency, f64> = HashMap::new();
        for trans in &self.account.transactions {
            if let Some((curr, amount)) = trans.get_change(&self.id){
                *map.entry(curr).or_insert(0.0) += amount;
            }
        }
        // map.into_iter().map(|(curr, amount)| Money::new(curr, amount)).collect()
        map
    } 
}

#[derive(Debug)]
struct FiledAccount {
    pub path: String,
    pub id: Uuid,
    pub owner: Owner,
}

impl FiledAccount {
    pub fn access(&self, folder: PathBuf) -> Result<StoredAccount, serde_json::Error> {
        let mut path = folder.clone();
        path.push(&self.path);
        println!("Accessing {:?}.", path);
        let file = fs::OpenOptions::new()
            .read(true).open(&self.path)?;
        FiledAccount::_load(file)
    }

    fn _load(file: fs::File) -> Result<StoredAccount, serde_json::Error> {
        // TODO: Make it so that a check exists so max one StoredAccount exists for each account.
        serde_json::from_reader(file)
    }
    pub fn new<F: AsRef<Path>>(file_path: F) -> Result<FiledAccount, serde_json::Error> {
        let file = fs::OpenOptions::new()
            .read(true).open(file_path)?;
        let account = FiledAccount::_load(file)?;
        let id = format!("{}.acc", account.id);
        Ok(
            FiledAccount{
                path: id,
                id: account.id.clone(),
                owner: account.owner.clone(),
            }
        )
    }
    #[deny(unused_result)]
    pub fn store<P: AsRef<Path>>(mut account: StoredAccount, folder: P) -> Result<FiledAccount, serde_json::Error> {
        // Maybe use Future??
        let folder = folder.as_ref();
        assert!(folder.is_dir());
		// --------------------------------------------------------------------------------------------
		// FIXME: FIXME: FIXME!!!!
        let id = format!("{}+.acc", account.id);
        let mut path = folder.to_path_buf();
        path.push(id.clone());
        println!("Saving {:?} to {:?}", id, path);
        let mut file = fs::OpenOptions::new()
            .create_new(true).write(true).open(&path)?;
        serde_json::to_writer(&mut file, &mut account)?;

        Ok(
            FiledAccount { // FIXME: This is seriously wrong somehow.
                // Maybe they changed box syntax?
                path: id, // path and id is the same thing in different formats. FIXME: 
                id: account.id.clone(),
                owner: account.owner.clone(),
            }    
        )
    }
}

impl hash::Hash for FiledAccount {
    fn hash<H>(&self, state: &mut H) where H: hash::Hasher {
        self.id.hash(state)
    }
}
pub struct AccountStorage {
    _accounts: HashMap<Uuid, FiledAccount>,
    path: PathBuf,
}

impl AccountStorage {
    pub fn from<T: AsRef<Path>>(path: T) -> Result<AccountStorage, serde_json::Error> {
        use std::ffi::OsStr;
        let path = path.as_ref().to_path_buf();
        let mut accounts = HashMap::new();
        for entry in path.read_dir()? {
            let entry = entry?.path();
            if entry.extension() != Some(OsStr::new("acc")) {
                continue;
            }
            let acc: FiledAccount = FiledAccount::new(entry)?;
            println!("Found {:?}, added to storage.", acc);
            accounts.insert(acc.id.clone(), acc);
        }
        Ok(
            AccountStorage {
                _accounts: accounts,
                path: path,
            }
        )
    }
    fn fetch_unloaded(&mut self, id: &Uuid) -> Result<&mut FiledAccount, serde_json::Error> {
        self._accounts.get_mut(id)
            .ok_or(io::Error::new(io::ErrorKind::Other, format!("No such account: {}", id)).into())
    }

    pub fn fetch(&mut self, id: &Uuid) -> Result<StoredAccount, serde_json::Error> {
        // TODO: Consider making this bounded with mut and a lifetime to prevent two
        // StoredAccounts on same path.
        let path = self.path.clone();
        let account = self.fetch_unloaded(id)?.access(path)?;
        Ok(account)
    }

    pub fn store(&mut self, account: StoredAccount) -> Result<(), serde_json::Error>{
        let filedacc = FiledAccount::store(account, &self.path)?;
        self._accounts.insert(filedacc.id, filedacc);
        Ok(())
    }

    pub fn get_ids(&self) -> Vec<&Uuid> {
        self._accounts.keys().collect()
    }
    

    pub fn get_owner(&self, id: &Uuid) -> Result<Owner, serde_json::Error> {
        let acc: &FiledAccount = self._accounts.get(id)
            .ok_or(io::Error::new(io::ErrorKind::Other, format!("No such account: {}", id)))?;
        let owner = acc.owner.clone();
        Ok(owner)
    }

    /// Get accounts of user with id user_id
    pub fn get_accounts_of_user(&mut self, user_id: &Uuid) -> Vec<Result<StoredAccount, serde_json::Error>> {
        let mut res = Vec::new();
        for acc in &mut self._accounts.values_mut() {
            if &acc.owner.id == user_id {
                res.push(acc.access(self.path.clone()));
            }
        }
        res
    }
    pub fn get_accounts(&mut self) -> Vec<Result<StoredAccount, serde_json::Error>> {
        let mut res = Vec::new();
        for acc in &mut self._accounts.values_mut() {
            res.push(acc.access(self.path.clone()));
        }
        res
    }
}

#[cfg(test)]
mod account_tests {
    use super::*;
    use currency::{Currency, Money};
    use super::super::uuid::Uuid;
    use transaction::Transaction;
    
    #[test]
    fn secure_account_and_decrypt() {
        let owner = Owner::new("John Doe");
        let mut sec_account = StoredAccount::new(owner, Money::new(Currency::SEK, 100.0), "hunter1");

        println!("{:#?}", sec_account);
        sec_account.open("hunter1").unwrap();
    }

    #[test]
    #[should_panic]
    fn open_with_wrong_password() {
        let owner = Owner::new("John Doe");
        let mut sec_account = StoredAccount::new(owner, None, "hunter1");

        //println!("{:?}", sec_account);
        let open_account = sec_account.open("wrongpass").expect("Fail means success");
    }

    #[test]
    fn check_funds() { 
        use std::collections::HashMap;
        let owner = Owner::new("John Doe");
        let mut sec_account = StoredAccount::new(owner, Money::new(Currency::SEK, 100.0), "hunter1");
        let other_owner = Owner::new("Jane Doe");
        let mut other_sec_account = StoredAccount::new(other_owner, Money::new(Currency::Yen, 100.0), "password");
        sec_account.account.transactions.push(Transaction::deposit(
            sec_account.id, Money::new(Currency::SEK, 100.0)));
        sec_account.account.transactions.push(Transaction::withdrawal(
            sec_account.id, Money::new(Currency::Other("ISK".into()), 40.0)));
        sec_account.account.transactions.push(Transaction::payment(
            other_sec_account.id, sec_account.id, Money::new(Currency::Dollar, 30.0)));
        other_sec_account.account.transactions.push(Transaction::payment(
            other_sec_account.id, sec_account.id, Money::new(Currency::Dollar, 30.0)));
        let funds = sec_account.funds();
        let checks = {
            let mut checks = HashMap::new();
            checks.insert(Currency::SEK, 200.0);
            checks.insert(Currency::Other("ISK".into()), -40.0);
            checks.insert(Currency::Dollar, 30.0);
            checks
        };

        assert_eq!(&funds.len(), &checks.len());
        for (curr, amount) in funds.iter() {
            assert_eq!(amount, checks.get(curr).unwrap());
        }
        // FIXME: Add check for other account
    }
}
