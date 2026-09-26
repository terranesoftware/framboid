use serde::{Deserialize, Serialize};

/// The kind of a `Status`.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum StatusKind {
    Completed,
    Enrolled,
    DroppedOut,
    Transferred,
    Withdrawn
}