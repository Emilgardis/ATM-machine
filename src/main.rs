#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]

extern crate uuid;
extern crate crypto;
extern crate chrono;
extern crate rand;
extern crate serde;
extern crate serde_json;
pub mod bank;
#[macro_use]
pub mod currency;


fn main() {
    println!("YE")
}
