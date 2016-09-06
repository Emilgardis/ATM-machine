use currency::{Currency, IndexBill};

use uuid::Uuid;
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transaction<C: Currency> {
    // FIXME: Make generic with Box, or fix in another way
    Transfer {
        sender: Uuid,
        recipient: Uuid,
        amount: C,
        date: chrono::DateTime<chrono::UTC>,
    },
    Deposit {
        from: Uuid,
        amount: C,
        date: chrono::DateTime<chrono::UTC>,
    },
    Withdrawal {
        to: Uuid,
        amount: C,
        date: chrono::DateTime<chrono::UTC>,
    },
    Payment {
        sender: Uuid,
        recipient: Uuid,
        amount: C,
        date: chrono::DateTime<chrono::UTC>,
    }
}
