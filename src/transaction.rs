use currency::Money;

use uuid::Uuid;
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction {
    // FIXME: Make generic with Box, or fix in another way
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
    /// This is here more for bookkeeping. 
    // FIXME: Decide if this should be used in transactions and or pending_transactions
    Pending {
        transaction: Box<Transaction>,
        status: TransactionStatus,
        sent: chrono::DateTime<chrono::UTC>,
        responded: Option<chrono::DateTime<chrono::UTC>>,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    Completed,
    Declined,
}
