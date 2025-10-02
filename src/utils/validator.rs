use regex::Regex;

pub struct Validator;

impl Validator {
    /// Validate email address format
    /// Accepts standard email format: user@domain.tld
    pub fn validate_email(email: &str) -> bool {
        if email.is_empty() {
            return false;
        }

        // Simple but effective email regex
        // Matches: username@domain.extension
        let email_regex = Regex::new(
            r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        ).unwrap();

        email_regex.is_match(email)
    }

    /// Validate profile name
    /// Allows: alphanumeric characters, hyphens, and underscores
    /// Must be between 1 and 50 characters
    pub fn validate_profile_name(name: &str) -> bool {
        if name.is_empty() || name.len() > 50 {
            return false;
        }

        // Only allow alphanumeric, hyphens, and underscores
        let name_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
        name_regex.is_match(name)
    }

    /// Validate SSH key name
    /// Allows valid file name characters
    /// Common SSH key names: id_rsa, id_ed25519, id_ecdsa, etc.
    pub fn validate_ssh_key_name(key_name: &str) -> bool {
        if key_name.is_empty() || key_name.len() > 255 {
            return false;
        }

        // Disallow path separators and special characters that could cause issues
        let invalid_chars = ['/', '\\', '\0', '<', '>', ':', '"', '|', '?', '*'];
        
        for ch in invalid_chars.iter() {
            if key_name.contains(*ch) {
                return false;
            }
        }

        // Must not start or end with whitespace
        if key_name.trim() != key_name {
            return false;
        }

        true
    }

    /// Validate GitHub username
    /// GitHub usernames can contain alphanumeric characters and hyphens
    /// Cannot start or end with a hyphen
    /// Maximum 39 characters
    pub fn validate_username(username: &str) -> bool {
        if username.is_empty() || username.len() > 39 {
            return false;
        }

        // Cannot start or end with hyphen
        if username.starts_with('-') || username.ends_with('-') {
            return false;
        }

        // Only alphanumeric and hyphens allowed
        let username_regex = Regex::new(r"^[a-zA-Z0-9-]+$").unwrap();
        username_regex.is_match(username)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        // Valid emails
        assert!(Validator::validate_email("user@example.com"));
        assert!(Validator::validate_email("john.doe@company.co.uk"));
        assert!(Validator::validate_email("test+tag@domain.org"));
        assert!(Validator::validate_email("user123@test-domain.com"));

        // Invalid emails
        assert!(!Validator::validate_email(""));
        assert!(!Validator::validate_email("invalid"));
        assert!(!Validator::validate_email("@example.com"));
        assert!(!Validator::validate_email("user@"));
        assert!(!Validator::validate_email("user@domain"));
        assert!(!Validator::validate_email("user @domain.com"));
    }

    #[test]
    fn test_validate_profile_name() {
        // Valid profile names
        assert!(Validator::validate_profile_name("personal"));
        assert!(Validator::validate_profile_name("work-account"));
        assert!(Validator::validate_profile_name("my_profile"));
        assert!(Validator::validate_profile_name("profile123"));
        assert!(Validator::validate_profile_name("a"));

        // Invalid profile names
        assert!(!Validator::validate_profile_name(""));
        assert!(!Validator::validate_profile_name("profile with spaces"));
        assert!(!Validator::validate_profile_name("profile@special"));
        assert!(!Validator::validate_profile_name("profile.dot"));
        assert!(!Validator::validate_profile_name(&"a".repeat(51))); // Too long
    }

    #[test]
    fn test_validate_ssh_key_name() {
        // Valid SSH key names
        assert!(Validator::validate_ssh_key_name("id_rsa"));
        assert!(Validator::validate_ssh_key_name("id_ed25519"));
        assert!(Validator::validate_ssh_key_name("id_rsa_personal"));
        assert!(Validator::validate_ssh_key_name("my-key"));
        assert!(Validator::validate_ssh_key_name("key.pub"));

        // Invalid SSH key names
        assert!(!Validator::validate_ssh_key_name(""));
        assert!(!Validator::validate_ssh_key_name("key/name")); // Path separator
        assert!(!Validator::validate_ssh_key_name("key\\name")); // Path separator
        assert!(!Validator::validate_ssh_key_name("key:name")); // Invalid char
        assert!(!Validator::validate_ssh_key_name(" key")); // Leading space
        assert!(!Validator::validate_ssh_key_name("key ")); // Trailing space
        assert!(!Validator::validate_ssh_key_name(&"a".repeat(256))); // Too long
    }

    #[test]
    fn test_validate_username() {
        // Valid GitHub usernames
        assert!(Validator::validate_username("john-doe"));
        assert!(Validator::validate_username("user123"));
        assert!(Validator::validate_username("a"));
        assert!(Validator::validate_username("my-username-123"));

        // Invalid GitHub usernames
        assert!(!Validator::validate_username(""));
        assert!(!Validator::validate_username("-username")); // Starts with hyphen
        assert!(!Validator::validate_username("username-")); // Ends with hyphen
        assert!(!Validator::validate_username("user_name")); // Underscore not allowed
        assert!(!Validator::validate_username("user name")); // Space not allowed
        assert!(!Validator::validate_username("user@name")); // Special char
        assert!(!Validator::validate_username(&"a".repeat(40))); // Too long
    }
}
