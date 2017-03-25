
use argon2;
use diesel;
use dotenv;
error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Argon2(argon2::Error);
        DotEnv(dotenv::DotenvError);
        VarErr(::std::env::VarError);
        DieselConn(diesel::ConnectionError);
        Diesel(diesel::result::Error);
    }
}
