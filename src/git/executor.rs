use crate::error::{ProfileError, Result};
use std::process::Command;

/// Execute a git command with the given arguments
pub fn execute_git(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ProfileError::GitNotInstalled
            } else {
                ProfileError::Io(e)
            }
        })?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(stdout)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(ProfileError::InvalidInput(format!("Git command failed: {}", stderr)))
    }
}

/// Check if git is installed and available in PATH
pub fn is_git_installed() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Get the installed git version
pub fn get_git_version() -> Result<String> {
    execute_git(&["--version"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_installed() {
        // This test assumes git is installed on the system
        // If git is not installed, this test will fail
        let installed = is_git_installed();
        assert!(installed, "Git should be installed for tests to run");
    }

    #[test]
    fn test_get_git_version() {
        // Skip if git is not installed
        if !is_git_installed() {
            return;
        }

        let version = get_git_version();
        assert!(version.is_ok());
        
        let version_str = version.unwrap();
        assert!(version_str.contains("git version"));
    }

    #[test]
    fn test_execute_git_success() {
        // Skip if git is not installed
        if !is_git_installed() {
            return;
        }

        let result = execute_git(&["--version"]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_git_invalid_command() {
        // Skip if git is not installed
        if !is_git_installed() {
            return;
        }

        let result = execute_git(&["invalid-command-that-does-not-exist"]);
        assert!(result.is_err());
    }
}
