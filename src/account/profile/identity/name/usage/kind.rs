use serde::{Deserialize, Serialize};

use crate::account::profile::period::Period;

/// The kind of a `Usage`.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum UsageKind {
    Legal,
    Prior(Period),
    Used
}