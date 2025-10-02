# Setup script for pushing gex to GitHub
# Run this script to initialize git and push to your repository

Write-Host "Setting up gex for GitHub..." -ForegroundColor Cyan
Write-Host ""

# Check if git is installed
if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Host "Error: Git is not installed" -ForegroundColor Red
    Write-Host "Please install Git from https://git-scm.com/downloads" -ForegroundColor Yellow
    exit 1
}

# Initialize git if not already done
if (-not (Test-Path .git)) {
    Write-Host "Initializing git repository..." -ForegroundColor Yellow
    git init
    Write-Host "✓ Git initialized" -ForegroundColor Green
} else {
    Write-Host "✓ Git repository already initialized" -ForegroundColor Green
}

# Add all files
Write-Host "Adding files..." -ForegroundColor Yellow
git add .

# Show status
Write-Host ""
Write-Host "Files to be committed:" -ForegroundColor Cyan
git status --short

# Commit
Write-Host ""
$commitMessage = Read-Host "Enter commit message (or press Enter for default)"
if ([string]::IsNullOrWhiteSpace($commitMessage)) {
    $commitMessage = "Initial commit: gex - Git Profile Switcher"
}

git commit -m "$commitMessage"
Write-Host "✓ Changes committed" -ForegroundColor Green

# Check if remote exists
$remoteExists = git remote | Select-String "origin"

if (-not $remoteExists) {
    Write-Host ""
    Write-Host "Adding remote repository..." -ForegroundColor Yellow
    git remote add origin https://github.com/FriezaForce/gex.git
    Write-Host "✓ Remote added" -ForegroundColor Green
} else {
    Write-Host "✓ Remote already configured" -ForegroundColor Green
}

# Set main branch
git branch -M main

# Ask before pushing
Write-Host ""
$push = Read-Host "Push to GitHub? (y/n)"

if ($push -eq "y" -or $push -eq "Y") {
    Write-Host "Pushing to GitHub..." -ForegroundColor Yellow
    git push -u origin main
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✓ Successfully pushed to GitHub!" -ForegroundColor Green
        Write-Host ""
        Write-Host "Next steps:" -ForegroundColor Cyan
        Write-Host "1. Visit: https://github.com/FriezaForce/gex" -ForegroundColor White
        Write-Host "2. Create a release by pushing a tag:" -ForegroundColor White
        Write-Host "   git tag v0.1.0" -ForegroundColor Yellow
        Write-Host "   git push origin v0.1.0" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "3. GitHub Actions will automatically build binaries for all platforms" -ForegroundColor White
        Write-Host ""
    } else {
        Write-Host ""
        Write-Host "Push failed. You may need to:" -ForegroundColor Yellow
        Write-Host "1. Create the repository on GitHub first" -ForegroundColor White
        Write-Host "2. Authenticate with GitHub (gh auth login or set up SSH keys)" -ForegroundColor White
        Write-Host ""
    }
} else {
    Write-Host ""
    Write-Host "Skipped push. You can push later with:" -ForegroundColor Yellow
    Write-Host "  git push -u origin main" -ForegroundColor White
    Write-Host ""
}
