
use serde_json;
error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        SerdeJson(serde_json::Error);

    }
}
