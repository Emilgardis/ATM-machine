use currency::Money;

use uuid::Uuid;
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
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

impl Transaction {
    /// As account with id `id`, how much does this transaction affect me?
    pub fn get_change(&self, id: &Uuid) -> Option<Money> {
        match *self {
            Transaction::Deposit { from, amount, date: _} => {
                if &from == id { // What does from even do here?
                    return Some(amount);
                }
                return None;
            },
            Transaction::Transfer { sender, recipient, amount, date: _} => {
                if &sender == id {
                    return Some(amount.checked_neg().unwrap());
                };
                if &recipient == id {
                    return Some(amount);
                };
            },
            Transaction::Payment { sender, recipient, amount, date: _} => {
                if &sender == id {
                    return Some(amount.checked_neg().unwrap());
                };
                if &recipient == id {
                    return Some(amount);
                };
                return None; // This shouldn't happen :/, user error
            },
            Transaction::Withdrawal{ to, amount, date: _} => {
                if &to == id {
                    return Some(amount.checked_neg().unwrap());
                };
                return None; // Shouldn't also happen...
            },
        };
        return None;
    }

    pub fn deposit(from: Uuid, money: Money) -> Transaction {
        Transaction::Deposit {
            from: from,
            amount: money,
            date: chrono::UTC::now(),
        }
    }
    
    pub fn withdrawal(to: Uuid, money: Money) -> Transaction {
        Transaction::Withdrawal {
            to: to,
            amount: money,
            date: chrono::UTC::now(),
        }
    }
    
    pub fn transfer(sender: Uuid, recipient: Uuid, money: Money) -> Transaction {
        Transaction::Transfer {
            sender: sender,
            recipient: recipient,
            amount: money,
            date: chrono::UTC::now(),
        }
    }

    pub fn payment(sender: Uuid, recipient: Uuid, money: Money) -> Transaction {
        Transaction::Payment {
            sender: sender,
            recipient: recipient,
            amount: money,
            date: chrono::UTC::now(),
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
