<#
.SYNOPSIS
    Install Zippy — the Zeta toolchain installer — on Windows.
.DESCRIPTION
    Downloads the latest zippy-windows-x64.exe from GitHub releases,
    places it in ~\.zippy\bin\, and adds it to the user PATH.
.LINK
    https://z-lang.org
    https://github.com/murphsicles/zippy
.EXAMPLE
    iex (iwr -useb https://raw.githubusercontent.com/murphsicles/zippy/main/install.ps1)
#>

$Version = "v0.1.1"
$Repo = "murphsicles/zippy"
$InstallDir = "$env:USERPROFILE\.zippy\bin"
$Binary = "zippy.exe"

function Write-Logo {
    Write-Host ""
    Write-Host "╔════════════════════════════════════╗" -ForegroundColor Yellow
    Write-Host "║                                    ║" -ForegroundColor Yellow
    Write-Host "║  ███████╗███████╗████████╗         ║" -ForegroundColor Yellow
    Write-Host "║  ╚══███╔╝╚══███╔╝╚══███╔╝         ║" -ForegroundColor Yellow
    Write-Host "║    ███╔╝   ███╔╝   ███╔╝          ║" -ForegroundColor Yellow
    Write-Host "║   ███╔╝   ███╔╝   ███╔╝           ║" -ForegroundColor Yellow
    Write-Host "║  ███████╗███████╗███████╗          ║" -ForegroundColor Yellow
    Write-Host "║  ╚══════╝╚══════╝╚══════╝          ║" -ForegroundColor Yellow
    Write-Host "║                                    ║" -ForegroundColor Yellow
    Write-Host "║  Zeta Installer $Version             ║" -ForegroundColor DarkGray
    Write-Host "╚════════════════════════════════════╝" -ForegroundColor Yellow
    Write-Host ""
}

function Write-Step($Message) {
    Write-Host "  ▶ $Message" -ForegroundColor Yellow -NoNewline
}

function Write-StepOk($Message) {
    Write-Host " ✓ $Message" -ForegroundColor Green
}

function Write-Error($Message) {
    Write-Host "  ✗ $Message" -ForegroundColor Red
    exit 1
}

function Write-Info($Message) {
    Write-Host "  $Message" -ForegroundColor DarkGray
}

# ── Main ──────────────────────────────────────────────────────────────────────

Write-Logo

# Check architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
if ($arch -ne "x86_64") {
    Write-Host "  Unsupported architecture: $arch" -ForegroundColor Red
    Write-Host "  Zippy currently supports 64-bit Windows only." -ForegroundColor DarkGray
    exit 1
}

$Asset = "zippy-windows-x64.exe"
$DownloadUrl = "https://github.com/$Repo/releases/download/$Version/$Asset"

# Step 1: Create install directory
Write-Step "Creating directories..."
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Write-StepOk "~\.zippy\bin"

# Step 2: Download zippy
Write-Step "Downloading zippy..."
try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    $tmpFile = [System.IO.Path]::GetTempFileName()
    $webClient = New-Object System.Net.WebClient
    $webClient.DownloadFile($DownloadUrl, $tmpFile)
    Write-StepOk "zippy ($Version)"
}
catch {
    Write-Error "Download failed: $_"
}

# Step 3: Move to install directory
$destPath = Join-Path $InstallDir $Binary
Move-Item -Force $tmpFile $destPath

# Step 4: Add to PATH
Write-Step "Configuring PATH..."
$currentPath = [Environment]::GetEnvironmentVariable("Path", [EnvironmentVariableTarget]::User)
if ($currentPath -notlike "*$InstallDir*") {
    $newPath = "$InstallDir;$currentPath"
    [Environment]::SetEnvironmentVariable("Path", $newPath, [EnvironmentVariableTarget]::User)
    # Also update current session
    $env:Path = "$InstallDir;$env:Path"
    Write-StepOk "Added to user PATH"
}
else {
    Write-Info "Already in PATH"
}

# Step 5: Verify
Write-Step "Verifying..."
if (Test-Path $destPath) {
    Write-StepOk "zippy is ready"
}
else {
    Write-Error "Installation failed: zippy not found at $destPath"
}

Write-Host ""
Write-Host " ✓ Zeta Installer $Version installed!" -ForegroundColor Green
Write-Host ""
Write-Host "  Next steps:" -ForegroundColor DarkGray
Write-Host "  ▶ zippy install" -ForegroundColor Yellow
Write-Host "  ▶ zetac --version" -ForegroundColor Yellow
Write-Host "  ▶ zetac --zorb search nour" -ForegroundColor Yellow
Write-Host ""
Write-Host "  z-lang.org  ·  zorbs.io/docs" -ForegroundColor DarkGray
Write-Host ""

# Refresh PATH for this session
Write-Host "  NOTE: Open a new terminal for PATH changes to take effect." -ForegroundColor DarkGray
Write-Host "  Or run: `$env:Path = \"$InstallDir;`$env:Path\"" -ForegroundColor DarkGray
Write-Host ""
