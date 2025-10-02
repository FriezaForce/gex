pub mod manager;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub email: String,
    pub ssh_key_name: String,
}

impl Profile {
    /// Create a new profile
    pub fn new(name: String, username: String, email: String, ssh_key_name: String) -> Self {
        Self {
            name,
            username,
            email,
            ssh_key_name,
        }
    }

    /// Get the SSH host identifier for this profile
    pub fn ssh_host(&self) -> String {
        format!("github.com-{}", self.name)
    }
}
