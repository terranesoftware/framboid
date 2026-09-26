use serde::{Deserialize, Serialize};

/// The kind of an `Account`.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum AccountKind {
    Checking,
    Savings
}