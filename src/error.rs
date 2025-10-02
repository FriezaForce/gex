use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("Profile '{0}' not found")]
    ProfileNotFound(String),

    #[error("Profile '{0}' already exists")]
    ProfileExists(String),

    #[error("SSH key not found: {0}")]
    SshKeyNotFound(String),

    #[error("Not a git repository")]
    NotGitRepo,

    #[error("Git is not installed")]
    GitNotInstalled,

    #[error("Configuration file is corrupted")]
    ConfigCorrupted,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

impl ProfileError {
    /// Get a user-friendly error message with suggestions
    pub fn with_suggestion(&self) -> String {
        match self {
            ProfileError::ProfileNotFound(name) => {
                format!(
                    "Profile '{}' not found\n\n💡 Suggestion: Run 'gex list' to see available profiles\n   Or create it with: gex add {} --username <user> --email <email> --ssh-key <key>",
                    name, name
                )
            }
            ProfileError::ProfileExists(name) => {
                format!(
                    "Profile '{}' already exists\n\n💡 Suggestion: Use 'gex edit {}' to modify it or choose a different name",
                    name, name
                )
            }
            ProfileError::SshKeyNotFound(path) => {
                format!(
                    "SSH key not found: {}\n\n💡 Suggestions:\n   • Check if the SSH key exists at the expected location\n   • Generate a new SSH key:\n     ssh-keygen -t ed25519 -f ~/.ssh/your_key_name\n   • Update the profile with the correct key name:\n     gex edit <profile>",
                    path
                )
            }
            ProfileError::NotGitRepo => {
                "Not a git repository\n\n💡 Suggestion: Use --global flag to set the profile globally:\n   gex switch <profile> --global\n\n   Or run this command inside a git repository for local configuration".to_string()
            }
            ProfileError::GitNotInstalled => {
                "Git is not installed or not found in PATH\n\n💡 Suggestion: Install git from https://git-scm.com/downloads\n   After installation, restart your terminal".to_string()
            }
            ProfileError::ConfigCorrupted => {
                "Configuration file is corrupted\n\n💡 Suggestions:\n   • Backup the config file (if needed)\n   • Delete the config file to start fresh:\n     Windows: del %USERPROFILE%\\.github-profile-switcher\\profiles.json\n     Linux/Mac: rm ~/.github-profile-switcher/profiles.json\n   • Or manually fix the JSON syntax in the config file".to_string()
            }
            ProfileError::PermissionDenied(path) => {
                format!(
                    "Permission denied: {}\n\n💡 Suggestions:\n   • Check file permissions\n   • Ensure you have write access to the directory\n   • Try running with appropriate permissions",
                    path
                )
            }
            ProfileError::InvalidInput(msg) => {
                format!("Invalid input: {}\n\n💡 Tip: Use 'gex <command> --help' for usage information", msg)
            }
            ProfileError::Io(err) => {
                format!("IO error: {}\n\n💡 Tip: Check file permissions and disk space", err)
            }
            ProfileError::Json(err) => {
                format!("JSON parsing error: {}\n\n💡 Tip: The configuration file may be corrupted. Use 'gex list' to verify", err)
            }
        }
    }

    /// Check if this error should show suggestions
    pub fn should_show_suggestion(&self) -> bool {
        !matches!(self, ProfileError::Io(_) | ProfileError::Json(_))
    }
}

pub type Result<T> = std::result::Result<T, ProfileError>;
