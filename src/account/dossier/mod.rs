pub mod event;
pub mod working;

use blake3::Hash;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use time::Timestamp;

use crate::account::dossier::event::Event;

/// A bounded span of events with a coherent identity.
#[derive(Serialize, Deserialize)]
pub struct Dossier {
    pub(super) events: IndexMap<Hash, Event>,
    pub(super) since: Timestamp,
    until: Timestamp
}

impl Dossier {
    /// Returns a reference to the contained events.
    pub fn events(&self) -> &IndexMap<Hash, Event> {
        &self.events
    }

    /// Returns a copy of the since `Timestamp`.
    pub fn since(&self) -> Timestamp {
        self.since
    }

    /// Returns a copy of the until `Timestamp`.
    pub fn until(&self) -> Timestamp {
        self.until
    }
}