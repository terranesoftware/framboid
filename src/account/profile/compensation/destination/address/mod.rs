pub mod account;
pub mod kind;

use serde::{Deserialize, Serialize};

use crate::account::profile::compensation::destination::address::{account::Account, kind::AddressKind};

/// An address a payment is sent to.
#[derive(Serialize, Deserialize)]
pub struct Address(AddressKind);

impl Address {
    /// Creates an `Address` with an ACH `AddressKind`.
    pub fn ach(
        account: String,
        routing: String,
        kind: Account
    ) -> Self {
        Self(
            AddressKind::Ach {
                account,
                routing,
                kind
            }
        )
    }

    /// Creates an `Address` with an IBAN `AddressKind`.
    pub fn iban(iban: String) -> Self {
        Self(AddressKind::Iban(iban))
    }

    /// Creates an `Address` with a Pix `AddressKind`.
    pub fn pix(key: String) -> Self {
        Self(AddressKind::Pix(key))
    }

    /// Creates an `Address` with a UPI `AddressKind`.
    pub fn upi(vpa: String) -> Self {
        Self(AddressKind::Upi(vpa))
    }

    /// Returns a reference to the contained `AddressKind`.
    pub fn kind(&self) -> &AddressKind {
        &self.0
    }
}