use std::collections::HashMap;

pub use steel_cent::Money;
pub use steel_cent::currency;

// TODO: Better name please
pub fn convert_map_to_money(map: HashMap<currency::Currency, i64>) -> Vec<Money> {
    let mut vec = vec![];
    for (c, a) in map {
        vec.push(Money::of_minor(c, a));
    }
    vec
}
