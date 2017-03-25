#![feature(plugin, custom_derive)]
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

#[macro_use]
extern crate error_chain;

extern crate uuid;

pub mod error;
pub mod pool;
pub mod routes;

use error::*;

quick_main!(run);

fn run() -> Result<i32> {
    let pool = pool::establish_connection_pool(None)?;
    rocket::ignite()
        .manage(pool)
        .mount("/", routes![routes::admin_show_accounts, routes::admin_show_account])
        .launch();

    Ok(0)
}

pub fn print_error_to_stderr<E>(e: E) where E: Into<Error> {
        use ::std::io::Write;
        let e = e.into();
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }
}
