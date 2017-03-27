use argon2;
use diesel;
use dotenv;

error_chain! {
    links { 
        DotEnv(dotenv::Error, dotenv::ErrorKind);
    }
    foreign_links {
        Fmt(::std::fmt::Error);
        Argon2(argon2::Error);
        VarErr(::std::env::VarError);
        DieselConn(diesel::ConnectionError);
        Diesel(diesel::result::Error);
    }
}
