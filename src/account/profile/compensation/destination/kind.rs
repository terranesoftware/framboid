use serde::{Deserialize, Serialize};

use crate::account::profile::compensation::destination::address::Address;

/// The kind of a `Destination`.
#[derive(Serialize, Deserialize)]
pub enum DestinationKind {
    Default(Address),
    Retirement(Address)
}