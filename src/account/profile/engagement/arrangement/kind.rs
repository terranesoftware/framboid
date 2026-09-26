use serde::{Deserialize, Serialize};

/// The kind of an `Arrangement`.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ArrangementKind {
    Hybrid,
    OnSite,
    Remote
}