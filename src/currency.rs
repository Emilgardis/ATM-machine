use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub currency: Currency,
    pub amount: f64,
}

impl Money {
    pub fn new(currency: Currency, amount: f64) -> Money {
        Money {
            currency: currency,
            amount: amount,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    Dollar,
    SEK,
    Yen,
    Euro,
    Other(String),
}

pub struct CurrencyInfo {
    format: String,
    index: f64,
}

struct CurrencyDatabase {
    db: BTreeMap<Currency, f64>
}
