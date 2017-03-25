#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate atm_lib;

extern crate diesel;

extern crate rocket;
extern crate rocket_contrib;

extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

fn main() {
    println!("Hello, world!");
}
