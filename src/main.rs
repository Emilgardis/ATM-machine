#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]

extern crate uuid;
extern crate crypto;
extern crate chrono;
extern crate rand;
extern crate serde;
extern crate bincode;
extern crate base64;
#[macro_use]
pub mod currency;
pub mod bank;


fn main() {
    println!("YE")
}
