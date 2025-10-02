# Deployment Guide for gex

This guide explains how to deploy gex to GitHub and create releases.

## Initial Setup

### 1. Repository URLs

All repository URLs have been configured for: **FriezaForce/gex**

Files already updated:
- ✅ Cargo.toml
- ✅ install.ps1
- ✅ install.sh
- ✅ README.md
- ✅ CONTRIBUTING.md
- ✅ src/cli/handlers.rs

### 2. Create GitHub Repository

```bash
# Initialize git (if not already done)
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit: gex - Git Profile Switcher"

# Add remote
git remote add origin https://github.com/FriezaForce/gex.git

# Push to GitHub
git branch -M main
git push -u origin main
```

### 3. Set up GitHub Secrets (Optional)

For publishing to crates.io, add a secret:

1. Go to your repository on GitHub
2. Settings → Secrets and variables → Actions
3. Add new repository secret:
   - Name: `CARGO_TOKEN`
   - Value: Your crates.io API token (get from https://crates.io/me)

## Creating a Release

### Method 1: Using Git Tags (Recommended)

```bash
# Create and push a tag
git tag v0.1.0
git push origin v0.1.0
```

This will automatically:
- Trigger the GitHub Actions workflow
- Build binaries for Windows, macOS, and Linux
- Create a GitHub release
- Upload all binaries to the release
- Publish to crates.io (if CARGO_TOKEN is set)

### Method 2: Manual Release

1. Go to your repository on GitHub
2. Click "Releases" → "Create a new release"
3. Create a new tag (e.g., `v0.1.0`)
4. Fill in release notes
5. The workflow will automatically build and upload binaries

## Release Checklist

Before creating a release:

- [ ] Update version in `Cargo.toml`
- [ ] Update CHANGELOG.md (if you have one)
- [ ] Run all tests: `cargo test`
- [ ] Build release locally: `cargo build --release`
- [ ] Test the binary: `./target/release/gex --help`
- [ ] Commit version changes
- [ ] Create and push tag

## Installation Methods After Release

Once released, users can install gex using:

### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/FriezaForce/gex/main/install.ps1 | iex
```

### Linux/macOS
```bash
curl -fsSL https://raw.githubusercontent.com/FriezaForce/gex/main/install.sh | bash
```

### From crates.io
```bash
cargo install gex
```

## CI/CD Workflows

### CI Workflow (`.github/workflows/ci.yml`)
Runs on every push and pull request:
- Tests on Windows, macOS, and Linux
- Checks code formatting
- Runs clippy linter
- Generates code coverage

### Release Workflow (`.github/workflows/release.yml`)
Runs when a tag is pushed:
- Builds binaries for all platforms
- Creates GitHub release
- Uploads binaries
- Publishes to crates.io

## Supported Platforms

The release workflow builds for:

- **Windows:** x86_64-pc-windows-msvc
- **macOS:** x86_64-apple-darwin, aarch64-apple-darwin (Apple Silicon)
- **Linux:** x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu

## Troubleshooting

### Build Fails on GitHub Actions

- Check the Actions tab for error logs
- Ensure all tests pass locally
- Verify Cargo.toml is valid

### Release Not Created

- Ensure tag follows format `v*` (e.g., v0.1.0)
- Check GitHub Actions permissions
- Verify workflow file syntax

### Installation Script Fails

- Ensure the release has been created
- Check that binaries were uploaded
- Verify URLs in installation scripts

## Version Numbering

Follow Semantic Versioning (semver):

- **MAJOR** version: Incompatible API changes
- **MINOR** version: New functionality (backwards compatible)
- **PATCH** version: Bug fixes (backwards compatible)

Example: `v1.2.3` = Major.Minor.Patch

## Post-Release

After creating a release:

1. Test installation scripts on different platforms
2. Update documentation if needed
3. Announce the release (social media, forums, etc.)
4. Monitor issues for bug reports

## Support

For issues with deployment:
- Check GitHub Actions logs
- Review this deployment guide
- Open an issue on GitHub
