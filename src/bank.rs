//! All the account and bank/money functions, handles things.
use std::path::Path;
use std::str;
//use std::io::{Error as IOError, IOErrorKind};

use crypto::digest::Digest;
use crypto::aes;
use crypto::blockmodes;
use crypto::buffer;
use rand;
use rand::Rng;

use serde;
use serde_json;

use chrono;
use uuid::Uuid;

use currency::{Currency, IndexBill};


#[derive(Serialize, Deserialize)]
pub struct Account {
    /// Stores everything one have to know about the account.
    pub owner: Owner,
    pub id: Uuid, // Id of account
    pub created: chrono::DateTime<chrono::UTC>,
    pub funds: u64, // TODO: Should be safer.
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Owner {
    /// An end-user
    pub id: Uuid, // Id of owner.
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct StoredAccount {
    /// A stored Account.
    account_string: String,
    // pub path: Path,
    pub owner: Owner,
    pub id: Uuid, // Same as Account.id
}

impl StoredAccount {
    fn store<T: AsRef<str>>(owner: &Owner, password: T, account: Account) -> StoredAccount {
        let id = Uuid::new_v4();
        // FIXME: Values taken from haskells default of ScryptParams, are they good?
        let account_string = {
            let mut password_u8: Vec<u8> = password.as_ref().chars().map(|ch| ch as u8).collect();
            let iv: Vec<u8> = {
                let mut rand: rand::OsRng = match rand::OsRng::new() {
                    Ok(rng) => rng,
                    Err(_) => panic!("Couldn't get safe random number generator."),
                };
                rand.gen_iter().take(16).collect()
            };
            let mut buf = Vec::new();
            {
                let mut enc = aes::cbc_encryptor(
                    aes::KeySize::KeySize256, &password_u8,
                    &iv, blockmodes::PkcsPadding);
                let mut wr_buff = buffer::RefWriteBuffer::new(&mut buf);
                let mut rr_buff = buffer::RefReadBuffer::new(&password_u8);
                enc.encrypt(&mut rr_buff, &mut wr_buff, true);
            }
            str::from_utf8(&buf)
                .unwrap_or_else(|e| panic!("Couldn't make parse result. \n{:?}", e))
                .into()
        };
        StoredAccount {
            account_string: account_string,
            id: id,
            owner: owner.clone(),
        }
    }

}
