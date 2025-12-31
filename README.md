<p align="center">
  <img src="website/static/img/jilebi_logo.svg" alt="Jilebi Logo" width="120" height="120" />
</p>

<p align="center">
<pre>
                              $$$$$\ $$\ $$\           $$\       $$\ 
                              \__$$ |\__|$$ |          $$ |      \__|
                                 $$ |$$\ $$ | $$$$$$\  $$$$$$$\  $$\ 
                                 $$ |$$ |$$ |$$  __$$\ $$  __$$\ $$ |
                           $$\   $$ |$$ |$$ |$$$$$$$$ |$$ |  $$ |$$ |
                           $$ |  $$ |$$ |$$ |$$   ____|$$ |  $$ |$$ |
                           \$$$$$$  |$$ |$$ |\$$$$$$$\ $$$$$$$  |$$ |
                            \______/ \__|\__| \_______|\_______/ \__|
</pre>
</p>

<p align="center">
  <a href="https://jilebi.ai">https://jilebi.ai</a>
</p>

**A powerful, plugin-based MCP (Model Context Protocol) server that extends AI assistants with custom tools, resources, and prompts.**

---

## Table of Contents

- [Overview](#overview)
- [Download](#download)
- [Installation](#installation)
  - [Windows](#windows)
  - [Linux](#linux)
  - [macOS](#macos)
- [Host Configuration](#host-configuration)
- [Plugin System](#plugin-system)
- [Creating Plugins](#creating-plugins)
- [Logging and Debugging](#logging-and-debugging)
- [Links](#links)

---

## Overview

Jilebi is an MCP server implementation that enables AI assistants to interact with external systems through a modular plugin architecture. It provides a standardized way to expose tools, resources, and prompts to language models while maintaining security through a granular permission system.

**Key Features:**

- Cross-platform support (Windows, Linux, macOS)
- Plugin-based extensibility
- Secure permission model for file system and network access
- Persistent state management for plugins
- Compatible with major AI development environments

---

## Download

| Platform | Architecture | Download |
|----------|--------------|----------|
| Windows 10/11 | x86_64 | [Download](https://jilebi.ai/api/download/bin/jilebi-x86_64-pc-windows-msvc.zip) |
| Linux | x86_64 | [Download](https://jilebi.ai/api/download/bin/jilebi-x86_64-unknown-linux-gnu.zip) |
| macOS | Apple Silicon | [Download](https://jilebi.ai/api/download/bin/jilebi-aarch64-apple-darwin.zip) |
| macOS | Intel | [Download](https://jilebi.ai/api/download/bin/jilebi-x86_64-apple-darwin.zip) |

---

## Installation

### Windows

```powershell
# 1. Unzip the downloaded file to your desired location

# 2. Install recommended plugins
.\jilebi.exe plugins add memory
.\jilebi.exe plugins add sequential-thinking

# 3. Start the MCP server
.\jilebi.exe stdio
```

### Linux

```bash
# 1. Unzip the downloaded file to your desired location

# 2. Grant execution access
chmod +x jilebi

# 3. Install recommended plugins
./jilebi plugins add memory
./jilebi plugins add sequential-thinking

# 4. Start the MCP server
./jilebi stdio
```

### macOS

```bash
# 1. Unzip the downloaded file to your desired location

# 2. Grant execution access
chmod +x jilebi

# 3. Remove quarantine attribute (required for unverified developers)
xattr -d com.apple.quarantine /path/to/jilebi

# 4. Install xz dependency
brew install xz

# 5. Install recommended plugins
./jilebi plugins add memory
./jilebi plugins add sequential-thinking

# 6. Start the MCP server
./jilebi stdio
```

Find more plugins at [github.com/datron/jilebi-plugins](https://github.com/datron/jilebi-plugins)

---

## Host Configuration

Jilebi integrates with various AI development environments. Below are configuration examples for supported hosts.

### Claude Desktop

**Config Location:**
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Linux: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "jilebi": {
      "command": "jilebi",
      "args": ["stdio"],
      "env": {}
    }
  }
}
```

### Claude Code

Add to `.claude.json`:

```json
{
  "mcpServers": {
    "jilebi": {
      "command": "jilebi",
      "args": ["stdio"],
      "env": {}
    }
  }
}
```

### Cursor

Edit `~/.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "jilebi": {
      "command": "jilebi",
      "args": ["stdio"],
      "env": {}
    }
  }
}
```

### VS Code

Add to your `mcp.json`:

```json
{
  "servers": {
    "jilebi": {
      "type": "stdio",
      "command": "jilebi",
      "args": ["stdio"]
    }
  }
}
```

### Zed

Edit `~/.config/zed/settings.json`:

```json
{
  "context_servers": {
    "jilebi": {
      "command": "jilebi",
      "args": ["stdio"],
      "enabled": true,
      "source": "custom",
      "env": {}
    }
  }
}
```

### Codex CLI

Add to `~/.codex/config.toml`:

```toml
[mcp_servers.jilebi]
command = "jilebi"
args = ["stdio"]
```

### Opencode

Edit `~/.config/opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "mcp": {
    "jilebi": {
      "type": "local",
      "command": ["jilebi", "stdio"],
      "enabled": true,
      "environment": {}
    }
  }
}
```

---

## Plugin System

Plugins extend Jilebi with custom functionality through three main components:

| Component | Description |
|-----------|-------------|
| **Tools** | Functions that models can invoke to interact with external systems |
| **Resources** | Read-only data sources for context (files, schemas, documentation) |
| **Prompts** | Pre-defined conversation templates with argument interpolation |

### Managing Plugins

```bash
# Install a plugin
jilebi plugins add <plugin-name>

# Create a new plugin
jilebi plugins create

# Setup/reconfigure a plugin
jilebi plugins setup <plugin-name>

# View plugin logs
jilebi plugins log <plugin-id>
```

---

## Creating Plugins

### Quick Start

```bash
jilebi plugins create
```

Follow the prompts to bootstrap a new plugin project.

### Plugin Manifest

Plugins are defined using a TOML manifest file. Here is an example structure:

```toml
# Metadata
name = "my-plugin"
version = "1.0.0"
homepage = "github.com/user/my-plugin"
creator = "Your Name"
contact = "you@example.com"

# Environment Variables
[env]
API_URL = { schema = { type = "string" }, default = "https://api.example.com" }

# Secrets (no defaults allowed)
[secrets]
API_KEY = { schema = { type = "string" } }

# Tools
[tools.my-tool]
name = "my-tool"
description = "Description of what this tool does"
input_schema = { type = "object", properties = { param = { type = "string" } }, required = ["param"] }
function = "my_tool_function"

[tools.my-tool.permissions]
hosts = ["https://api.example.com"]

# Resources
[resources.my-resource]
name = "my-resource"
description = "Description of this resource"
mime_type = "application/json"
function = "get_resource"

# Prompts
[prompts.my-prompt]
name = "my-prompt"
description = "A helpful prompt template"
arguments = [
  { name = "topic", description = "The topic to discuss", required = true }
]
messages = [
  { role = "user", content = { type = "text", content = "Tell me about {{topic}}" } }
]
```

### Permission Types

Plugins can request the following permissions:

| Permission | Description |
|------------|-------------|
| `hosts` | Allowed host domains for network requests |
| `urls` | Specific allowed URLs |
| `read_files` | File paths the plugin can read |
| `write_files` | File paths the plugin can write |
| `read_dirs` | Directories the plugin can read from |
| `write_dirs` | Directories the plugin can write to |

Use `"user_defined"` to prompt the user for a custom value during setup.

### Built-in Functions

Plugins have access to:

**State Management:**
- `setState(env, key, value)` - Persist data across sessions
- `getState(env, key)` - Retrieve persisted data
- `deleteState(env, key)` - Remove persisted data

**File System (Deno API):**
- Directory operations: `mkdir`, `readDir`
- File operations: `readFile`, `writeFile`, `readTextFile`, `writeTextFile`
- File manipulation: `stat`, `copyFile`, `rename`, `remove`
- Temporary files: `makeTempDir`, `makeTempFile`
- And more...

---

## Logging and Debugging

### Log Locations

| Platform | Path |
|----------|------|
| Windows | `C:\Users\<user>\AppData\Roaming\jilebi\jilebi-server\data\logs` |
| Linux | `~/.local/share/jilebi-server/logs/` |
| macOS | `~/Library/Application Support/ai.jilebi.jilebi-server/logs` |

### Viewing Logs

```bash
# View Jilebi server logs
jilebi log

# View plugin-specific logs
jilebi plugins log <plugin-id>
```

### Writing Logs in Plugins

```javascript
console.log("Info message");
console.warn("Warning message");
console.error("Error message");
console.debug("Debug message");
```

---

## Links

- [Plugin Repository](https://github.com/datron/jilebi-plugins)
- [MCP Specification](https://modelcontextprotocol.io/specification/2025-03-26)

---

<p align="center">
<sub>Built with the Model Context Protocol (MCP) specification version 2025-03-26</sub>
</p>