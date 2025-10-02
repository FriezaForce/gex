use crate::error::{ProfileError, Result};
use crate::profile::Profile;
use crate::storage::service::StorageService;

pub struct ProfileManager {
    pub(crate) storage: StorageService,
}

impl ProfileManager {
    /// Create a new ProfileManager instance
    pub fn new() -> Result<Self> {
        let storage = StorageService::new()?;
        Ok(Self { storage })
    }

    /// Create a new profile
    pub fn create_profile(&mut self, profile: Profile) -> Result<()> {
        // Check if profile already exists
        if self.profile_exists(&profile.name)? {
            return Err(ProfileError::ProfileExists(profile.name.clone()));
        }

        // Load current data
        let mut data = self.storage.load()?;

        // Add the new profile
        data.profiles.push(profile);
        data.touch();

        // Save back to storage
        self.storage.save(&data)?;

        Ok(())
    }

    /// Get a profile by name
    pub fn get_profile(&self, name: &str) -> Result<Option<Profile>> {
        let data = self.storage.load()?;

        let profile = data
            .profiles
            .into_iter()
            .find(|p| p.name == name);

        Ok(profile)
    }

    /// Get all profiles
    pub fn get_all_profiles(&self) -> Result<Vec<Profile>> {
        let data = self.storage.load()?;
        Ok(data.profiles)
    }

    /// Update an existing profile
    pub fn update_profile(&mut self, name: &str, updated_profile: Profile) -> Result<()> {
        // Load current data
        let mut data = self.storage.load()?;

        // Find the profile to update
        let profile_index = data
            .profiles
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| ProfileError::ProfileNotFound(name.to_string()))?;

        // Update the profile
        data.profiles[profile_index] = updated_profile;
        data.touch();

        // Save back to storage
        self.storage.save(&data)?;

        Ok(())
    }

    /// Delete a profile
    pub fn delete_profile(&mut self, name: &str) -> Result<()> {
        // Load current data
        let mut data = self.storage.load()?;

        // Find the profile to delete
        let profile_index = data
            .profiles
            .iter()
            .position(|p| p.name == name)
            .ok_or_else(|| ProfileError::ProfileNotFound(name.to_string()))?;

        // Remove the profile
        data.profiles.remove(profile_index);
        data.touch();

        // Save back to storage
        self.storage.save(&data)?;

        Ok(())
    }

    /// Check if a profile exists
    pub fn profile_exists(&self, name: &str) -> Result<bool> {
        let data = self.storage.load()?;
        Ok(data.profiles.iter().any(|p| p.name == name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Helper to create a ProfileManager with a temporary storage location
    fn create_test_manager() -> (ProfileManager, PathBuf) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("gex_profile_test_{}", timestamp));
        fs::create_dir_all(&temp_dir).unwrap();

        let config_path = temp_dir.join("profiles.json");
        let storage = StorageService {
            config_path: config_path.clone(),
        };

        let manager = ProfileManager { storage };
        (manager, temp_dir)
    }

    fn cleanup_temp_dir(temp_dir: &PathBuf) {
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(temp_dir);
        }
    }

    fn create_test_profile(name: &str) -> Profile {
        Profile {
            name: name.to_string(),
            username: format!("{}-user", name),
            email: format!("{}@example.com", name),
            ssh_key_name: format!("id_rsa_{}", name),
        }
    }

    #[test]
    fn test_create_profile_success() {
        let (mut manager, temp_dir) = create_test_manager();

        let profile = create_test_profile("personal");
        let result = manager.create_profile(profile);

        assert!(result.is_ok());

        // Verify profile was saved
        let profiles = manager.get_all_profiles().unwrap();
        assert_eq!(profiles.len(), 1);
        assert_eq!(profiles[0].name, "personal");

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_create_duplicate_profile() {
        let (mut manager, temp_dir) = create_test_manager();

        let profile1 = create_test_profile("work");
        manager.create_profile(profile1).unwrap();

        // Try to create duplicate
        let profile2 = create_test_profile("work");
        let result = manager.create_profile(profile2);

        assert!(result.is_err());
        match result {
            Err(ProfileError::ProfileExists(name)) => {
                assert_eq!(name, "work");
            }
            _ => panic!("Expected ProfileExists error"),
        }

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_get_profile_exists() {
        let (mut manager, temp_dir) = create_test_manager();

        let profile = create_test_profile("personal");
        manager.create_profile(profile).unwrap();

        let retrieved = manager.get_profile("personal").unwrap();
        assert!(retrieved.is_some());

        let profile = retrieved.unwrap();
        assert_eq!(profile.name, "personal");
        assert_eq!(profile.username, "personal-user");
        assert_eq!(profile.email, "personal@example.com");

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_get_profile_not_found() {
        let (manager, temp_dir) = create_test_manager();

        let retrieved = manager.get_profile("nonexistent").unwrap();
        assert!(retrieved.is_none());

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_get_all_profiles() {
        let (mut manager, temp_dir) = create_test_manager();

        // Create multiple profiles
        manager.create_profile(create_test_profile("personal")).unwrap();
        manager.create_profile(create_test_profile("work")).unwrap();
        manager.create_profile(create_test_profile("opensource")).unwrap();

        let profiles = manager.get_all_profiles().unwrap();
        assert_eq!(profiles.len(), 3);

        let names: Vec<String> = profiles.iter().map(|p| p.name.clone()).collect();
        assert!(names.contains(&"personal".to_string()));
        assert!(names.contains(&"work".to_string()));
        assert!(names.contains(&"opensource".to_string()));

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_get_all_profiles_empty() {
        let (manager, temp_dir) = create_test_manager();

        let profiles = manager.get_all_profiles().unwrap();
        assert_eq!(profiles.len(), 0);

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_update_profile_success() {
        let (mut manager, temp_dir) = create_test_manager();

        // Create initial profile
        let profile = create_test_profile("personal");
        manager.create_profile(profile).unwrap();

        // Update the profile
        let updated_profile = Profile {
            name: "personal".to_string(),
            username: "new-username".to_string(),
            email: "newemail@example.com".to_string(),
            ssh_key_name: "id_ed25519_new".to_string(),
        };

        let result = manager.update_profile("personal", updated_profile);
        assert!(result.is_ok());

        // Verify the update
        let retrieved = manager.get_profile("personal").unwrap().unwrap();
        assert_eq!(retrieved.username, "new-username");
        assert_eq!(retrieved.email, "newemail@example.com");
        assert_eq!(retrieved.ssh_key_name, "id_ed25519_new");

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_update_profile_not_found() {
        let (mut manager, temp_dir) = create_test_manager();

        let updated_profile = create_test_profile("nonexistent");
        let result = manager.update_profile("nonexistent", updated_profile);

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
    fn test_delete_profile_success() {
        let (mut manager, temp_dir) = create_test_manager();

        // Create profiles
        manager.create_profile(create_test_profile("personal")).unwrap();
        manager.create_profile(create_test_profile("work")).unwrap();

        // Delete one
        let result = manager.delete_profile("personal");
        assert!(result.is_ok());

        // Verify deletion
        let profiles = manager.get_all_profiles().unwrap();
        assert_eq!(profiles.len(), 1);
        assert_eq!(profiles[0].name, "work");

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_delete_profile_not_found() {
        let (mut manager, temp_dir) = create_test_manager();

        let result = manager.delete_profile("nonexistent");

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
    fn test_profile_exists() {
        let (mut manager, temp_dir) = create_test_manager();

        manager.create_profile(create_test_profile("personal")).unwrap();

        assert!(manager.profile_exists("personal").unwrap());
        assert!(!manager.profile_exists("nonexistent").unwrap());

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_multiple_operations() {
        let (mut manager, temp_dir) = create_test_manager();

        // Create
        manager.create_profile(create_test_profile("profile1")).unwrap();
        manager.create_profile(create_test_profile("profile2")).unwrap();
        manager.create_profile(create_test_profile("profile3")).unwrap();

        // Update
        let updated = Profile {
            name: "profile2".to_string(),
            username: "updated-user".to_string(),
            email: "updated@example.com".to_string(),
            ssh_key_name: "id_rsa_updated".to_string(),
        };
        manager.update_profile("profile2", updated).unwrap();

        // Delete
        manager.delete_profile("profile1").unwrap();

        // Verify final state
        let profiles = manager.get_all_profiles().unwrap();
        assert_eq!(profiles.len(), 2);

        let profile2 = manager.get_profile("profile2").unwrap().unwrap();
        assert_eq!(profile2.username, "updated-user");

        assert!(manager.get_profile("profile1").unwrap().is_none());
        assert!(manager.get_profile("profile3").unwrap().is_some());

        cleanup_temp_dir(&temp_dir);
    }
}
