//! Defines behaviour for currency.
//! 
//! Taken freely from https://github.com/archer884/exchange/

pub trait Currency {
    type Value;
    fn to_normal(&self) -> f64;
    fn from_normal(f64) -> Self::Value;
    fn to<C: Currency>(&self) -> <C as Currency>::Value;
}

macro_rules! currency {
    ($t:ident, $c:expr, $disp:expr) => {
        #[derive(Copy, Clone, Debug)]
        struct $t(f64);

        impl Currency for $t {
            type Value = $t;

            fn to_normal(&self) -> f64 {
                self.0 * $c
            }

            fn from_normal(n: f64) -> Self::Value {
                $t(n / $c)
            }

            fn to<C: Currency>(&self) -> <C as Currency>::Value {
                C::from_normal(self.to_normal())
            }
        }

        impl<C: Currency> ::std::ops::Add<C> for $t {
            type Output = <Self as Currency>::Value;

            fn add(self, rhs: C) -> Self::Output {
                Self::Output::from_normal(self.to_normal() + rhs.to_normal())
            }
        }

        impl<C: Currency> ::std::cmp::PartialEq<C> for $t {
            fn eq(&self, rhs: &C) -> bool {
                self.to_normal() == rhs.to_normal()
            }
            fn ne(&self, rhs: &C) -> bool {
                self.to_normal() != rhs.to_normal()
            }
        }

        impl ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, $disp, self.0)
            }
        }
    }
}


#[derive(Debug)]
pub struct IndexBill(f64);

impl Currency for IndexBill {
    type Value = IndexBill;

    fn to_normal(&self) -> f64 {
        self.0
    }

    fn from_normal(n: f64) ->Self::Value {
        IndexBill(n)
    }

    fn to<C: Currency>(&self) -> <C as Currency>::Value {
        C::from_normal(self.to_normal())
    }
}

impl<C: Currency> ::std::ops::Add<C> for IndexBill {
    type Output = <Self as Currency>::Value;

    fn add(self, rhs: C) -> Self::Output {
        Self::Output::from_normal(self.to_normal() + rhs.to_normal())
    }
}
impl ::std::fmt::Display for IndexBill {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{} units", self.0)
    }
}


#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;
    #[test]
    fn currency_works(){
        currency!(USD, 1.00, "${} USD");
        currency!(SEK, 0.120293, "{} kr");
        let usd: USD = USD(100.0);
        let sek: SEK = SEK(100.0 / 0.120293);
        println!("Value is {}, {}", usd, usd.to::<SEK>());
        assert_eq!(usd, sek)
    }
}
