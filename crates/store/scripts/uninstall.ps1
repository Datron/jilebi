#Requires -Version 5.1
<#
.SYNOPSIS
    Jilebi Uninstallation Script for Windows
.DESCRIPTION
    Removes Jilebi MCP server from Windows systems.
.PARAMETER InstallDir
    The directory where Jilebi is installed. Default: $env:LOCALAPPDATA\jilebi
.PARAMETER KeepData
    Keep plugins and user data.
.PARAMETER Force
    Skip confirmation prompts.
.EXAMPLE
    .\uninstall.ps1
    Uninstalls Jilebi with confirmation prompts.
.EXAMPLE
    .\uninstall.ps1 -Force
    Uninstalls Jilebi without confirmation prompts.
.EXAMPLE
    .\uninstall.ps1 -KeepData
    Uninstalls Jilebi but keeps plugins and data.
#>

param(
    [string]$InstallDir = "$env:LOCALAPPDATA\jilebi",
    [switch]$KeepData,
    [switch]$Force
)

$ErrorActionPreference = "Stop"

# Data and config directories on Windows
$DataDir = "$env:APPDATA\jilebi\jilebi-server\data"
$ConfigDir = "$env:APPDATA\jilebi\jilebi-server"
$BinDir = Join-Path $InstallDir "bin"

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
}

function Remove-FromPath {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")

    if ($currentPath -like "*$BinDir*") {
        Write-Info "Removing $BinDir from user PATH..."

        # Split, filter, and rejoin
        $pathParts = $currentPath -split ";" | Where-Object { $_ -ne $BinDir -and $_ -ne "" }
        $newPath = $pathParts -join ";"

        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")

        # Update current session
        $env:Path = ($env:Path -split ";" | Where-Object { $_ -ne $BinDir -and $_ -ne "" }) -join ";"

        Write-Success "Removed from PATH"
    }
    else {
        Write-Info "Jilebi not found in user PATH"
    }
}

function Remove-InstallDir {
    if (Test-Path $InstallDir) {
        Write-Info "Removing installation directory: $InstallDir"
        Remove-Item -Path $InstallDir -Recurse -Force
        Write-Success "Removed $InstallDir"
    }
    else {
        Write-Info "Installation directory not found: $InstallDir"
    }
}

function Remove-DataDir {
    if (Test-Path $DataDir) {
        Write-Info "Removing data directory: $DataDir"
        Remove-Item -Path $DataDir -Recurse -Force
        Write-Success "Removed $DataDir"
    }
    else {
        Write-Info "Data directory not found: $DataDir"
    }
}

function Remove-ConfigDir {
    if (Test-Path $ConfigDir) {
        Write-Info "Removing config directory: $ConfigDir"
        Remove-Item -Path $ConfigDir -Recurse -Force
        Write-Success "Removed $ConfigDir"
    }
    else {
        Write-Info "Config directory not found: $ConfigDir"
    }

    # Also try to remove the parent jilebi folder if empty
    $parentDir = "$env:APPDATA\jilebi"
    if ((Test-Path $parentDir) -and ((Get-ChildItem $parentDir -Force | Measure-Object).Count -eq 0)) {
        Remove-Item -Path $parentDir -Force
        Write-Info "Removed empty parent directory: $parentDir"
    }
}

function Main {
    Write-Host ""
    Write-Host "       ██╗██╗██╗     ███████╗██████╗ ██╗" -ForegroundColor Cyan
    Write-Host "       ██║██║██║     ██╔════╝██╔══██╗██║" -ForegroundColor Cyan
    Write-Host "       ██║██║██║     █████╗  ██████╔╝██║" -ForegroundColor Cyan
    Write-Host "  ██   ██║██║██║     ██╔══╝  ██╔══██╗██║" -ForegroundColor Cyan
    Write-Host "  ╚█████╔╝██║███████╗███████╗██████╔╝██║" -ForegroundColor Cyan
    Write-Host "   ╚════╝ ╚═╝╚══════╝╚══════╝╚═════╝ ╚═╝" -ForegroundColor Cyan
    Write-Host "        Uninstallation Script" -ForegroundColor DarkGray
    Write-Host ""

    # Check if jilebi is installed
    $jilebiExe = Join-Path $BinDir "jilebi.exe"
    if (-not (Test-Path $InstallDir) -and -not (Test-Path $jilebiExe)) {
        Write-Warn "Jilebi does not appear to be installed at $InstallDir"
        Write-Host ""

        if (-not $Force) {
            $response = Read-Host "Continue anyway? [y/N]"
            if ($response -notmatch "^[Yy]$") {
                Write-Info "Uninstallation cancelled."
                exit 0
            }
        }
    }

    Write-Host ""
    Write-Warn "This will remove Jilebi and all its data from your system."
    Write-Host ""
    Write-Host "The following will be removed:"
    Write-Host "  - Installation directory: $InstallDir"
    Write-Host "  - Data directory: $DataDir"
    Write-Host "  - Config directory: $ConfigDir"
    Write-Host "  - PATH entry"
    Write-Host ""

    if (-not $Force) {
        $response = Read-Host "Are you sure you want to uninstall Jilebi? [y/N]"
        if ($response -notmatch "^[Yy]$") {
            Write-Info "Uninstallation cancelled."
            exit 0
        }

        Write-Host ""

        if (-not $KeepData) {
            $keepResponse = Read-Host "Keep plugins and data? [y/N]"
            if ($keepResponse -match "^[Yy]$") {
                $KeepData = $true
                Write-Info "Plugins and data will be preserved."
            }
        }
    }

    Write-Host ""
    Write-Info "Uninstalling Jilebi..."
    Write-Host ""

    # Remove from PATH
    Remove-FromPath

    # Remove installation directory
    Remove-InstallDir

    # Remove data and config if not keeping
    if (-not $KeepData) {
        Remove-DataDir
        Remove-ConfigDir
    }
    else {
        Write-Info "Skipping data and config removal (KeepData specified)"
    }

    Write-Host ""
    Write-Success "Jilebi has been uninstalled!"
    Write-Host ""
    Write-Info "You may need to restart your terminal for PATH changes to take effect."
    Write-Host ""
}

Main
