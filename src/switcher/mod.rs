use crate::error::{ProfileError, Result};
use crate::git::config::GitConfigManager;
use crate::git::ConfigScope;
use crate::profile::manager::ProfileManager;
use crate::profile::Profile;
use crate::ssh::config::SSHConfigManager;

pub struct ProfileSwitcher {
    profile_manager: ProfileManager,
    ssh_config: SSHConfigManager,
}

#[derive(Debug)]
pub struct ProfileStatus {
    pub global: Option<Profile>,
    pub local: Option<Profile>,
}

impl ProfileSwitcher {
    /// Create a new ProfileSwitcher instance
    pub fn new() -> Result<Self> {
        let profile_manager = ProfileManager::new()?;
        let ssh_config = SSHConfigManager::new()?;

        Ok(Self {
            profile_manager,
            ssh_config,
        })
    }

    /// Switch to a profile with the specified scope
    pub fn switch_profile(&mut self, profile_name: &str, scope: ConfigScope) -> Result<()> {
        println!("Switching to profile '{}'...", profile_name);

        // 1. Validate profile exists
        println!("  ✓ Checking if profile exists...");
        let profile = self
            .profile_manager
            .get_profile(profile_name)?
            .ok_or_else(|| ProfileError::ProfileNotFound(profile_name.to_string()))?;

        // 2. Validate SSH key exists
        println!("  ✓ Validating SSH key...");
        if !SSHConfigManager::validate_ssh_key(&profile.ssh_key_name)? {
            let key_path = SSHConfigManager::get_ssh_key_path(&profile.ssh_key_name);
            return Err(ProfileError::SshKeyNotFound(
                key_path.to_string_lossy().to_string(),
            ));
        }

        // 3. Apply git config changes
        println!("  ✓ Updating git config ({})...", scope);
        GitConfigManager::apply_profile(&profile, scope)?;

        // 4. Update SSH config
        println!("  ✓ Updating SSH config...");
        self.ssh_config.add_or_update_host(&profile)?;

        println!("\n✓ Successfully switched to profile '{}'", profile_name);
        println!("  Username: {}", profile.username);
        println!("  Email: {}", profile.email);
        println!("  SSH Key: {}", profile.ssh_key_name);
        println!("  Scope: {}", scope);

        Ok(())
    }

    /// Get the current profile status for both global and local scopes
    pub fn get_current_status(&self) -> Result<ProfileStatus> {
        // Get global profile
        let global = match GitConfigManager::get_current_profile(ConfigScope::Global)? {
            Some((username, email)) => {
                // Try to find a matching profile
                self.find_profile_by_credentials(&username, &email)?
            }
            None => None,
        };

        // Get local profile (if in a git repo)
        let local = if GitConfigManager::is_git_repository()? {
            match GitConfigManager::get_current_profile(ConfigScope::Local)? {
                Some((username, email)) => {
                    // Try to find a matching profile
                    self.find_profile_by_credentials(&username, &email)?
                }
                None => None,
            }
        } else {
            None
        };

        Ok(ProfileStatus { global, local })
    }

    /// Find a profile by username and email
    fn find_profile_by_credentials(&self, username: &str, email: &str) -> Result<Option<Profile>> {
        let profiles = self.profile_manager.get_all_profiles()?;

        for profile in profiles {
            if profile.username == username && profile.email == email {
                return Ok(Some(profile));
            }
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::executor::is_git_installed;
    use crate::storage::service::StorageService;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Helper to create a test environment with temporary storage and SSH config
    fn create_test_environment() -> (ProfileSwitcher, std::path::PathBuf, std::path::PathBuf) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        
        let temp_dir = std::env::temp_dir().join(format!("gex_switcher_test_{}", timestamp));
        fs::create_dir_all(&temp_dir).unwrap();

        // Create temporary storage
        let storage_path = temp_dir.join("profiles.json");
        let storage = StorageService {
            config_path: storage_path.clone(),
        };

        // Create temporary SSH config
        let ssh_config_path = temp_dir.join("ssh_config");
        let ssh_config = SSHConfigManager {
            config_path: ssh_config_path.clone(),
        };

        let profile_manager = ProfileManager { storage };

        let switcher = ProfileSwitcher {
            profile_manager,
            ssh_config,
        };

        (switcher, temp_dir, ssh_config_path)
    }

    fn cleanup_temp_dir(temp_dir: &std::path::PathBuf) {
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(temp_dir);
        }
    }

    fn create_dummy_ssh_key(temp_dir: &std::path::PathBuf, key_name: &str) {
        let ssh_dir = temp_dir.join(".ssh");
        fs::create_dir_all(&ssh_dir).unwrap();
        let key_path = ssh_dir.join(key_name);
        fs::write(&key_path, "dummy key content").unwrap();
    }

    #[test]
    fn test_switch_profile_not_found() {
        let (mut switcher, temp_dir, _) = create_test_environment();

        let result = switcher.switch_profile("nonexistent", ConfigScope::Global);
        assert!(result.is_err());

        match result {
            Err(ProfileError::ProfileNotFound(name)) => {
                assert_eq!(name, "nonexistent");
            }
            _ => panic!("Expected ProfileNotFound error"),
        }

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_switch_profile_ssh_key_missing() {
        let (mut switcher, temp_dir, _) = create_test_environment();

        // Create a profile without creating the SSH key
        let profile = Profile {
            name: "test".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            ssh_key_name: "nonexistent_key".to_string(),
        };

        let _ = switcher
            .profile_manager
            .create_profile(profile);

        // Try to switch - should fail because SSH key doesn't exist
        let result = switcher.switch_profile("test", ConfigScope::Global);
        assert!(result.is_err());

        match result {
            Err(ProfileError::SshKeyNotFound(_)) => {
                // Expected error
            }
            _ => panic!("Expected SshKeyNotFound error"),
        }

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_get_current_status_no_profiles() {
        let (switcher, temp_dir, _) = create_test_environment();

        let status = switcher.get_current_status().unwrap();
        assert!(status.global.is_none());
        assert!(status.local.is_none());

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_find_profile_by_credentials() {
        let (mut switcher, temp_dir, _) = create_test_environment();

        // Create a profile
        let profile = Profile {
            name: "test".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            ssh_key_name: "id_rsa".to_string(),
        };

        let _ = switcher
            .profile_manager
            .create_profile(profile.clone());

        // Find by credentials
        let found = switcher
            .find_profile_by_credentials("testuser", "test@example.com")
            .unwrap();

        assert!(found.is_some());
        let found_profile = found.unwrap();
        assert_eq!(found_profile.name, "test");
        assert_eq!(found_profile.username, "testuser");

        // Try with wrong credentials
        let not_found = switcher
            .find_profile_by_credentials("wronguser", "wrong@example.com")
            .unwrap();
        assert!(not_found.is_none());

        cleanup_temp_dir(&temp_dir);
    }

    // Note: Full end-to-end tests that actually switch git config are skipped
    // because they would modify the user's actual git configuration.
    // These tests verify the orchestration logic without side effects.
}
