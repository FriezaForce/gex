# Quick Start Guide for Deploying gex

## 🚀 Ready to Deploy!

All files have been configured for your GitHub account: **FriezaForce/gex**

## Step 1: Push to GitHub

### Option A: Using the Setup Script (Recommended)
```powershell
.\setup-github.ps1
```

### Option B: Manual Setup
```bash
# Initialize git
git init

# Add all files
git add .

# Commit
git commit -m "Initial commit: gex - Git Profile Switcher"

# Add remote
git remote add origin https://github.com/FriezaForce/gex.git

# Push
git branch -M main
git push -u origin main
```

## Step 2: Create Your First Release

```bash
# Create and push a tag
git tag v0.1.0
git push origin v0.1.0
```

This will automatically:
- ✅ Build binaries for Windows, macOS, and Linux
- ✅ Create a GitHub release
- ✅ Upload all binaries

## Step 3: Test Installation

After the release is created, users can install with:

### Windows
```powershell
irm https://raw.githubusercontent.com/FriezaForce/gex/main/install.ps1 | iex
```

### Linux/macOS
```bash
curl -fsSL https://raw.githubusercontent.com/FriezaForce/gex/main/install.sh | bash
```

## What's Included

### ✅ Installation Scripts
- `install.ps1` - Windows PowerShell installer
- `install.sh` - Linux/macOS bash installer

### ✅ CI/CD Workflows
- `.github/workflows/ci.yml` - Runs tests on every push
- `.github/workflows/release.yml` - Builds and releases binaries

### ✅ Documentation
- `README.md` - Complete user documentation
- `CONTRIBUTING.md` - Contribution guidelines
- `DEPLOYMENT.md` - Detailed deployment guide
- `LICENSE` - MIT License

### ✅ Project Files
- `Cargo.toml` - Configured with your repository
- `.gitignore` - Ignores build artifacts
- All source code with 54 passing tests

## Supported Platforms

Your releases will include binaries for:
- Windows (x86_64)
- macOS (x86_64 and Apple Silicon)
- Linux (x86_64 and ARM64)

## Repository Structure

```
gex/
├── .github/
│   └── workflows/
│       ├── ci.yml           # Continuous Integration
│       └── release.yml      # Release automation
├── src/                     # Source code
├── tests/                   # Integration tests
├── install.ps1              # Windows installer
├── install.sh               # Unix installer
├── README.md                # User documentation
├── CONTRIBUTING.md          # Contribution guide
├── DEPLOYMENT.md            # Deployment guide
├── LICENSE                  # MIT License
└── Cargo.toml               # Project configuration
```

## Next Steps

1. **Create the repository on GitHub** (if not already done)
   - Go to https://github.com/new
   - Name: `gex`
   - Description: "Git profile switcher for managing multiple GitHub accounts"
   - Public or Private (your choice)
   - Don't initialize with README (we already have one)

2. **Run the setup script**
   ```powershell
   .\setup-github.ps1
   ```

3. **Create your first release**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

4. **Wait for GitHub Actions** (takes ~5-10 minutes)
   - Go to Actions tab on GitHub
   - Watch the build progress
   - Binaries will be uploaded to the release

5. **Test the installation**
   - Try the PowerShell installer on Windows
   - Share with others!

## Troubleshooting

### "Repository not found" error
- Make sure you've created the repository on GitHub first
- Verify the repository name is exactly `gex`

### Authentication failed
- Set up GitHub authentication:
  ```bash
  gh auth login
  ```
  Or use SSH keys

### Build fails on GitHub Actions
- Check the Actions tab for error logs
- All tests pass locally, so it should work!

## Support

- Repository: https://github.com/FriezaForce/gex
- Issues: https://github.com/FriezaForce/gex/issues

## 🎉 You're All Set!

Your project is ready to be deployed. Just push to GitHub and create a release tag!
