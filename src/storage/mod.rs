pub mod service;

use serde::{Deserialize, Serialize};
use crate::profile::Profile;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageData {
    pub version: String,
    pub profiles: Vec<Profile>,
    pub last_modified: String,
}

impl StorageData {
    /// Create a new empty storage data structure
    pub fn new() -> Self {
        Self {
            version: "1.0.0".to_string(),
            profiles: Vec::new(),
            last_modified: Utc::now().to_rfc3339(),
        }
    }

    /// Update the last modified timestamp
    pub fn touch(&mut self) {
        self.last_modified = Utc::now().to_rfc3339();
    }
}

impl Default for StorageData {
    fn default() -> Self {
        Self::new()
    }
}
