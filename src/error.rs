
use serde_json;
use argon2;
error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        SerdeJson(serde_json::Error);
        Argon2(argon2::Error);
    }
}
