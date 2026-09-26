pub mod document;

use serde::{Deserialize, Serialize};

use crate::account::profile::authorization::document::Document;

/// A person's set of authorizing documents.
#[derive(Serialize, Deserialize)]
pub struct Authorization(Vec<Document>);

impl Authorization {
    /// Creates an `Authorization`.
    pub fn new(documents: Vec<Document>) -> Self {
        Self(documents)
    }

    /// Returns a reference to the contained documents.
    pub fn documents(&self) -> &[Document] {
        &self.0
    }
}