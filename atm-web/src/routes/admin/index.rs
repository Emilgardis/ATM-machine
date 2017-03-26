
use pool;
use rocket_contrib::Template;
use rocket;
use rocket_contrib::UUID;
use atm_lib::account::Account;
use atm_lib::transaction;
use atm_lib::interface;
use atm_lib::currency;
use uuid::Uuid;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};
use rocket::http::{Cookies, Cookie, Session};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;

use super::AdminUser;
#[derive(FromForm)]
struct AdminForm {
    username: String,
    password: String,
}

#[post("/admin-login", data = "<admin_form>")]
fn admin_login_post(socket_addr: SocketAddr, mut session: Session, admin_form: Form<AdminForm>) -> Result<Redirect, Flash<Redirect>> {
    println!("Login from {:?}", socket_addr);
    let admin = admin_form.get();
    
    if (admin.username == "admin") & (admin.password == "hunter1") {
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
fn index_page(_admin: AdminUser) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("admin_view", &context)
}

#[get("/admin-panel", rank = 1)]
fn index_login_page(flash: Option<FlashMessage>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    if let Some(ref flash) = flash {
        context.insert("name", flash.name());
        context.insert("msg", flash.msg());
    }
    Template::render("admin_login", &context)
}

#[get("/admin-panel/<path..>", rank = 99)]
fn no_admin_fall(path: PathBuf) -> Redirect {
    Redirect::to("/admin-panel")
}