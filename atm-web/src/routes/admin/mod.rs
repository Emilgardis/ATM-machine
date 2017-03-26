pub mod account;
pub mod index;
pub mod owner;

use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;
use rocket::Outcome;

pub struct AdminUser;

impl <'a, 'r> FromRequest<'a, 'r > for AdminUser {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, ()> {
        let admin = request.session()
           .get("admin_id")
           .map(|cookie| cookie.value() == "1");
        match admin {
            Some(true) => Outcome::Success(AdminUser),
            Some(false) => Outcome::Failure((Status::Unauthorized, ())),
            None => Outcome::Forward(()),
        }
    }
}

