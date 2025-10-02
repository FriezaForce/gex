# gex Installation Script for Windows
# Usage: irm https://raw.githubusercontent.com/yourusername/gex/main/install.ps1 | iex

$ErrorActionPreference = "Stop"

Write-Host "Installing gex - Git Profile Switcher" -ForegroundColor Cyan
Write-Host ""

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "i686" }
$target = "$arch-pc-windows-msvc"

# Get latest release info
$repo = "FriezaForce/gex"
$apiUrl = "https://api.github.com/repos/$repo/releases/latest"

Write-Host "Fetching latest release..." -ForegroundColor Yellow

try {
    $release = Invoke-RestMethod -Uri $apiUrl -Headers @{ "User-Agent" = "gex-installer" }
    $version = $release.tag_name
    Write-Host "Latest version: $version" -ForegroundColor Green
} catch {
    Write-Host "Error: Failed to fetch release information" -ForegroundColor Red
    Write-Host "Please check your internet connection and repository URL" -ForegroundColor Red
    exit 1
}

# Find the Windows binary asset
$asset = $release.assets | Where-Object { $_.name -like "*windows*.zip" -or $_.name -like "*$target*.zip" } | Select-Object -First 1

if (-not $asset) {
    Write-Host "Error: No Windows binary found in the latest release" -ForegroundColor Red
    exit 1
}

$downloadUrl = $asset.browser_download_url
$fileName = $asset.name

Write-Host "Downloading $fileName..." -ForegroundColor Yellow

# Create temp directory
$tempDir = Join-Path $env:TEMP "gex-install"
if (Test-Path $tempDir) {
    Remove-Item $tempDir -Recurse -Force
}
New-Item -ItemType Directory -Path $tempDir | Out-Null

$zipPath = Join-Path $tempDir $fileName

# Download the binary
try {
    Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
    Write-Host "Downloaded successfully" -ForegroundColor Green
} catch {
    Write-Host "Error: Failed to download binary" -ForegroundColor Red
    exit 1
}

# Extract the archive
Write-Host "Extracting..." -ForegroundColor Yellow
Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

# Find the gex.exe file
$exePath = Get-ChildItem -Path $tempDir -Filter "gex.exe" -Recurse | Select-Object -First 1

if (-not $exePath) {
    Write-Host "Error: gex.exe not found in the archive" -ForegroundColor Red
    exit 1
}

# Determine installation directory
$installDir = Join-Path $env:LOCALAPPDATA "gex"
if (-not (Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir | Out-Null
}

$targetPath = Join-Path $installDir "gex.exe"

# Copy the binary
Write-Host "Installing to $installDir..." -ForegroundColor Yellow
Copy-Item $exePath.FullName -Destination $targetPath -Force

# Add to PATH if not already there
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    Write-Host "Adding to PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$userPath;$installDir",
        "User"
    )
    $env:Path = "$env:Path;$installDir"
    Write-Host "Added to PATH (restart your terminal for changes to take effect)" -ForegroundColor Green
}

# Cleanup
Remove-Item $tempDir -Recurse -Force

Write-Host ""
Write-Host "✓ gex installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Installation location: $targetPath" -ForegroundColor Cyan
Write-Host ""
Write-Host "Quick start:" -ForegroundColor Yellow
Write-Host "  gex --help                    # Show help"
Write-Host "  gex add <name> ...            # Add a profile"
Write-Host "  gex list                      # List profiles"
Write-Host "  gex switch <name> --global    # Switch profile"
Write-Host ""
Write-Host "Note: You may need to restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
