use std::fs;
use std::path::PathBuf;
use crate::error::{ProfileError, Result};
use crate::storage::StorageData;

pub struct StorageService {
    pub(crate) config_path: PathBuf,
}

impl StorageService {
    /// Create a new StorageService instance
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        Ok(Self { config_path })
    }

    /// Get the platform-specific config file path
    pub fn get_config_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| ProfileError::PermissionDenied("Could not determine home directory".to_string()))?;
        
        let config_dir = home_dir.join(".github-profile-switcher");
        let config_file = config_dir.join("profiles.json");
        
        Ok(config_file)
    }

    /// Ensure the config directory and file exist
    pub fn ensure_config_exists(&self) -> Result<()> {
        // Get the parent directory (config directory)
        if let Some(parent) = self.config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| ProfileError::PermissionDenied(
                        format!("Failed to create config directory: {}", e)
                    ))?;
            }
        }

        // Create the config file if it doesn't exist
        if !self.config_path.exists() {
            let initial_data = StorageData::new();
            self.save(&initial_data)?;
        }

        Ok(())
    }

    /// Load profile data from the config file
    pub fn load(&self) -> Result<StorageData> {
        // Ensure config exists before loading
        self.ensure_config_exists()?;

        // Read the file
        let contents = fs::read_to_string(&self.config_path)
            .map_err(|e| ProfileError::PermissionDenied(
                format!("Failed to read config file: {}", e)
            ))?;

        // Parse JSON
        let data: StorageData = serde_json::from_str(&contents)
            .map_err(|_| ProfileError::ConfigCorrupted)?;

        Ok(data)
    }

    /// Save profile data to the config file
    pub fn save(&self, data: &StorageData) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = self.config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| ProfileError::PermissionDenied(
                        format!("Failed to create config directory: {}", e)
                    ))?;
            }
        }

        // Serialize to pretty JSON
        let json = serde_json::to_string_pretty(data)?;

        // Write to file
        fs::write(&self.config_path, json)
            .map_err(|e| ProfileError::PermissionDenied(
                format!("Failed to write config file: {}", e)
            ))?;

        Ok(())
    }

    /// Validate the config file structure
    pub fn validate_config(&self) -> Result<bool> {
        if !self.config_path.exists() {
            return Ok(false);
        }

        // Try to load and parse the config
        match self.load() {
            Ok(_) => Ok(true),
            Err(ProfileError::ConfigCorrupted) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Get the config file path
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::Profile;
    use std::fs;
    use std::path::PathBuf;

    // Helper to create a temporary test directory
    fn create_temp_service() -> (StorageService, PathBuf) {
        use std::time::{SystemTime, UNIX_EPOCH};
        
        // Create unique temp directory using timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("gex_test_{}", timestamp));
        
        // Create the directory
        fs::create_dir_all(&temp_dir).unwrap();
        
        let config_path = temp_dir.join("profiles.json");
        
        let service = StorageService {
            config_path: config_path.clone(),
        };
        
        (service, temp_dir)
    }

    // Helper to cleanup test directory
    fn cleanup_temp_dir(temp_dir: &PathBuf) {
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(temp_dir);
        }
    }

    #[test]
    fn test_save_and_load_empty_profiles() {
        let (service, temp_dir) = create_temp_service();
        
        // Create empty storage data
        let data = StorageData::new();
        
        // Save it
        let save_result = service.save(&data);
        assert!(save_result.is_ok(), "Failed to save: {:?}", save_result.err());
        
        // Load it back
        let loaded_result = service.load();
        assert!(loaded_result.is_ok(), "Failed to load: {:?}", loaded_result.err());
        
        let loaded_data = loaded_result.unwrap();
        assert_eq!(loaded_data.version, "1.0.0");
        assert_eq!(loaded_data.profiles.len(), 0);
        
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_save_and_load_with_profiles() {
        let (service, temp_dir) = create_temp_service();
        
        // Create storage data with profiles
        let mut data = StorageData::new();
        data.profiles.push(Profile {
            name: "personal".to_string(),
            username: "john-doe".to_string(),
            email: "john@personal.com".to_string(),
            ssh_key_name: "id_rsa_personal".to_string(),
        });
        data.profiles.push(Profile {
            name: "work".to_string(),
            username: "john-work".to_string(),
            email: "john@company.com".to_string(),
            ssh_key_name: "id_ed25519_work".to_string(),
        });
        
        // Save it
        assert!(service.save(&data).is_ok());
        
        // Load it back
        let loaded_data = service.load().unwrap();
        assert_eq!(loaded_data.profiles.len(), 2);
        assert_eq!(loaded_data.profiles[0].name, "personal");
        assert_eq!(loaded_data.profiles[0].username, "john-doe");
        assert_eq!(loaded_data.profiles[1].name, "work");
        assert_eq!(loaded_data.profiles[1].email, "john@company.com");
        
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_ensure_config_exists_creates_file() {
        let (service, temp_dir) = create_temp_service();
        
        // Ensure file doesn't exist
        assert!(!service.config_path.exists());
        
        // Call ensure_config_exists
        assert!(service.ensure_config_exists().is_ok());
        
        // File should now exist
        assert!(service.config_path.exists());
        
        // Should be able to load it
        let loaded_data = service.load().unwrap();
        assert_eq!(loaded_data.profiles.len(), 0);
        
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_load_corrupted_json() {
        let (service, temp_dir) = create_temp_service();
        
        // Create directory
        if let Some(parent) = service.config_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        
        // Write invalid JSON
        fs::write(&service.config_path, "{ invalid json }").unwrap();
        
        // Try to load - should return ConfigCorrupted error
        let result = service.load();
        assert!(result.is_err());
        
        if let Err(ProfileError::ConfigCorrupted) = result {
            // Expected error
        } else {
            panic!("Expected ConfigCorrupted error, got: {:?}", result);
        }
        
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_validate_config_with_valid_file() {
        let (service, temp_dir) = create_temp_service();
        
        // Create valid config
        let data = StorageData::new();
        service.save(&data).unwrap();
        
        // Validate should return true
        let is_valid = service.validate_config().unwrap();
        assert!(is_valid);
        
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_validate_config_with_missing_file() {
        let (service, temp_dir) = create_temp_service();
        
        // File doesn't exist
        assert!(!service.config_path.exists());
        
        // Validate should return false
        let is_valid = service.validate_config().unwrap();
        assert!(!is_valid);
        
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_validate_config_with_corrupted_file() {
        let (service, temp_dir) = create_temp_service();
        
        // Create directory and write invalid JSON
        if let Some(parent) = service.config_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&service.config_path, "corrupted").unwrap();
        
        // Validate should return false
        let is_valid = service.validate_config().unwrap();
        assert!(!is_valid);
        
        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_json_formatting() {
        let (service, temp_dir) = create_temp_service();
        
        // Create and save data
        let mut data = StorageData::new();
        data.profiles.push(Profile {
            name: "test".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            ssh_key_name: "id_rsa".to_string(),
        });
        
        service.save(&data).unwrap();
        
        // Read the raw file content
        let content = fs::read_to_string(&service.config_path).unwrap();
        
        // Should be pretty-printed (contains newlines and indentation)
        assert!(content.contains('\n'));
        assert!(content.contains("  ")); // Indentation
        assert!(content.contains("\"version\""));
        assert!(content.contains("\"profiles\""));
        
        cleanup_temp_dir(&temp_dir);
    }
}
