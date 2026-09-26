pub mod kind;

use serde::{Deserialize, Serialize};

use crate::account::profile::compensation::destination::address::account::kind::AccountKind;

/// A type of bank account.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Account(AccountKind);

impl Account {
    /// Creates an `Account` with a checking `AccountKind`.
    pub fn checking() -> Self {
        Account(AccountKind::Checking)
    }

    /// Creates an `Account` with a savings `AccountKind`.
    pub fn savings() -> Self {
        Account(AccountKind::Savings)
    }

    /// Returns a copy of the contained `AccountKind`.
    pub fn kind(&self) -> AccountKind {
        self.0
    }
}