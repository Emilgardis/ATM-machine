pub mod account;
pub mod index;
pub mod owner;
pub mod transaction;
use pool;
use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;
use rocket::Outcome;

pub struct AdminUser;

impl <'a, 'r> FromRequest<'a, 'r > for AdminUser {
    type Error = &'static str;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminUser, Self::Error> {
        let conn = match <pool::Conn as FromRequest>::from_request(request) {
            Outcome::Success(conn) => conn,
            Outcome::Failure(_) => return Outcome::Failure((Status::NotFound, "Unable to get a connection to database")),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        let admin = request.session()
           .get("admin_id")
           .map(|cookie| cookie.value() == "1");
        match admin {
            Some(true) => Outcome::Success(AdminUser),
            None => Outcome::Forward(()),
            _ => Outcome::Failure((Status::Unauthorized, "User is not authorized to access this page.")),
        }
    }
}

