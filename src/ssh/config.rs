use crate::error::{ProfileError, Result};
use crate::profile::Profile;
use std::fs;
use std::path::PathBuf;

pub struct SSHConfigManager {
    pub(crate) config_path: PathBuf,
}

impl SSHConfigManager {
    /// Create a new SSHConfigManager instance
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| ProfileError::PermissionDenied("Could not determine home directory".to_string()))?;
        
        let config_path = home_dir.join(".ssh").join("config");
        
        Ok(Self { config_path })
    }

    /// Get the full path to an SSH key
    pub fn get_ssh_key_path(key_name: &str) -> PathBuf {
        let home_dir = dirs::home_dir().expect("Could not determine home directory");
        home_dir.join(".ssh").join(key_name)
    }

    /// Validate that an SSH key exists
    pub fn validate_ssh_key(key_name: &str) -> Result<bool> {
        let key_path = Self::get_ssh_key_path(key_name);
        Ok(key_path.exists())
    }

    /// Ensure the SSH config file exists
    pub fn ensure_ssh_config_exists(&self) -> Result<()> {
        // Ensure .ssh directory exists
        if let Some(parent) = self.config_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| ProfileError::PermissionDenied(
                        format!("Failed to create .ssh directory: {}", e)
                    ))?;
            }
        }

        // Create config file if it doesn't exist
        if !self.config_path.exists() {
            fs::write(&self.config_path, "")
                .map_err(|e| ProfileError::PermissionDenied(
                    format!("Failed to create SSH config file: {}", e)
                ))?;
        }

        Ok(())
    }

    /// Backup the SSH config file
    pub fn backup_ssh_config(&self) -> Result<()> {
        if self.config_path.exists() {
            let backup_path = self.config_path.with_extension("config.bak");
            fs::copy(&self.config_path, &backup_path)
                .map_err(|e| ProfileError::PermissionDenied(
                    format!("Failed to backup SSH config: {}", e)
                ))?;
        }
        Ok(())
    }

    /// Add or update a host entry for a profile
    pub fn add_or_update_host(&mut self, profile: &Profile) -> Result<()> {
        self.ensure_ssh_config_exists()?;
        self.backup_ssh_config()?;

        // Read existing config
        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| ProfileError::PermissionDenied(
                format!("Failed to read SSH config: {}", e)
            ))?;

        // Parse and update config
        let updated_content = self.update_config_content(&content, profile)?;

        // Write back
        fs::write(&self.config_path, updated_content)
            .map_err(|e| ProfileError::PermissionDenied(
                format!("Failed to write SSH config: {}", e)
            ))?;

        Ok(())
    }

    /// Remove a host entry for a profile
    pub fn remove_host(&mut self, profile_name: &str) -> Result<()> {
        if !self.config_path.exists() {
            return Ok(()); // Nothing to remove
        }

        self.backup_ssh_config()?;

        // Read existing config
        let content = fs::read_to_string(&self.config_path)
            .map_err(|e| ProfileError::PermissionDenied(
                format!("Failed to read SSH config: {}", e)
            ))?;

        // Remove the profile's host entry
        let updated_content = self.remove_host_from_content(&content, profile_name);

        // Write back
        fs::write(&self.config_path, updated_content)
            .map_err(|e| ProfileError::PermissionDenied(
                format!("Failed to write SSH config: {}", e)
            ))?;

        Ok(())
    }

    /// Update the config content with a new or updated host entry
    fn update_config_content(&self, content: &str, profile: &Profile) -> Result<String> {
        let host_marker = format!("# GitHub Profile: {}", profile.name);
        let host_name = format!("github.com-{}", profile.name);
        let key_path = Self::get_ssh_key_path(&profile.ssh_key_name);

        // Build the new host entry
        let new_entry = format!(
            "{}\nHost {}\n  HostName github.com\n  User git\n  IdentityFile {}\n  IdentitiesOnly yes\n",
            host_marker,
            host_name,
            key_path.display()
        );

        // Check if this profile already has an entry
        let lines: Vec<&str> = content.lines().collect();
        let mut result = String::new();
        let mut i = 0;

        while i < lines.len() {
            if lines[i] == host_marker {
                // Found existing entry, skip the entire block
                i += 1;
                
                // The next line should be the Host line - skip it and all its properties
                let mut in_host_block = false;
                while i < lines.len() {
                    let line = lines[i];
                    
                    // If this is the Host line for this block, mark that we're in it
                    if line.starts_with("Host ") && !in_host_block {
                        in_host_block = true;
                        i += 1;
                        continue;
                    }
                    
                    // If we're in the host block and hit an indented line, skip it
                    if in_host_block && (line.starts_with("  ") || line.trim().is_empty()) {
                        i += 1;
                        continue;
                    }
                    
                    // If we hit a comment or another Host line, we're done
                    if line.trim().starts_with('#') || line.starts_with("Host ") {
                        break;
                    }
                    
                    // Skip empty lines between blocks
                    if line.trim().is_empty() {
                        i += 1;
                        continue;
                    }
                    
                    // Anything else means we're done with this block
                    break;
                }
            } else {
                result.push_str(lines[i]);
                result.push('\n');
                i += 1;
            }
        }

        // Add the new entry at the end
        if !result.is_empty() && !result.ends_with("\n\n") {
            result.push('\n');
        }
        result.push_str(&new_entry);

        Ok(result)
    }

    /// Remove a host entry from the config content
    fn remove_host_from_content(&self, content: &str, profile_name: &str) -> String {
        let host_marker = format!("# GitHub Profile: {}", profile_name);
        let lines: Vec<&str> = content.lines().collect();
        let mut result = String::new();
        let mut i = 0;

        while i < lines.len() {
            if lines[i] == host_marker {
                // Found the entry to remove, skip the entire block
                i += 1;
                
                // The next line should be the Host line - skip it and all its properties
                let mut in_host_block = false;
                while i < lines.len() {
                    let line = lines[i];
                    
                    // If this is the Host line for this block, mark that we're in it
                    if line.starts_with("Host ") && !in_host_block {
                        in_host_block = true;
                        i += 1;
                        continue;
                    }
                    
                    // If we're in the host block and hit an indented line, skip it
                    if in_host_block && (line.starts_with("  ") || line.trim().is_empty()) {
                        i += 1;
                        continue;
                    }
                    
                    // If we hit a comment or another Host line, we're done
                    if line.trim().starts_with('#') || line.starts_with("Host ") {
                        break;
                    }
                    
                    // Skip empty lines between blocks
                    if line.trim().is_empty() {
                        i += 1;
                        continue;
                    }
                    
                    // Anything else means we're done with this block
                    break;
                }
            } else {
                result.push_str(lines[i]);
                result.push('\n');
                i += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_temp_ssh_manager() -> (SSHConfigManager, PathBuf) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("gex_ssh_test_{}", timestamp));
        fs::create_dir_all(&temp_dir).unwrap();

        let config_path = temp_dir.join("config");
        let manager = SSHConfigManager { config_path };

        (manager, temp_dir)
    }

    fn cleanup_temp_dir(temp_dir: &PathBuf) {
        if temp_dir.exists() {
            let _ = fs::remove_dir_all(temp_dir);
        }
    }

    #[test]
    fn test_get_ssh_key_path() {
        let path = SSHConfigManager::get_ssh_key_path("id_rsa");
        assert!(path.to_string_lossy().contains(".ssh"));
        assert!(path.to_string_lossy().contains("id_rsa"));
    }

    #[test]
    fn test_ensure_ssh_config_exists() {
        let (manager, temp_dir) = create_temp_ssh_manager();

        assert!(!manager.config_path.exists());

        let result = manager.ensure_ssh_config_exists();
        assert!(result.is_ok());
        assert!(manager.config_path.exists());

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_add_host() {
        let (mut manager, temp_dir) = create_temp_ssh_manager();

        let profile = Profile {
            name: "personal".to_string(),
            username: "john-doe".to_string(),
            email: "john@example.com".to_string(),
            ssh_key_name: "id_rsa_personal".to_string(),
        };

        let result = manager.add_or_update_host(&profile);
        assert!(result.is_ok());

        // Read the config and verify
        let content = fs::read_to_string(&manager.config_path).unwrap();
        assert!(content.contains("# GitHub Profile: personal"));
        assert!(content.contains("Host github.com-personal"));
        assert!(content.contains("HostName github.com"));
        assert!(content.contains("id_rsa_personal"));

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_update_existing_host() {
        let (mut manager, temp_dir) = create_temp_ssh_manager();

        // Add initial profile
        let profile1 = Profile {
            name: "work".to_string(),
            username: "john-work".to_string(),
            email: "john@work.com".to_string(),
            ssh_key_name: "id_rsa_work".to_string(),
        };
        manager.add_or_update_host(&profile1).unwrap();

        // Update with different SSH key
        let profile2 = Profile {
            name: "work".to_string(),
            username: "john-work".to_string(),
            email: "john@work.com".to_string(),
            ssh_key_name: "id_ed25519_work".to_string(),
        };
        manager.add_or_update_host(&profile2).unwrap();

        // Verify the update
        let content = fs::read_to_string(&manager.config_path).unwrap();
        assert!(content.contains("id_ed25519_work"));
        assert!(!content.contains("id_rsa_work"));

        // Should only have one entry for "work"
        let count = content.matches("# GitHub Profile: work").count();
        assert_eq!(count, 1);

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_remove_host() {
        let (mut manager, temp_dir) = create_temp_ssh_manager();

        // Add two profiles
        let profile1 = Profile {
            name: "personal".to_string(),
            username: "john".to_string(),
            email: "john@personal.com".to_string(),
            ssh_key_name: "id_rsa_personal".to_string(),
        };
        let profile2 = Profile {
            name: "work".to_string(),
            username: "john".to_string(),
            email: "john@work.com".to_string(),
            ssh_key_name: "id_rsa_work".to_string(),
        };

        manager.add_or_update_host(&profile1).unwrap();
        manager.add_or_update_host(&profile2).unwrap();

        // Remove one
        let result = manager.remove_host("personal");
        assert!(result.is_ok());

        // Verify removal
        let content = fs::read_to_string(&manager.config_path).unwrap();
        assert!(!content.contains("# GitHub Profile: personal"));
        assert!(!content.contains("github.com-personal"));
        assert!(content.contains("# GitHub Profile: work"));
        assert!(content.contains("github.com-work"));

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_backup_ssh_config() {
        let (manager, temp_dir) = create_temp_ssh_manager();

        // Create a config file
        fs::write(&manager.config_path, "test content").unwrap();

        // Backup
        let result = manager.backup_ssh_config();
        assert!(result.is_ok());

        // Verify backup exists
        let backup_path = manager.config_path.with_extension("config.bak");
        assert!(backup_path.exists());

        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, "test content");

        cleanup_temp_dir(&temp_dir);
    }

    #[test]
    fn test_preserve_existing_entries() {
        let (mut manager, temp_dir) = create_temp_ssh_manager();

        // Create config with existing non-GitHub entries
        let existing_config = "# My custom server\nHost myserver\n  HostName example.com\n  User admin\n";
        fs::write(&manager.config_path, existing_config).unwrap();

        // Add a GitHub profile
        let profile = Profile {
            name: "personal".to_string(),
            username: "john".to_string(),
            email: "john@example.com".to_string(),
            ssh_key_name: "id_rsa".to_string(),
        };
        manager.add_or_update_host(&profile).unwrap();

        // Verify existing entry is preserved
        let content = fs::read_to_string(&manager.config_path).unwrap();
        assert!(content.contains("# My custom server"));
        assert!(content.contains("Host myserver"));
        assert!(content.contains("# GitHub Profile: personal"));

        cleanup_temp_dir(&temp_dir);
    }
}
