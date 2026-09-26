use serde::{Deserialize, Serialize};

use crate::account::profile::{credentials::status::Status, location::Location};

/// The kind of a `Credential`.
#[derive(Serialize, Deserialize)]
pub enum CredentialKind {
    Education {
        degree: String,
        discipline: String,
        location: Location,
        school: String,
        status: Status
    }
}