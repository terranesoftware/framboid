use serde::{Deserialize, Serialize};

use crate::keys::Key;

/// An account identifier.
#[derive(Serialize, Deserialize)]
pub struct AccountKey(String);

impl Key for AccountKey {
    /// Creates an `AccountKey`.
    fn new(key: String) -> Self {
        AccountKey(key)
    }

    /// Returns a reference to the contained key.
    fn key(&self) -> &str {
        &self.0
    }
}