pub mod kind;

use serde::{Deserialize, Serialize};

use crate::account::profile::credentials::status::kind::StatusKind;

/// An education's status.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Status(StatusKind);

impl Status {
    /// Creates a `Status` with a completed `StatusKind`.
    pub fn completed() -> Self {
        Self(StatusKind::Completed)
    }
    
    /// Creates a `Status` with an enrolled `StatusKind`.
    pub fn enrolled() -> Self {
        Self(StatusKind::Enrolled)
    }

    /// Creates a `Status` with a dropped-out `StatusKind`.
    pub fn dropped_out() -> Self {
        Self(StatusKind::DroppedOut)
    }

    /// Creates a `Status` with a transferred `StatusKind`.
    pub fn transferred() -> Self {
        Self(StatusKind::Transferred)
    }

    /// Creates a `Status` with a withdrawn `StatusKind`.
    pub fn withdrawn() -> Self {
        Self(StatusKind::Withdrawn)
    }

    /// Returns a copy of the contained `StatusKind`.
    pub fn kind(&self) -> StatusKind {
        self.0
    }
}