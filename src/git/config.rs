use crate::error::{ProfileError, Result};
use crate::git::executor::execute_git;
use crate::git::ConfigScope;
use crate::profile::Profile;
use std::path::Path;

pub struct GitConfigManager;

impl GitConfigManager {
    /// Set a git config value for the specified scope
    pub fn set_config(scope: ConfigScope, key: &str, value: &str) -> Result<()> {
        let scope_flag = scope.as_flag();
        execute_git(&["config", scope_flag, key, value])?;
        Ok(())
    }

    /// Get a git config value for the specified scope
    pub fn get_config(scope: ConfigScope, key: &str) -> Result<Option<String>> {
        let scope_flag = scope.as_flag();
        match execute_git(&["config", scope_flag, key]) {
            Ok(value) => Ok(Some(value)),
            Err(ProfileError::InvalidInput(_)) => Ok(None), // Key not found
            Err(e) => Err(e),
        }
    }

    /// Check if the current directory is a git repository
    pub fn is_git_repository() -> Result<bool> {
        Ok(Path::new(".git").exists())
    }

    /// Get the current profile information from git config
    pub fn get_current_profile(scope: ConfigScope) -> Result<Option<(String, String)>> {
        let username = Self::get_config(scope, "user.name")?;
        let email = Self::get_config(scope, "user.email")?;

        match (username, email) {
            (Some(u), Some(e)) => Ok(Some((u, e))),
            _ => Ok(None),
        }
    }

    /// Apply a profile's git configuration
    pub fn apply_profile(profile: &Profile, scope: ConfigScope) -> Result<()> {
        // Check if we're in a git repo for local scope
        if scope == ConfigScope::Local && !Self::is_git_repository()? {
            return Err(ProfileError::NotGitRepo);
        }

        // Set user.name
        Self::set_config(scope, "user.name", &profile.username)?;

        // Set user.email
        Self::set_config(scope, "user.email", &profile.email)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::executor::is_git_installed;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_temp_git_repo() -> std::path::PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("gex_git_test_{}", timestamp));
        fs::create_dir_all(&temp_dir).unwrap();

        // Initialize git repo
        std::env::set_current_dir(&temp_dir).unwrap();
        execute_git(&["init"]).unwrap();

        temp_dir
    }

    fn cleanup_temp_dir(temp_dir: &std::path::PathBuf) {
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(temp_dir);
        }
    }

    #[test]
    fn test_set_and_get_config_global() {
        if !is_git_installed() {
            return;
        }

        // Set a test config value
        let result = GitConfigManager::set_config(
            ConfigScope::Global,
            "gex.test.value",
            "test123",
        );
        assert!(result.is_ok());

        // Get it back
        let value = GitConfigManager::get_config(ConfigScope::Global, "gex.test.value");
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), Some("test123".to_string()));

        // Cleanup - unset the test value
        let _ = execute_git(&["config", "--global", "--unset", "gex.test.value"]);
    }

    #[test]
    fn test_get_config_not_found() {
        if !is_git_installed() {
            return;
        }

        let value = GitConfigManager::get_config(
            ConfigScope::Global,
            "gex.nonexistent.key.that.does.not.exist",
        );
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), None);
    }

    #[test]
    fn test_is_git_repository() {
        if !is_git_installed() {
            return;
        }

        let original_dir = std::env::current_dir().unwrap();
        let temp_dir = create_temp_git_repo();

        // Should be true in git repo
        assert!(GitConfigManager::is_git_repository().unwrap());

        // Cleanup
        std::env::set_current_dir(&original_dir).unwrap();
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_get_current_profile() {
        if !is_git_installed() {
            return;
        }

        let original_dir = std::env::current_dir().unwrap();
        let temp_dir = create_temp_git_repo();

        // Ensure we're in the temp directory
        std::env::set_current_dir(&temp_dir).unwrap();

        // Set some config
        GitConfigManager::set_config(ConfigScope::Local, "user.name", "testuser").unwrap();
        GitConfigManager::set_config(ConfigScope::Local, "user.email", "test@example.com")
            .unwrap();

        // Get current profile
        let profile = GitConfigManager::get_current_profile(ConfigScope::Local).unwrap();
        assert!(profile.is_some());

        let (username, email) = profile.unwrap();
        assert_eq!(username, "testuser");
        assert_eq!(email, "test@example.com");

        // Cleanup
        std::env::set_current_dir(&original_dir).unwrap();
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_apply_profile_local() {
        if !is_git_installed() {
            return;
        }

        let original_dir = std::env::current_dir().unwrap();
        let temp_dir = create_temp_git_repo();

        // Ensure we're in the temp directory
        std::env::set_current_dir(&temp_dir).unwrap();

        let profile = Profile {
            name: "test".to_string(),
            username: "john-doe".to_string(),
            email: "john@example.com".to_string(),
            ssh_key_name: "id_rsa".to_string(),
        };

        let result = GitConfigManager::apply_profile(&profile, ConfigScope::Local);
        assert!(result.is_ok());

        // Verify the config was set
        let username = GitConfigManager::get_config(ConfigScope::Local, "user.name").unwrap();
        let email = GitConfigManager::get_config(ConfigScope::Local, "user.email").unwrap();

        assert_eq!(username, Some("john-doe".to_string()));
        assert_eq!(email, Some("john@example.com".to_string()));

        // Cleanup
        std::env::set_current_dir(&original_dir).unwrap();
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_apply_profile_local_not_git_repo() {
        if !is_git_installed() {
            return;
        }

        let original_dir = std::env::current_dir().unwrap();

        // Create temp dir without git
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("gex_nogit_test_{}", timestamp));
        fs::create_dir_all(&temp_dir).unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let profile = Profile {
            name: "test".to_string(),
            username: "john-doe".to_string(),
            email: "john@example.com".to_string(),
            ssh_key_name: "id_rsa".to_string(),
        };

        let result = GitConfigManager::apply_profile(&profile, ConfigScope::Local);
        assert!(result.is_err());

        match result {
            Err(ProfileError::NotGitRepo) => {
                // Expected error
            }
            _ => panic!("Expected NotGitRepo error"),
        }

        // Cleanup
        std::env::set_current_dir(&original_dir).unwrap();
        cleanup_temp_dir(&temp_dir);
    }
}
