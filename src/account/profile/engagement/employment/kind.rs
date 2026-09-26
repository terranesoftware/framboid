use serde::{Deserialize, Serialize};

/// The kind of an `Employment`.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EmploymentKind {
    Apprenticeship,
    Contract,
    Internship,
    FullTime,
    PartTime
}