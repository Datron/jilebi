# Context7 Jilebi Plugin

This is a Jilebi plugin that provides access to Context7 documentation services. It converts the Context7 MCP server functionality into the Jilebi plugin format.

## Features

### Resources
- **library-search**: Search for libraries and get Context7-compatible library IDs
- **library-docs**: Fetch up-to-date documentation for a specific library

### Tools
- **resolve-library-id**: Resolves a package/product name to a Context7-compatible library ID and returns a list of matching libraries
- **get-library-docs**: Fetches up-to-date documentation for a library using exact Context7-compatible library ID

### Prompts
- **library-summary**: Generate a summary of a library's capabilities and usage

## Functions

### Resource Functions
Each resource function takes `request` and `_env` parameters and returns content with MIME type.

- `searchLibraries(request: { query: string }, _env: unknown)` - Search for libraries
- `getLibraryDocs(request: { libraryId: string; tokens?: number; topic?: string }, _env: unknown)` - Get library documentation

### Tool Functions
Each tool function takes `request` and `_env` parameters and returns a ToolResponse.

- `resolveLibraryId(request: { libraryName: string }, _env: unknown)` - Resolve library ID
- `getLibraryDocumentation(request: { context7CompatibleLibraryID: string; topic?: string; tokens?: number }, _env: unknown)` - Get library documentation

## Usage

This plugin provides access to Context7's comprehensive library documentation database. Use the `resolve-library-id` tool first to find the correct library ID, then use `get-library-docs` to fetch the documentation.

## Configuration

The plugin requires access to `https://context7.com/api/*` endpoints as specified in the manifest.

## Installation

Place the plugin files in your jilebi plugins directory and ensure the manifest.toml is properly configured.
