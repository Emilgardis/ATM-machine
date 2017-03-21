use std::collections::BTreeMap;

pub use steel_cent::Money;
pub use steel_cent::currency;

struct CurrencyDatabase {
    db: BTreeMap<currency::Currency, f64>
}
