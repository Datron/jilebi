---
sidebar_position: 5
title: Uninstall Jilebi
description: Uninstall Jilebi from your system
---

# Uninstall Jilebi

To completely remove Jilebi from your system:

### Linux & macOS

Using `curl`:
```bash
curl -fsSL https://mcp.jilebi.ai/uninstall.sh | bash
```

Using `wget`:
```bash
wget -qO- https://mcp.jilebi.ai/uninstall.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://mcp.jilebi.ai/uninstall.ps1 | iex
```

Or with options:
```powershell
# Skip confirmation prompts
irm https://mcp.jilebi.ai/uninstall.ps1 -OutFile uninstall.ps1; .\uninstall.ps1 -Force

# Keep plugins and data
irm https://mcp.jilebi.ai/uninstall.ps1 -OutFile uninstall.ps1; .\uninstall.ps1 -KeepData
```

### What the uninstaller removes

- Jilebi binary and installation directory
- PATH entry from shell profile (Unix) or user environment (Windows)
- Optionally: plugins, data, and configuration files