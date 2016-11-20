#![feature(proc_macro)]
#![feature(stmt_expr_attributes)]

extern crate uuid;
extern crate crypto;
extern crate chrono;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate base64;
pub mod currency;
pub mod bank;
pub mod transaction;
