use atm_lib::error as atm_error;
use r2d2;

error_chain! {
    links {
        AtmLib(atm_error::Error, atm_error::ErrorKind);
    }
    foreign_links {
        PoolInitializationError(r2d2::InitializationError);
    }
}
