pub mod address;
pub mod kind;

use serde::{Deserialize, Serialize};

use crate::account::profile::compensation::destination::{address::Address, kind::DestinationKind};

/// A destination for a person's compensation.
#[derive(Serialize, Deserialize)]
pub struct Destination(DestinationKind);

impl Destination {
    /// Creates a `Destination` with a default `DestinationKind`.
    pub fn default(address: Address) -> Self {
        Self(DestinationKind::Default(address))
    }

    /// Creates a `Destination` with a retirement `DestinationKind`.
    pub fn retirement(address: Address) -> Self {
        Self(DestinationKind::Retirement(address))
    }

    /// Returns a reference to the contained `DestinationKind`.
    pub fn kind(&self) -> &DestinationKind {
        &self.0
    }
}