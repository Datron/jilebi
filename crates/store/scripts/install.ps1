#Requires -Version 5.1
<#
.SYNOPSIS
    Jilebi Installation Script for Windows
.DESCRIPTION
    Downloads and installs Jilebi MCP server on Windows systems.
.PARAMETER InstallDir
    The directory where Jilebi will be installed. Default: $env:LOCALAPPDATA\jilebi
.PARAMETER SkipPlugins
    Skip installing recommended plugins.
.PARAMETER SkipPath
    Skip adding Jilebi to the PATH.
.EXAMPLE
    .\install.ps1
    Installs Jilebi with default settings.
.EXAMPLE
    .\install.ps1 -InstallDir "C:\Tools\jilebi"
    Installs Jilebi to a custom directory.
.EXAMPLE
    .\install.ps1 -SkipPlugins
    Installs Jilebi without prompting for plugin installation.
#>

param(
    [string]$InstallDir = "$env:LOCALAPPDATA\jilebi",
    [switch]$SkipPlugins,
    [switch]$SkipPath
)

$ErrorActionPreference = "Stop"

$JILEBI_BASE_URL = "https://mcp.jilebi.ai/api/download/bin"

function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] " -ForegroundColor Blue -NoNewline
    Write-Host $Message
}

function Write-Success {
    param([string]$Message)
    Write-Host "[SUCCESS] " -ForegroundColor Green -NoNewline
    Write-Host $Message
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] " -ForegroundColor Yellow -NoNewline
    Write-Host $Message
}

function Write-Err {
    param([string]$Message)
    Write-Host "[ERROR] " -ForegroundColor Red -NoNewline
    Write-Host $Message
    exit 1
}

function Get-Platform {
    $arch = $env:PROCESSOR_ARCHITECTURE

    switch ($arch) {
        "AMD64" {
            return "jilebi-x86_64-pc-windows-msvc"
        }
        "x86" {
            Write-Err "32-bit Windows is not supported."
        }
        "ARM64" {
            Write-Err "ARM64 Windows is not yet supported. Please check for updates."
        }
        default {
            Write-Err "Unsupported architecture: $arch"
        }
    }
}

function Install-Jilebi {
    param([string]$Target)

    $downloadUrl = "$JILEBI_BASE_URL/$Target.zip"
    $binDir = Join-Path $InstallDir "bin"
    $tempDir = Join-Path $env:TEMP "jilebi-install-$(Get-Random)"
    $zipPath = Join-Path $tempDir "jilebi.zip"

    try {
        # Create temp directory
        New-Item -ItemType Directory -Path $tempDir -Force | Out-Null

        Write-Info "Downloading Jilebi from $downloadUrl..."

        # Download with progress
        $ProgressPreference = 'SilentlyContinue'  # Faster download
        Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath -UseBasicParsing
        $ProgressPreference = 'Continue'

        Write-Info "Extracting..."

        # Extract
        Expand-Archive -Path $zipPath -DestinationPath $tempDir -Force

        # Create installation directory
        New-Item -ItemType Directory -Path $binDir -Force | Out-Null

        # Find and move binary
        $binaryPath = Get-ChildItem -Path $tempDir -Filter "jilebi.exe" -Recurse | Select-Object -First 1

        if (-not $binaryPath) {
            Write-Err "Could not find jilebi.exe in the downloaded archive."
        }

        Copy-Item -Path $binaryPath.FullName -Destination (Join-Path $binDir "jilebi.exe") -Force

        Write-Success "Jilebi installed to $(Join-Path $binDir 'jilebi.exe')"
    }
    finally {
        # Cleanup
        if (Test-Path $tempDir) {
            Remove-Item -Path $tempDir -Recurse -Force -ErrorAction SilentlyContinue
        }
    }

    return $binDir
}

function Add-ToPath {
    param([string]$BinDir)

    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")

    if ($currentPath -like "*$BinDir*") {
        Write-Info "PATH already contains $BinDir"
        return
    }

    Write-Info "Adding $BinDir to user PATH..."

    $newPath = $currentPath + ";" + $BinDir
    [Environment]::SetEnvironmentVariable("Path", $newPath, "User")

    # Update current session
    $env:Path = $env:Path + ";" + $BinDir

    Write-Success "Added to PATH"
    Write-Warn "You may need to restart your terminal for PATH changes to take effect."
}

function Install-Plugins {
    param([string]$BinDir)

    Write-Info "Installing recommended plugins..."

    $jilebiPath = Join-Path $BinDir "jilebi.exe"

    try {
        & $jilebiPath plugins add memory 2>&1 | Out-Null
        Write-Success "Installed 'memory' plugin"
    }
    catch {
        Write-Warn "Failed to install 'memory' plugin: $_"
    }

    try {
        & $jilebiPath plugins add sequential-thinking 2>&1 | Out-Null
        Write-Success "Installed 'sequential-thinking' plugin"
    }
    catch {
        Write-Warn "Failed to install 'sequential-thinking' plugin: $_"
    }
}

# Main
function Main {
    Write-Host ""
    Write-Host "       ██╗██╗██╗     ███████╗██████╗ ██╗" -ForegroundColor Cyan
    Write-Host "       ██║██║██║     ██╔════╝██╔══██╗██║" -ForegroundColor Cyan
    Write-Host "       ██║██║██║     █████╗  ██████╔╝██║" -ForegroundColor Cyan
    Write-Host "  ██   ██║██║██║     ██╔══╝  ██╔══██╗██║" -ForegroundColor Cyan
    Write-Host "  ╚█████╔╝██║███████╗███████╗██████╔╝██║" -ForegroundColor Cyan
    Write-Host "   ╚════╝ ╚═╝╚══════╝╚══════╝╚═════╝ ╚═╝" -ForegroundColor Cyan
    Write-Host "        Installation Script" -ForegroundColor DarkGray
    Write-Host ""

    $target = Get-Platform
    Write-Info "Detected platform: $target"

    $binDir = Install-Jilebi -Target $target

    if (-not $SkipPath) {
        Add-ToPath -BinDir $binDir
    }

    if (-not $SkipPlugins) {
        $response = Read-Host "Install recommended plugins (memory, sequential-thinking)? [y/N]"
        if ($response -match "^[Yy]$") {
            Install-Plugins -BinDir $binDir
        }
    }

    Write-Host ""
    Write-Success "Jilebi installation complete!"
    Write-Host ""
    Write-Info "To get started, run: jilebi --help"
    Write-Info "To start the MCP server: jilebi stdio"
    Write-Host ""
}

Main
