//! All the account and bank/money functions, handles things.
use std::path::Path;
use std::str;
use std::error::Error;
use std::iter;
//use std::io::{Error as IOError, IOErrorKind};

use crypto::digest::Digest;
use crypto::scrypt;
use crypto::aes;
use crypto::blockmodes;
use crypto::buffer;

use rand;
use rand::Rng;

use base64;

use serde;
use bincode;

use chrono;
use uuid::Uuid;

use currency::{Currency, IndexBill};

/// Basic representation of rscrypt, params are always 14, 8 and 1
pub struct Scrypt {
    pub salt: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Scrypt {
    pub fn new<T: AsRef<str>>(source: T) -> Result<Scrypt, &'static str> {
        // Code mainly copied from crypto::scrypt::sdrypt_check
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
    pub owner: Owner,
    pub id: Uuid, // Id of account
    pub created: chrono::DateTime<chrono::UTC>,
    pub funds: IndexBill, // TODO: Should be safer.
    // Last updated
}

impl Account {
    pub fn new<C: Currency>(owner: Owner, funds: C) -> Account {
        Account {
            owner: owner,
            id: Uuid::new_v4(),
            created: chrono::UTC::now(),
            funds: funds.to::<IndexBill>(),
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
    account_u8: Vec<u8>,
    // pub path: Path,
    pub owner: Owner,
    salt: Vec<u8>,
    //scrypt: String,
    iv: Vec<u8>,
    pub id: Uuid, // Same as Account.id
}

impl StoredAccount {
    // FIXME: Should we take account? Or just borrow?
    fn store_new<T: AsRef<str>>(password: T, account: Account) -> StoredAccount {
        let id = account.id;
        let owner = account.owner.clone();
        let iv: Vec<u8> = {
            let mut rand: rand::OsRng = match rand::OsRng::new() {
                Ok(rng) => rng,
                Err(_) => panic!("Couldn't get safe random number generator."),
            };
            rand.gen_iter().take(16).collect()
        };
        let scrypt: Scrypt = {
            let s_params = scrypt::ScryptParams::new(14, 8, 1); // Set first param to 2
            Scrypt::new(scrypt::scrypt_simple(password.as_ref(), &s_params).unwrap()).unwrap() 
        };
        let account_u8 = {
            let mut key_u8: Vec<u8> = scrypt.hash;
            assert_eq!(key_u8.len(), 32);
            let mut buf: Vec<u8> = iter::repeat(0).take(256).collect();
            let mut account_ser = bincode::serde::serialize(&account, bincode::SizeLimit::Infinite).unwrap();
            {
                let mut enc = aes::cbc_encryptor(
                    aes::KeySize::KeySize256, &key_u8,
                    &iv, blockmodes::PkcsPadding);
                let mut rr_buff = buffer::RefReadBuffer::new(&account_ser);
                let mut wr_buff = buffer::RefWriteBuffer::new(&mut buf);
                enc.encrypt(&mut rr_buff, &mut wr_buff, true);
            }
            buf
        };
        StoredAccount {
            account_u8: account_u8,
            id: id,
            salt: scrypt.salt,
            iv: iv,
            owner: owner,
        }
    }
    

    pub fn decrypt<T: AsRef<str>>(&mut self, password: T) ->
        Result<Account, bincode::serde::DeserializeError> {

        let result = {
            let key_u8 = {
                let s_params = scrypt::ScryptParams::new(14, 8, 1); // Set first param to 2
                let mut dk = [0u8; 32];
                scrypt::scrypt(password.as_ref().as_bytes(), &self.salt, &s_params, &mut dk);
                dk
            };
            println!("Len of key: {}", key_u8.len());
            let mut buf: Vec<u8> = iter::repeat(0).take(256).collect();
            {
                let mut dec = aes::cbc_decryptor(
                    aes::KeySize::KeySize256, &key_u8,
                    &self.iv, blockmodes::PkcsPadding);
                let mut rr_buff = buffer::RefReadBuffer::new(&mut self.account_u8);
                let mut wr_buff = buffer::RefWriteBuffer::new(&mut buf);
                dec.decrypt(&mut rr_buff, &mut wr_buff, true);
            }
            buf
        };
        bincode::serde::deserialize(&result)
    }
}

#[cfg(test)]
mod bank_tests {
    use super::*;
    use currency::Currency;
    currency!(SEK, 0.120293, "{} kr");
    
    #[test]
    fn secure_account_and_decrypt() {
        let owner = Owner::new("John Doe");
        let account = Account::new(owner, SEK(100.0));
        let mut sec_account = StoredAccount::store_new("hunter1", account.clone());

        //println!("{:?}", sec_account);
        let deser_account = sec_account.decrypt("hunter1").unwrap();
        assert!(account.owner == deser_account.owner &&
                account.id == deser_account.id &&
                account.created == deser_account.created &&
                account.funds == deser_account.funds);
    }
}
