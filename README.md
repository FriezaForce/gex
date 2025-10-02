# gex - Git Profile Switcher

A fast and intuitive CLI tool for managing multiple GitHub accounts with different SSH keys and git configurations.

## Features

- 🚀 **Fast** - Built with Rust for maximum performance
- 🔐 **SSH Key Management** - Automatically configures SSH keys for each profile
- 🎯 **Git Config** - Manages git user.name and user.email for global and local scopes
- 💾 **Profile Storage** - Stores profiles in JSON format
- 🎨 **TUI Mode** - Interactive terminal UI for easy profile management
- ✅ **Validation** - Input validation for all profile fields
- 🛡️ **Safe** - Backs up SSH config before modifications

## Installation

### Quick Install (Recommended)

#### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/FriezaForce/gex/main/install.ps1 | iex
```

#### Linux/macOS
```bash
curl -fsSL https://raw.githubusercontent.com/FriezaForce/gex/main/install.sh | bash
```

### From GitHub Releases

Download the latest binary for your platform from the [Releases page](https://github.com/FriezaForce/gex/releases).

#### Windows
1. Download `gex-vX.X.X-x86_64-pc-windows-msvc.zip`
2. Extract `gex.exe`
3. Add to your PATH or move to a directory in your PATH

#### Linux
```bash
# Download and extract
wget https://github.com/FriezaForce/gex/releases/latest/download/gex-vX.X.X-x86_64-unknown-linux-gnu.tar.gz
tar -xzf gex-vX.X.X-x86_64-unknown-linux-gnu.tar.gz

# Move to PATH
sudo mv gex /usr/local/bin/
```

#### macOS
```bash
# Download and extract
curl -LO https://github.com/FriezaForce/gex/releases/latest/download/gex-vX.X.X-x86_64-apple-darwin.tar.gz
tar -xzf gex-vX.X.X-x86_64-apple-darwin.tar.gz

# Move to PATH
sudo mv gex /usr/local/bin/
```

### From Source

```bash
cargo install --path .
```

### From Crates.io

```bash
cargo install gex
```

## Quick Start

### 1. Add a Profile

```bash
gex add personal --username john-doe --email john@personal.com --ssh-key id_rsa_personal
```

### 2. List Profiles

```bash
gex list
```

### 3. Switch Profile

```bash
# Switch globally
gex switch personal --global

# Switch for current repository only
gex switch work --local
```

### 4. Check Status

```bash
gex status
```

## Usage

### Commands

#### Add a Profile

```bash
gex add <name> --username <username> --email <email> --ssh-key <ssh-key-name>
```

**Example:**
```bash
gex add work --username john-work --email john@company.com --ssh-key id_ed25519_work
```

#### List All Profiles

```bash
gex list
```

**Output:**
```
Available profiles:

  ● personal
    Username: john-doe
    Email: john@personal.com
    SSH Key: id_rsa_personal

  ● work
    Username: john-work
    Email: john@company.com
    SSH Key: id_ed25519_work
```

#### Switch Profile

```bash
# Switch globally (affects all repositories)
gex switch <profile-name> --global

# Switch locally (affects current repository only)
gex switch <profile-name> --local
```

**Examples:**
```bash
gex switch personal --global
gex switch work --local
```

#### Delete a Profile

```bash
gex delete <profile-name>
```

The command will ask for confirmation before deleting.

#### Edit a Profile

```bash
gex edit <profile-name>
```

Interactive prompts will guide you through updating the profile fields.

#### Show Status

```bash
gex status
```

**Output:**
```
Current Profile Status:

Global:
  Profile: personal
  Username: john-doe
  Email: john@personal.com
  SSH Key: id_rsa_personal

Local (current repository):
  No profile set or not in a git repository
```

#### Launch TUI

```bash
gex tui
```

Opens an interactive terminal UI for managing profiles.

## Configuration

### Profile Storage

Profiles are stored in:
- **Windows:** `%USERPROFILE%\.github-profile-switcher\profiles.json`
- **Linux/macOS:** `~/.github-profile-switcher/profiles.json`

### SSH Configuration

gex automatically manages your `~/.ssh/config` file by adding host entries for each profile:

```
# GitHub Profile: personal
Host github.com-personal
  HostName github.com
  User git
  IdentityFile ~/.ssh/id_rsa_personal
  IdentitiesOnly yes
```

A backup is created before any modifications (`.ssh/config.bak`).

### Git Configuration

When you switch profiles, gex updates:
- `user.name` - Your GitHub username
- `user.email` - Your email address

For global scope: Updates `~/.gitconfig`
For local scope: Updates `.git/config` in the current repository

## SSH Key Setup

Before using gex, ensure you have SSH keys set up for each GitHub account:

### Generate SSH Key

```bash
# For personal account
ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519_personal -C "your-email@personal.com"

# For work account
ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519_work -C "your-email@work.com"
```

### Add SSH Key to GitHub

1. Copy your public key:
   ```bash
   # Windows
   type ~/.ssh/id_ed25519_personal.pub | clip
   
   # Linux/macOS
   cat ~/.ssh/id_ed25519_personal.pub | pbcopy
   ```

2. Go to GitHub → Settings → SSH and GPG keys → New SSH key
3. Paste your public key and save

### Test SSH Connection

```bash
ssh -T git@github.com
```

## Troubleshooting

### SSH Key Not Found

**Error:** `SSH key not found: ~/.ssh/id_rsa_personal`

**Solution:**
- Verify the SSH key exists: `ls ~/.ssh/`
- Generate a new key if needed (see SSH Key Setup above)
- Update the profile with the correct key name: `gex edit <profile>`

### Not a Git Repository

**Error:** `Not a git repository`

**Solution:**
- Use `--global` flag to set the profile globally
- Or run the command inside a git repository for local configuration

### Git Not Installed

**Error:** `Git is not installed`

**Solution:**
- Install git from https://git-scm.com/downloads
- Restart your terminal after installation

### Permission Denied

**Error:** `Permission denied: <path>`

**Solution:**
- Check file permissions
- Ensure you have write access to the directory
- On Windows, try running as administrator

## Examples

### Scenario: Personal and Work Accounts

```bash
# Set up personal profile
gex add personal --username john-doe --email john@personal.com --ssh-key id_ed25519_personal

# Set up work profile
gex add work --username john-work --email john@company.com --ssh-key id_ed25519_work

# Set personal as global default
gex switch personal --global

# In a work repository, switch to work profile locally
cd ~/work/company-repo
gex switch work --local

# Check current status
gex status
```

### Scenario: Open Source Contributions

```bash
# Add open source profile
gex add opensource --username john-oss --email john@opensource.org --ssh-key id_ed25519_oss

# Switch to open source profile for a specific project
cd ~/projects/open-source-project
gex switch opensource --local
```

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/gex.git
cd gex

# Build
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- CLI powered by [clap](https://github.com/clap-rs/clap)
- TUI powered by [ratatui](https://github.com/ratatui-org/ratatui)

## Support

If you encounter any issues or have questions:
- Open an issue on GitHub
- Check the troubleshooting section above
- Use `gex <command> --help` for command-specific help
