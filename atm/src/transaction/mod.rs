use currency::{Money, currency};

use uuid::Uuid;
use chrono;
use diesel;
use diesel::prelude::*;
use diesel::expression::{self, AsExpression};
use diesel::types::{Nullable, SmallInt};
use error;
use interface::schemas::transactions;
#[repr(i16)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TransactionType {
    Transfer = 1,
    Deposit = 2,
    Withdrawal = 3,
    Payment = 4,
}

impl diesel::types::FromSqlRow<diesel::types::SmallInt, diesel::pg::Pg> for TransactionType {
    fn build_from_row<R: diesel::row::Row<diesel::pg::Pg>>(row: &mut R) -> ::std::result::Result<Self, Box<::std::error::Error+Send+Sync>> {
        use self::TransactionType::*;
        match i16::build_from_row(row)? {
            1 => Ok(Transfer),
            2 => Ok(Deposit),
            3 => Ok(Withdrawal),
            4 => Ok(Payment),
            other => Err(format!("Unable to make a TransactionType of value {}", other).into()),
        }
    }
}

impl<'a> AsExpression<SmallInt> for &'a TransactionType {
    type Expression = expression::helper_types::AsExprOf<i16, SmallInt>;
    fn as_expression(self) -> Self::Expression {
        AsExpression::<SmallInt>::as_expression(*self as i16)
    }
}
impl<'a> AsExpression<Nullable<SmallInt>> for &'a TransactionType {
    type Expression = expression::helper_types::AsExprOf<i16, Nullable<SmallInt>>;
    fn as_expression(self) -> Self::Expression {
        AsExpression::<Nullable<SmallInt>>::as_expression(*self as i16)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name="transactions"]
pub struct NewTransaction {
	// TODO: Better name please.
    sender: Uuid,
    recipient: Option<Uuid>,
    trans_type: TransactionType,
    amount: i64,
    currency: String, 
    date: chrono::DateTime<chrono::UTC>,
}

impl NewTransaction {
    pub fn deposit(from: Uuid, money: Money) -> NewTransaction {
        TransactionE::Deposit {
            from: from,
            amount: money,
            date: chrono::UTC::now(),
        }.into()
    }

    pub fn withdrawal(to: Uuid, money: Money) -> NewTransaction {
        TransactionE::Withdrawal {
            to: to,
            amount: money,
            date: chrono::UTC::now(),
        }.into()
    }

    pub fn transfer(sender: Uuid, recipient: Uuid, money: Money) -> NewTransaction {
        TransactionE::Transfer {
            sender: sender,
            recipient: recipient,
            amount: money,
            date: chrono::UTC::now(),
        }.into()
    }

    pub fn payment(sender: Uuid, recipient: Uuid, money: Money) -> NewTransaction {
        TransactionE::Payment {
            sender: sender,
            recipient: recipient,
            amount: money,
            date: chrono::UTC::now(),
        }.into()
    }

}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Transaction {
    serial: i32,
    sender: Uuid,
    recipient: Option<Uuid>,
    trans_type: TransactionType,
    amount: i64,
    currency: String, 
    date: chrono::DateTime<chrono::UTC>,
}

impl ::std::convert::TryFrom<Transaction> for TransactionE {
    type Error = error::Error;
    fn try_from(ts: Transaction) -> error::Result<TransactionE> {
        let ty = ts.trans_type;
        use self::TransactionType::*;
        let amount = Money::of_minor(
            currency::with_code(
                &ts.currency).ok_or(format!("Not a valid currency code: {}", ts.currency))?,
            ts.amount);
        Ok(
            match ty {
                Transfer => {
                    TransactionE::Transfer {
                        sender: ts.sender,
                        recipient: ts.recipient
                            .ok_or("Transaction of type `Transfer` was invalid, no recipient.")?,
                        amount: amount,
                        date: ts.date,
                    }
                },
                Deposit => {
                    if ts.recipient.is_some() {
                        bail!("Transaction of type `Transfer` was invalid, recipient specified.");
                    }
                    TransactionE::Deposit {
                        from: ts.sender,
                        amount: amount,
                        date: ts.date,
                    }
                },
                Withdrawal => {
                    if ts.recipient.is_some() {
                        bail!("Transaction of type `Transfer` was invalid, recipient specified.");
                    }
                    TransactionE::Withdrawal {
                        to: ts.sender,
                        amount: amount,
                        date: ts.date,
                    }
                },
                Payment => {
                    TransactionE::Payment {
                        sender: ts.sender,
                        recipient: ts.recipient
                            .ok_or("Transaction of type `Transfer` was invalid, no recipient.")?,
                        amount: amount,
                        date: ts.date,
                    }
                },
            }
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionE {
    Transfer {
        sender: Uuid,
        recipient: Uuid,
        amount: Money,
        date: chrono::DateTime<chrono::UTC>,
    },
    Deposit {
        from: Uuid,
        amount: Money,
        date: chrono::DateTime<chrono::UTC>,
    },
    Withdrawal {
        to: Uuid,
        amount: Money,
        date: chrono::DateTime<chrono::UTC>,
    },
    Payment {
        sender: Uuid,
        recipient: Uuid,
        amount: Money,
        date: chrono::DateTime<chrono::UTC>,
    },
}

impl From<TransactionE> for NewTransaction {
    fn from(ts: TransactionE) -> NewTransaction {
        use steel_cent::formatting::FormattableMoney;
        let (sender, recipient, date, money, ty) = match ts {
            TransactionE::Deposit { from, amount, date } => {
                (from, None, date, amount, TransactionType::Deposit)
            }
            TransactionE::Transfer { sender, recipient, amount, date } => {
                (sender, Some(recipient), date, amount, TransactionType::Transfer)
            }
            TransactionE::Payment { sender, recipient, amount, date } => {
                (sender, Some(recipient), date, amount, TransactionType::Payment)
            }
            TransactionE::Withdrawal { to, amount, date } => {
                (to, None, date, amount, TransactionType::Withdrawal)
            }
        };
        NewTransaction {
            sender: sender,
            recipient: recipient,
            trans_type: ty,
            amount: money.minor_amount(),
            currency: money.currency.code(),
            date: date,

        }
    }
}

impl Transaction {
    pub fn serial(&self) -> i32 {
        self.serial
    }
    /// As account with id `id`, how much does this transaction affect me?
    pub fn get_change(&self, id: &Uuid) -> error::Result<Option<Money>> {
        let amount = Money::of_minor(
            currency::with_code(
                &self.currency).ok_or(format!("Not a valid currency code: {}", self.currency))?,
            self.amount);
        match self.trans_type {
            TransactionType::Deposit => {
                if &self.sender == id {
                    return Ok(Some(amount));
                }
                Ok(None)
            }
            TransactionType::Transfer |
            TransactionType::Payment => {
                if self.recipient.is_none() {
                    bail!("Transaction of type `{:?}` was invalid, recipient was null", self.trans_type );
                }
                if &self.sender == id {
                    return Ok(Some(amount));
                };
                if &self.recipient.unwrap() == id {
                    return Ok(
                        Some(
                            amount.checked_neg()
                            .expect("This error shouldn't happen, but not sure how to fix.")
                            )
                        );
                };
                Ok(None)
            }
            TransactionType::Withdrawal => {
                if self.recipient.is_none() {
                    bail!("Transaction of type `{:?}` was invalid, recipient was null", self.trans_type );
                }
                if &self.recipient.unwrap() == id {
                    return Ok(
                        Some(
                            amount.checked_neg()
                            .expect("This error shouldn't happen, but not sure how to fix.")
                            )
                        );
                };
                Ok(None)
            }
        }
    }
}


/// This is here more for bookkeeping.
// FIXME: Decide if this should be used in transactions and or pending_transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTransaction {
    transaction: Transaction,
    status: TransactionStatus,
    sent: chrono::DateTime<chrono::UTC>,
    responded: Option<chrono::DateTime<chrono::UTC>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed, // FIXME: Should this be used, pendingtransactions vec becomes more of a logbook
    Declined,
}


#[cfg(test)]
mod transaction_tests {
    use super::*;

    #[test]
    #[ignore]
    fn check_get_change() {
        panic!()
    }
}
