pub mod kind;
pub mod status;

use serde::{Deserialize, Serialize};

use crate::account::profile::{credentials::{kind::CredentialKind, status::Status}, location::Location};

/// A person's credential.
#[derive(Serialize, Deserialize)]
pub struct Credential(CredentialKind);

impl Credential {
    /// Creates a `Credential` with an education `CredentialKind`.
    pub fn education(
        degree: String,
        discipline: String,
        location: Location,
        school: String,
        status: Status
    ) -> Self {
        Self(
            CredentialKind::Education {
                degree,
                discipline,
                location,
                school,
                status
            }
        )
    }

    /// Returns a reference to the contained `CredentialKind`.
    pub fn kind(&self) -> &CredentialKind {
        &self.0
    }
}