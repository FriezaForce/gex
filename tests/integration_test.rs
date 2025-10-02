use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

/// Helper to create a temporary test environment
fn create_test_env() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!("gex_e2e_test_{}", timestamp));
    fs::create_dir_all(&temp_dir).unwrap();
    temp_dir
}

fn cleanup_test_env(temp_dir: &PathBuf) {
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(temp_dir);
    }
}

fn get_binary_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // Remove test binary name
    path.pop(); // Remove 'deps' directory
    path.push("gex");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path
}

#[test]
fn test_version_command() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("--version")
        .output()
        .expect("Failed to execute gex");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("gex"));
}

#[test]
fn test_help_command() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("--help")
        .output()
        .expect("Failed to execute gex");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Git profile switcher"));
    assert!(stdout.contains("add"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("switch"));
}

#[test]
fn test_list_empty_profiles() {
    let binary = get_binary_path();
    let temp_dir = create_test_env();
    
    // Set HOME to temp directory to use isolated config
    let output = Command::new(&binary)
        .arg("list")
        .env("HOME", &temp_dir)
        .env("USERPROFILE", &temp_dir)
        .output()
        .expect("Failed to execute gex");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No profiles found") || stdout.contains("Available profiles"));

    cleanup_test_env(&temp_dir);
}

#[test]
fn test_add_profile_missing_args() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("add")
        .arg("testprofile")
        .output()
        .expect("Failed to execute gex");

    // Should fail because required arguments are missing
    assert!(!output.status.success());
}

#[test]
fn test_switch_nonexistent_profile() {
    let binary = get_binary_path();
    let temp_dir = create_test_env();
    
    let output = Command::new(&binary)
        .arg("switch")
        .arg("nonexistent")
        .arg("--global")
        .env("HOME", &temp_dir)
        .env("USERPROFILE", &temp_dir)
        .output()
        .expect("Failed to execute gex");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("Error"));

    cleanup_test_env(&temp_dir);
}

#[test]
fn test_status_command() {
    let binary = get_binary_path();
    let temp_dir = create_test_env();
    
    let output = Command::new(&binary)
        .arg("status")
        .env("HOME", &temp_dir)
        .env("USERPROFILE", &temp_dir)
        .output()
        .expect("Failed to execute gex");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Global") || stdout.contains("Status"));

    cleanup_test_env(&temp_dir);
}

#[test]
fn test_invalid_command() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("invalid-command")
        .output()
        .expect("Failed to execute gex");

    assert!(!output.status.success());
}

#[test]
fn test_add_command_help() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("add")
        .arg("--help")
        .output()
        .expect("Failed to execute gex");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("username"));
    assert!(stdout.contains("email"));
    assert!(stdout.contains("ssh-key"));
}

#[test]
fn test_switch_command_help() {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .arg("switch")
        .arg("--help")
        .output()
        .expect("Failed to execute gex");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("global"));
}

// Note: Full end-to-end tests that actually create profiles, switch them,
// and verify git/SSH config changes are not included here because they would:
// 1. Modify the user's actual git configuration
// 2. Modify the user's SSH config
// 3. Require actual SSH keys to exist
//
// These tests verify the CLI interface works correctly without side effects.
// For full integration testing, use a dedicated test environment or CI/CD pipeline.
