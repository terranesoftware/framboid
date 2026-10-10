pub mod working;

use blake3::Hash;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Timestamp;

use crate::addressing::Action;

/// A slice of time deemed meaningful.
#[derive(Serialize, Deserialize)]
pub struct Event {
    actions: IndexMap<Hash, Action>,
    pub(super) since: Timestamp,
    until: Timestamp
}

impl Event {
    /// Returns a reference to the contained actions.
    pub fn actions(&self) -> &IndexMap<Hash, Action> {
        &self.actions
    }

    /// Returns a copy of the contained since `Timestamp`.
    pub fn since(&self) -> Timestamp {
        self.since
    }

    /// Returns a copy of the contained until `Timestamp`.
    pub fn until(&self) -> Timestamp {
        self.until
    }
}