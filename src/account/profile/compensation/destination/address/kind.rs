use serde::{Deserialize, Serialize};

use crate::account::profile::compensation::destination::address::account::Account;

/// The kind of an `Address`.
#[derive(Serialize, Deserialize)]
pub enum AddressKind {
    Ach {
        account: String,
        kind: Account,
        routing: String,
    },
    Iban(String),
    Pix(String),
    Upi(String)
}