pub mod config;
pub mod executor;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigScope {
    Global,
    Local,
}

impl ConfigScope {
    /// Get the git config flag for this scope
    pub fn as_flag(&self) -> &str {
        match self {
            ConfigScope::Global => "--global",
            ConfigScope::Local => "--local",
        }
    }
}

impl fmt::Display for ConfigScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigScope::Global => write!(f, "global"),
            ConfigScope::Local => write!(f, "local"),
        }
    }
}
