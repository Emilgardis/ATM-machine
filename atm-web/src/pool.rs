use atm_lib::interface;
use diesel::pg::PgConnection;
use error::*;
use r2d2;
use r2d2_diesel;
use rocket;
use rocket::{Outcome, State};
use std::ops::Deref;
pub type PgConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<PgConnection>>;

pub struct Conn(r2d2::PooledConnection<r2d2_diesel::ConnectionManager<PgConnection>>);

impl Deref for Conn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn establish_connection_pool<S>(db_url: S) -> Result<PgConnectionPool>
    where S: Into<Option<String>>
{
    let config = r2d2::Config::default();
    let manager = r2d2_diesel::ConnectionManager::new(db_url.into()
        .unwrap_or(interface::get_database_url()?));

    r2d2::Pool::new(config, manager).map_err(|e| e.into())
}

impl<'a, 'r> rocket::request::FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a rocket::Request<'r>) -> rocket::request::Outcome<Conn, ()> {
        let pool = match <State<PgConnectionPool> as rocket::request::FromRequest>::from_request(request) {
            Outcome::Success(pool) => pool,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(_) => return Outcome::Forward(()),
        };

        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((rocket::http::Status::ServiceUnavailable, ())),
        }
    }
}
