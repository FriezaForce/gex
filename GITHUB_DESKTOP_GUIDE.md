# GitHub Desktop Setup Guide

## Step-by-Step Instructions for Publishing gex

### Step 1: Create Repository on GitHub.com

1. Go to https://github.com/new
2. Fill in the details:
   - **Repository name:** `gex`
   - **Description:** `Git profile switcher for managing multiple GitHub accounts`
   - **Visibility:** Public (recommended) or Private
   - **Important:** Do NOT check any boxes (no README, no .gitignore, no license)
3. Click **"Create repository"**
4. Leave this page open (you'll need it in a moment)

### Step 2: Open GitHub Desktop

1. Open **GitHub Desktop** application
2. If you're not signed in:
   - Go to **File → Options → Accounts**
   - Click **Sign in** to GitHub.com
   - Follow the authentication steps

### Step 3: Add Your Local Repository

1. In GitHub Desktop, click **File → Add local repository**
2. Click **Choose...** button
3. Navigate to your gex project folder (where this file is located)
4. Click **Select Folder**

If you see "This directory does not appear to be a Git repository":
1. Click **Create a repository** instead
2. Or click **Cancel** and follow Step 3a below

### Step 3a: Initialize Repository (if needed)

If the folder isn't a Git repository yet:

1. In GitHub Desktop, click **File → New repository**
2. Fill in:
   - **Name:** `gex`
   - **Local path:** Choose the PARENT folder of your gex project
   - **Git ignore:** None
   - **License:** None
   - **Initialize with README:** Unchecked
3. Click **Create repository**

OR simply run this in PowerShell in your project folder:
```powershell
git init
```
Then go back to Step 3.

### Step 4: Review Changes

In GitHub Desktop, you should now see:
- Left panel: List of all your files (green + icons = new files)
- Right panel: File contents/changes
- Bottom left: Commit message box

### Step 5: Make Your First Commit

1. In the bottom left, you'll see:
   - **Summary:** Enter `Initial commit: gex - Git Profile Switcher`
   - **Description:** (optional) You can leave this empty
2. Click the blue **"Commit to main"** button

All your files are now committed locally!

### Step 6: Publish to GitHub

1. At the top, click the blue **"Publish repository"** button
2. A dialog will appear:
   - **Name:** `gex` (should be pre-filled)
   - **Description:** `Git profile switcher for managing multiple GitHub accounts`
   - **Keep this code private:** Uncheck this (unless you want it private)
   - **Organization:** Should show "FriezaForce" (your account)
3. Click **"Publish repository"**

GitHub Desktop will now upload all your files to GitHub!

### Step 7: Verify on GitHub

1. In GitHub Desktop, click **Repository → View on GitHub**
   - Or visit: https://github.com/FriezaForce/gex
2. You should see all your files!
3. The README.md should display on the main page

### Step 8: Create Your First Release

Now that your code is on GitHub, create a release to trigger the automated builds:

#### Option A: Using GitHub Desktop

1. In GitHub Desktop, click **Repository → Create tag**
2. Enter tag name: `v0.1.0`
3. Click **Create tag**
4. Click **Push origin** button at the top
5. In the push dialog, make sure **"Include tags"** is checked
6. Click **Push**

#### Option B: Using PowerShell

```powershell
git tag v0.1.0
git push origin v0.1.0
```

### Step 9: Watch the Magic Happen! ✨

1. Go to https://github.com/FriezaForce/gex/actions
2. You'll see the "Release" workflow running
3. Wait 5-10 minutes for all builds to complete
4. Go to https://github.com/FriezaForce/gex/releases
5. You'll see release v0.1.0 with binaries for:
   - Windows
   - macOS (Intel and Apple Silicon)
   - Linux (x86_64 and ARM64)

### Step 10: Test Installation

Now users (including you) can install gex with one command!

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/FriezaForce/gex/main/install.ps1 | iex
```

**Linux/macOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/FriezaForce/gex/main/install.sh | bash
```

## Making Future Changes

### When You Make Code Changes:

1. Make your changes to the code
2. Open GitHub Desktop
3. You'll see the changed files in the left panel
4. Enter a commit message (e.g., "Fix bug in profile switching")
5. Click **"Commit to main"**
6. Click **"Push origin"** at the top

### Creating New Releases:

1. Update version in `Cargo.toml`
2. Commit the change in GitHub Desktop
3. Push to GitHub
4. Create a new tag (e.g., `v0.2.0`)
5. Push the tag
6. GitHub Actions will automatically build and release!

## Troubleshooting

### "Repository not found" error
- Make sure you created the repository on GitHub.com first
- Check that you're signed in to the correct GitHub account in GitHub Desktop

### Can't see "Publish repository" button
- You might already be connected to a different repository
- Try: **Repository → Repository settings** and check the remote URL
- It should be: `https://github.com/FriezaForce/gex.git`

### Changes not showing in GitHub Desktop
- Make sure you've saved all your files
- Try clicking **Repository → Refresh** (or press F5)

### Push failed
- Check your internet connection
- Make sure you're signed in to GitHub Desktop
- Try: **Repository → Push** again

## Quick Reference

### Common GitHub Desktop Actions:

- **View changes:** Changes tab (left side)
- **Commit:** Fill in message, click "Commit to main"
- **Push to GitHub:** Click "Push origin" button at top
- **Pull from GitHub:** Click "Fetch origin" then "Pull origin"
- **View on GitHub:** Repository → View on GitHub
- **Create tag:** Repository → Create tag
- **Open in terminal:** Repository → Open in command prompt

### Useful Keyboard Shortcuts:

- `Ctrl+1` - Show changes
- `Ctrl+2` - Show history
- `Ctrl+Enter` - Commit
- `Ctrl+P` - Push
- `F5` - Refresh

## Success! 🎉

Once you've completed these steps:
- ✅ Your code is on GitHub
- ✅ CI/CD is set up and running
- ✅ Binaries are automatically built for all platforms
- ✅ Users can install with one command

**Your repository:** https://github.com/FriezaForce/gex

**Need help?** Check the other guides:
- `QUICK_START.md` - Quick overview
- `DEPLOYMENT_CHECKLIST.md` - Detailed checklist
- `DEPLOYMENT.md` - Complete deployment guide
