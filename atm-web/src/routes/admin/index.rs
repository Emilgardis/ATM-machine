use atm_lib::account::Account;
use atm_lib::currency;
use atm_lib::interface;
use atm_lib::transaction;
use error;
use pool;
use rocket;
use rocket::http::{Cookie, Session, Status};
use rocket::request::{FlashMessage, Form};
use rocket::response::{Failure, Flash, Redirect};
use rocket_contrib::Template;
use rocket_contrib::UUID;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use super::AdminUser;
use super::account;
use uuid::Uuid;


#[derive(FromForm)]
struct AdminForm {
    username: String,
    password: String,
}

#[post("/admin-login", data = "<admin_form>")]
fn admin_login_post(socket_addr: SocketAddr,
                    mut session: Session,
                    admin_form: Form<AdminForm>)
                    -> Result<Redirect, Flash<Redirect>> {
    println!("Login from {:?}", socket_addr);
    let admin = admin_form.get();

    if (admin.username == "admin") & (admin.password == "hunter1") {
        //let pw_hash: String = {
        //    let mut rng = OsRng::new().expect("Rng failed");

        //    let salt: Vec<u8> = rng.gen_iter::<u8>().take(16).collect();
        //    let pw = "1".as_bytes();
        //    let config = argon2::Config::default();
        //    argon2::hash_encoded(pw, salt.as_slice(), &config)
        //        .expect("Hash failed, shouldn't happen")
        //        .to_owned()
        //};
        session.set(
            Cookie::build("admin_id", "1")
            //.secure(true) 
            .finish()
            );
        return Ok(Redirect::to("/admin-panel"));
    }
    println!("Login attempted but failed!!");
    Err(Flash::error(Redirect::to("/admin-panel"), "Invalid login"))
}

#[get("/admin-panel")]
fn index_page(_admin: AdminUser, conn: pool::Conn) -> error::Result<Template> {
    // Borrowed from account.rs::show_accounts
    let context = account::AccountsContext {
        accounts: interface::diesel_conn::all_accounts(&conn)
            ?
            .into_iter()
            .map(|acc| {
                account::make_account_context(&conn, &acc.id).expect("FIX THIS, SHOULDN'T HAPPEN")
            })
            .collect(),
    };
    Ok(Template::render("admin-panel/admin_view", &context))
}

#[get("/admin-panel", rank = 1)]
fn index_login_page(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    if let Some(ref flash) = flash {
        context.insert("name", flash.name());
        context.insert("msg", flash.msg());
    }
    Template::render("admin-panel/admin_login", &context)
}

#[get("/admin-panel/<path..>", rank = 99)]
fn no_admin_fall(path: PathBuf) -> Failure {
    Failure(Status::Unauthorized)
}
