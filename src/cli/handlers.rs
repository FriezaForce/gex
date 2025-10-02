use crate::error::Result;
use crate::git::ConfigScope;
use crate::profile::manager::ProfileManager;
use crate::profile::Profile;
use crate::switcher::ProfileSwitcher;
use crate::utils::validator::Validator;
use dialoguer::{Confirm, Input};

/// Handle the 'add' command to create a new profile
pub fn handle_add(
    name: String,
    username: String,
    email: String,
    ssh_key: String,
) -> Result<()> {
    println!("Creating new profile '{}'...", name);

    // Validate inputs
    if !Validator::validate_profile_name(&name) {
        return Err(crate::error::ProfileError::InvalidInput(
            "Profile name must contain only alphanumeric characters, hyphens, and underscores"
                .to_string(),
        ));
    }

    if !Validator::validate_username(&username) {
        return Err(crate::error::ProfileError::InvalidInput(
            "Invalid GitHub username format".to_string(),
        ));
    }

    if !Validator::validate_email(&email) {
        return Err(crate::error::ProfileError::InvalidInput(
            "Invalid email format".to_string(),
        ));
    }

    if !Validator::validate_ssh_key_name(&ssh_key) {
        return Err(crate::error::ProfileError::InvalidInput(
            "Invalid SSH key name".to_string(),
        ));
    }

    // Create the profile
    let mut manager = ProfileManager::new()?;
    let profile = Profile {
        name: name.clone(),
        username,
        email,
        ssh_key_name: ssh_key,
    };

    manager.create_profile(profile)?;

    println!("✓ Profile '{}' created successfully!", name);
    Ok(())
}

/// Handle the 'list' command to display all profiles
pub fn handle_list() -> Result<()> {
    let manager = ProfileManager::new()?;
    let profiles = manager.get_all_profiles()?;

    if profiles.is_empty() {
        println!("No profiles found.");
        println!("\nCreate a profile with: gex add <name> --username <user> --email <email> --ssh-key <key>");
        return Ok(());
    }

    println!("Available profiles:\n");
    for profile in profiles {
        println!("  {} {}", "●".to_string(), profile.name);
        println!("    Username: {}", profile.username);
        println!("    Email: {}", profile.email);
        println!("    SSH Key: {}", profile.ssh_key_name);
        println!();
    }

    Ok(())
}

/// Handle the 'switch' command to switch to a profile
pub fn handle_switch(name: String, global: bool) -> Result<()> {
    let scope = if global {
        ConfigScope::Global
    } else {
        ConfigScope::Local
    };

    let mut switcher = ProfileSwitcher::new()?;
    switcher.switch_profile(&name, scope)?;

    Ok(())
}

/// Handle the 'delete' command to remove a profile
pub fn handle_delete(name: String) -> Result<()> {
    let mut manager = ProfileManager::new()?;

    // Check if profile exists
    if !manager.profile_exists(&name)? {
        return Err(crate::error::ProfileError::ProfileNotFound(name));
    }

    // Confirm deletion
    let confirm = Confirm::new()
        .with_prompt(format!("Are you sure you want to delete profile '{}'?", name))
        .default(false)
        .interact()
        .unwrap_or(false);

    if !confirm {
        println!("Deletion cancelled.");
        return Ok(());
    }

    manager.delete_profile(&name)?;
    println!("✓ Profile '{}' deleted successfully!", name);

    Ok(())
}

/// Handle the 'edit' command to update a profile
pub fn handle_edit(name: String) -> Result<()> {
    let mut manager = ProfileManager::new()?;

    // Get existing profile
    let existing = manager
        .get_profile(&name)?
        .ok_or_else(|| crate::error::ProfileError::ProfileNotFound(name.clone()))?;

    println!("Editing profile '{}'", name);
    println!("Press Enter to keep current value\n");

    // Get new values with defaults
    let username: String = Input::new()
        .with_prompt("Username")
        .default(existing.username.clone())
        .interact_text()
        .unwrap();

    let email: String = Input::new()
        .with_prompt("Email")
        .default(existing.email.clone())
        .interact_text()
        .unwrap();

    let ssh_key: String = Input::new()
        .with_prompt("SSH Key")
        .default(existing.ssh_key_name.clone())
        .interact_text()
        .unwrap();

    // Validate inputs
    if !Validator::validate_username(&username) {
        return Err(crate::error::ProfileError::InvalidInput(
            "Invalid GitHub username format".to_string(),
        ));
    }

    if !Validator::validate_email(&email) {
        return Err(crate::error::ProfileError::InvalidInput(
            "Invalid email format".to_string(),
        ));
    }

    if !Validator::validate_ssh_key_name(&ssh_key) {
        return Err(crate::error::ProfileError::InvalidInput(
            "Invalid SSH key name".to_string(),
        ));
    }

    // Update the profile
    let updated_profile = Profile {
        name: name.clone(),
        username,
        email,
        ssh_key_name: ssh_key,
    };

    manager.update_profile(&name, updated_profile)?;
    println!("\n✓ Profile '{}' updated successfully!", name);

    Ok(())
}

/// Handle the 'status' command to show current profile information
pub fn handle_status() -> Result<()> {
    let switcher = ProfileSwitcher::new()?;
    let status = switcher.get_current_status()?;

    println!("Current Profile Status:\n");

    // Global profile
    println!("Global:");
    if let Some(profile) = status.global {
        println!("  Profile: {}", profile.name);
        println!("  Username: {}", profile.username);
        println!("  Email: {}", profile.email);
        println!("  SSH Key: {}", profile.ssh_key_name);
    } else {
        println!("  No profile set");
    }

    println!();

    // Local profile
    println!("Local (current repository):");
    if let Some(profile) = status.local {
        println!("  Profile: {}", profile.name);
        println!("  Username: {}", profile.username);
        println!("  Email: {}", profile.email);
        println!("  SSH Key: {}", profile.ssh_key_name);
    } else {
        println!("  No profile set or not in a git repository");
    }

    Ok(())
}

/// Handle the 'help' command to display usage information
pub fn handle_help() {
    println!("gex - Git profile switcher for managing multiple GitHub accounts\n");
    println!("USAGE:");
    println!("    gex <COMMAND> [OPTIONS]\n");
    println!("COMMANDS:");
    println!("    add       Add a new profile");
    println!("    list      List all profiles");
    println!("    switch    Switch to a profile");
    println!("    delete    Delete a profile");
    println!("    edit      Edit a profile");
    println!("    status    Show current profile status");
    println!("    tui       Launch interactive TUI");
    println!("    help      Print this message\n");
    println!("EXAMPLES:");
    println!("    gex add personal --username john-doe --email john@personal.com --ssh-key id_rsa_personal");
    println!("    gex list");
    println!("    gex switch personal --global");
    println!("    gex switch work --local");
    println!("    gex status");
    println!("    gex delete old-profile");
    println!("    gex edit personal");
    println!("    gex tui\n");
    println!("For more information, visit: https://github.com/FriezaForce/gex");
}
