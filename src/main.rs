#![feature(plugin, custom_derive)]
#![feature(stmt_expr_attributes)]
#![plugin(serde_macros)]

extern crate uuid;
extern crate crypto;
extern crate chrono;
extern crate rand;
extern crate serde;
extern crate base64;
#[macro_use]
pub mod currency;
pub mod bank;
pub mod transaction;

fn main() {
    println!("YE")
}
