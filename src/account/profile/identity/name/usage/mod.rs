pub mod kind;

use serde::{Deserialize, Serialize};

use crate::account::profile::{identity::name::usage::kind::UsageKind, period::Period};

/// A way a `Name` is or was used.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Usage(UsageKind);

impl Usage {
    /// Creates a `Usage` with a legal `UsageKind`.
    pub fn legal() -> Self {
        Self(UsageKind::Legal)
    }

    /// Creates a `Usage` with a prior `UsageKind`.
    pub fn prior(period: Period) -> Self {
        Self(UsageKind::Prior(period))
    }

    /// Creates a `Usage` with a used `UsageKind`.
    pub fn used() -> Self {
        Self(UsageKind::Used)
    }

    /// Returns a copy of the contained `UsageKind`.
    pub fn kind(&self) -> UsageKind {
        self.0
    }
}