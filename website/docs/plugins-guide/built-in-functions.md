---
sidebar_position: 3
title: Built-in Functions
description: Functions you can call in your code to build cool plugins
---

# Built-in Functions

Jilebi provides a comprehensive set of built-in functions that plugins can use to interact with the system. These functions are categorized into state management and file system operations, with appropriate permission requirements for each.

## State Management

State management functions allow plugins to persist data across sessions. All state management functions require no special permissions and store data persistently to disk.

### setState

Stores an object against a key in the plugin's persistent state.

```typescript
function setState(env: Environment, key: string, value: object): boolean
```

**Parameters:**
- `env`: The plugin environment containing the plugin identifier
- `key`: A string identifier for the stored value
- `value`: Any object to be stored (must be serializable)

**Returns:**
- `boolean`: `true` if the state was successfully stored, `false` otherwise

**Example:**
```typescript
const knowledgeGraph = {
  nodes: ["concept1", "concept2"],
  relationships: [["concept1", "relates_to", "concept2"]]
};

const success = setState(env, "knowledgeGraph", knowledgeGraph);
if (success) {
  console.log("Knowledge graph saved successfully");
}
```

**Permissions:** None required

---

### getState

Retrieves an object stored against a key in the plugin's persistent state.

```typescript
function getState(env: Environment, key: string): Value | null
```

**Parameters:**
- `env`: The plugin environment containing the plugin identifier
- `key`: The string identifier for the value to retrieve

**Returns:**
- `Value | null`: The stored object if found, `null` if the key doesn't exist or retrieval fails

**Example:**
```typescript
const knowledgeGraph = getState(env, "knowledgeGraph");
if (knowledgeGraph) {
  console.log("Retrieved knowledge graph:", knowledgeGraph);
} else {
  console.log("No knowledge graph found, creating new one");
}
```

**Permissions:** None required

---

### deleteState

Removes an object stored against a key in the plugin's persistent state.

```typescript
function deleteState(env: Environment, key: string): boolean
```

**Parameters:**
- `env`: The plugin environment containing the plugin identifier
- `key`: The string identifier for the value to delete

**Returns:**
- `boolean`: `true` if the state was successfully deleted, `false` otherwise

**Example:**
```typescript
const deleted = deleteState(env, "temporaryCache");
if (deleted) {
  console.log("Temporary cache cleared");
}
```

**Permissions:** None required

---

## File System Operations

Jilebi provides access to Deno's file system API with appropriate permission controls. All file system operations require explicit permission grants during plugin installation.

### Directory Operations

#### Deno.mkdir

Creates a new directory at the specified path.

```typescript
function mkdir(path: string | URL, options?: MkdirOptions): Promise<void>
```

**Parameters:**
- `path`: The directory path to create
- `options`: Optional configuration object
  - `recursive`: Create intermediate directories if needed (default: `false`)
  - `mode`: Unix permissions for the directory (default: `0o777`)

**Example:**
```typescript
// Create a single directory
await Deno.mkdir("./data");

// Create nested directories
await Deno.mkdir("./data/cache/temp", { recursive: true });

// Create with specific permissions (Unix only)
await Deno.mkdir("./secure", { mode: 0o700 });
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.mkdirSync

Synchronously creates a new directory at the specified path.

```typescript
function mkdirSync(path: string | URL, options?: MkdirOptions): void
```

**Parameters:** Same as `mkdir`

**Example:**
```typescript
Deno.mkdirSync("./output", { recursive: true });
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.readDir

Asynchronously reads the contents of a directory.

```typescript
function readDir(path: string | URL): AsyncIterable<DirEntry>
```

**Parameters:**
- `path`: The directory path to read

**Returns:**
- `AsyncIterable<DirEntry>`: An async iterable of directory entries

**Example:**
```typescript
for await (const entry of Deno.readDir("./data")) {
  console.log(`${entry.name} - ${entry.isFile ? 'File' : 'Directory'}`);
}
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.readDirSync

Synchronously reads the contents of a directory.

```typescript
function readDirSync(path: string | URL): IteratorObject<DirEntry>
```

**Example:**
```typescript
for (const entry of Deno.readDirSync("./data")) {
  console.log(entry.name);
}
```

**Permissions:** `read_files, read_dirs`

---

### File Operations

#### Deno.readFile

Asynchronously reads the entire contents of a file as bytes.

```typescript
function readFile(path: string | URL, options?: ReadFileOptions): Promise<Uint8Array>
```

**Parameters:**
- `path`: The file path to read
- `options`: Optional configuration
  - `signal`: AbortSignal to cancel the operation

**Example:**
```typescript
const data = await Deno.readFile("./config.json");
const text = new TextDecoder().decode(data);
console.log(text);
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.readFileSync

Synchronously reads the entire contents of a file as bytes.

```typescript
function readFileSync(path: string | URL): Uint8Array
```

**Example:**
```typescript
const data = Deno.readFileSync("./config.json");
const text = new TextDecoder().decode(data);
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.readTextFile

Asynchronously reads the entire contents of a file as UTF-8 text.

```typescript
function readTextFile(path: string | URL, options?: ReadFileOptions): Promise<string>
```

**Example:**
```typescript
const content = await Deno.readTextFile("./README.md");
console.log(content);
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.readTextFileSync

Synchronously reads the entire contents of a file as UTF-8 text.

```typescript
function readTextFileSync(path: string | URL): string
```

**Example:**
```typescript
const content = Deno.readTextFileSync("./package.json");
const config = JSON.parse(content);
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.writeFile

Asynchronously writes data to a file.

```typescript
function writeFile(path: string | URL, data: Uint8Array | ReadableStream<Uint8Array>, options?: WriteFileOptions): Promise<void>
```

**Parameters:**
- `path`: The file path to write to
- `data`: The data to write (bytes or stream)
- `options`: Write configuration
  - `create`: Create file if it doesn't exist (default: `true`)
  - `append`: Append to existing file (default: `false`)
  - `mode`: File permissions (Unix only)

**Example:**
```typescript
const encoder = new TextEncoder();
const data = encoder.encode("Hello, World!");
await Deno.writeFile("./greeting.txt", data);

// Append to file
await Deno.writeFile("./log.txt", data, { append: true });
```

**Permissions:** `write_files, write_dirs`, `read_files, read_dirs` (if `create: false`)

---

#### Deno.writeFileSync

Synchronously writes data to a file.

```typescript
function writeFileSync(path: string | URL, data: Uint8Array, options?: WriteFileOptions): void
```

**Example:**
```typescript
const data = new TextEncoder().encode("Configuration data");
Deno.writeFileSync("./config.dat", data);
```

**Permissions:** `write_files, write_dirs`, `read_files, read_dirs` (if `create: false`)

---

#### Deno.writeTextFile

Asynchronously writes text data to a file.

```typescript
function writeTextFile(path: string | URL, data: string | ReadableStream<string>, options?: WriteFileOptions): Promise<void>
```

**Example:**
```typescript
await Deno.writeTextFile("./output.txt", "Hello, World!");

// Write JSON data
const config = { version: "1.0", debug: true };
await Deno.writeTextFile("./config.json", JSON.stringify(config, null, 2));
```

**Permissions:** `write_files, write_dirs`, `read_files, read_dirs` (if `create: false`)

---

#### Deno.writeTextFileSync

Synchronously writes text data to a file.

```typescript
function writeTextFileSync(path: string | URL, data: string, options?: WriteFileOptions): void
```

**Example:**
```typescript
Deno.writeTextFileSync("./output.txt", "Synchronous write operation");
```

**Permissions:** `write_files, write_dirs`, `read_files, read_dirs` (if `create: false`)

---

### File Information and Manipulation

#### Deno.stat

Asynchronously gets information about a file or directory, following symbolic links.

```typescript
function stat(path: string | URL): Promise<FileInfo>
```

**Example:**
```typescript
const fileInfo = await Deno.stat("./data.txt");
console.log(`File size: ${fileInfo.size} bytes`);
console.log(`Modified: ${fileInfo.mtime}`);
console.log(`Is file: ${fileInfo.isFile}`);
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.statSync

Synchronously gets information about a file or directory, following symbolic links.

```typescript
function statSync(path: string | URL): FileInfo
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.lstat

Asynchronously gets information about a file or directory, not following symbolic links.

```typescript
function lstat(path: string | URL): Promise<FileInfo>
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.lstatSync

Synchronously gets information about a file or directory, not following symbolic links.

```typescript
function lstatSync(path: string | URL): FileInfo
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.copyFile

Asynchronously copies a file from one location to another.

```typescript
function copyFile(fromPath: string | URL, toPath: string | URL): Promise<void>
```

**Example:**
```typescript
await Deno.copyFile("./source.txt", "./backup/source_backup.txt");
```

**Permissions:** `read_files, read_dirs` (source), `write_files, write_dirs` (destination)

---

#### Deno.copyFileSync

Synchronously copies a file from one location to another.

```typescript
function copyFileSync(fromPath: string | URL, toPath: string | URL): void
```

**Permissions:** `read_files, read_dirs` (source), `write_files, write_dirs` (destination)

---

#### Deno.rename

Asynchronously renames or moves a file or directory.

```typescript
function rename(oldpath: string | URL, newpath: string | URL): Promise<void>
```

**Example:**
```typescript
await Deno.rename("./old_name.txt", "./new_name.txt");
await Deno.rename("./temp_dir", "./archive/old_temp");
```

**Permissions:** `read_files, read_dirs`, `write_files, write_dirs`

---

#### Deno.renameSync

Synchronously renames or moves a file or directory.

```typescript
function renameSync(oldpath: string | URL, newpath: string | URL): void
```

**Permissions:** `read_files, read_dirs`, `write_files, write_dirs`

---

#### Deno.remove

Asynchronously removes a file or directory.

```typescript
function remove(path: string | URL, options?: RemoveOptions): Promise<void>
```

**Parameters:**
- `path`: The file or directory to remove
- `options`: Removal options
  - `recursive`: Remove directories and their contents recursively

**Example:**
```typescript
// Remove a file
await Deno.remove("./temp.txt");

// Remove a directory and all its contents
await Deno.remove("./temp_dir", { recursive: true });
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.removeSync

Synchronously removes a file or directory.

```typescript
function removeSync(path: string | URL, options?: RemoveOptions): void
```

**Permissions:** `write_files, write_dirs`

---

### Temporary Files and Directories

#### Deno.makeTempDir

Creates a temporary directory and returns its path.

```typescript
function makeTempDir(options?: MakeTempOptions): Promise<string>
```

**Parameters:**
- `options`: Configuration options
  - `dir`: Base directory for temporary directory
  - `prefix`: Prefix for the directory name
  - `suffix`: Suffix for the directory name

**Example:**
```typescript
const tempDir = await Deno.makeTempDir();
console.log(`Temporary directory: ${tempDir}`);

const namedTempDir = await Deno.makeTempDir({ 
  prefix: "my_plugin_", 
  suffix: "_cache" 
});
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.makeTempDirSync

Synchronously creates a temporary directory and returns its path.

```typescript
function makeTempDirSync(options?: MakeTempOptions): string
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.makeTempFile

Creates a temporary file and returns its path.

```typescript
function makeTempFile(options?: MakeTempOptions): Promise<string>
```

**Example:**
```typescript
const tempFile = await Deno.makeTempFile({ prefix: "data_", suffix: ".json" });
await Deno.writeTextFile(tempFile, JSON.stringify(data));
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.makeTempFileSync

Synchronously creates a temporary file and returns its path.

```typescript
function makeTempFileSync(options?: MakeTempOptions): string
```

**Permissions:** `write_files, write_dirs`

---

### File Permissions and Metadata

#### Deno.chmod

Asynchronously changes file or directory permissions.

```typescript
function chmod(path: string | URL, mode: number): Promise<void>
```

**Parameters:**
- `path`: The file or directory path
- `mode`: Octal permission mode (e.g., `0o755`)

**Example:**
```typescript
// Make file readable and writable by owner, readable by others
await Deno.chmod("./script.sh", 0o644);

// Make file executable by owner
await Deno.chmod("./script.sh", 0o755);
```

**Note:** Not available on Windows

**Permissions:** `write_files, write_dirs`

---

#### Deno.chmodSync

Synchronously changes file or directory permissions.

```typescript
function chmodSync(path: string | URL, mode: number): void
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.chown

Asynchronously changes file or directory ownership.

```typescript
function chown(path: string | URL, uid: number | null, gid: number | null): Promise<void>
```

**Parameters:**
- `path`: The file or directory path
- `uid`: User ID of new owner (or `null` for no change)
- `gid`: Group ID of new owner (or `null` for no change)

**Note:** Not available on Windows

**Permissions:** `write_files, write_dirs`

---

#### Deno.chownSync

Synchronously changes file or directory ownership.

```typescript
function chownSync(path: string | URL, uid: number | null, gid: number | null): void
```

**Permissions:** `write_files, write_dirs`

---

### Symbolic Links

#### Deno.symlink

Creates a symbolic link pointing from `newpath` to `oldpath`.

```typescript
function symlink(oldpath: string | URL, newpath: string | URL, options?: SymlinkOptions): Promise<void>
```

**Example:**
```typescript
await Deno.symlink("./target_file.txt", "./link_to_target.txt");
```

**Permissions:** `read_files, read_dirs`, `write_files, write_dirs`

---

#### Deno.symlinkSync

Synchronously creates a symbolic link.

```typescript
function symlinkSync(oldpath: string | URL, newpath: string | URL, options?: SymlinkOptions): void
```

**Permissions:** `read_files, read_dirs`, `write_files, write_dirs`

---

#### Deno.readLink

Reads the target of a symbolic link.

```typescript
function readLink(path: string | URL): Promise<string>
```

**Example:**
```typescript
const target = await Deno.readLink("./my_link.txt");
console.log(`Link points to: ${target}`);
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.readLinkSync

Synchronously reads the target of a symbolic link.

```typescript
function readLinkSync(path: string | URL): string
```

**Permissions:** `read_files, read_dirs`

---

### Path Resolution

#### Deno.realPath

Resolves a path to its absolute, canonical form, following symbolic links.

```typescript
function realPath(path: string | URL): Promise<string>
```

**Example:**
```typescript
const realPath = await Deno.realPath("./some/relative/path");
console.log(`Absolute path: ${realPath}`);
```

**Permissions:** `read_files, read_dirs`

---

#### Deno.realPathSync

Synchronously resolves a path to its absolute, canonical form.

```typescript
function realPathSync(path: string | URL): string
```

**Permissions:** `read_files, read_dirs`

---

### File Truncation and Timestamps

#### Deno.truncate

Truncates a file to a specified length.

```typescript
function truncate(name: string, len?: number): Promise<void>
```

**Parameters:**
- `name`: File path
- `len`: Target length in bytes (if omitted, truncates to 0)

**Example:**
```typescript
// Truncate file to 100 bytes
await Deno.truncate("./large_file.txt", 100);

// Truncate file completely
await Deno.truncate("./temp.log");
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.truncateSync

Synchronously truncates a file to a specified length.

```typescript
function truncateSync(name: string, len?: number): void
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.utime

Updates file access and modification times.

```typescript
function utime(path: string | URL, atime: number | Date, mtime: number | Date): Promise<void>
```

**Parameters:**
- `path`: File path
- `atime`: Access time (Unix timestamp or Date object)
- `mtime`: Modification time (Unix timestamp or Date object)

**Example:**
```typescript
const now = new Date();
await Deno.utime("./file.txt", now, now);
```

**Permissions:** `write_files, write_dirs`

---

#### Deno.utimeSync

Synchronously updates file access and modification times.

```typescript
function utimeSync(path: string | URL, atime: number | Date, mtime: number | Date): void
```

**Permissions:** `write_files, write_dirs`

---

## Permission Model

Jilebi follows a strict permission model where plugins must explicitly request and receive permission for various operations:

### Permission Types

- **No Permission Required**: State management functions
- **`read_files, read_dirs`**: File and directory reading operations
- **`write_files, write_dirs`**: File and directory writing operations
- **`read_files, read_dirs` + `write_files, write_dirs`**: Operations that both read and write

### Permission Requests

When installing a plugin, users will be prompted to grant the necessary permissions. Plugins should clearly document what permissions they require and why.

### Best Practices

1. **Minimal Permissions**: Request only the permissions your plugin actually needs
2. **Clear Documentation**: Explain why each permission is necessary
3. **Error Handling**: Gracefully handle permission denial scenarios
4. **State Over Files**: Use state management functions when possible to avoid file system permissions

## Environment Object

All functions that take an `Environment` parameter receive an object containing:

```typescript
type Environment = {
  id: string;  // Unique plugin identifier
} & Record<string, any>;  // Additional environment variables
```

The environment object is automatically provided by the Jilebi runtime and contains the plugin's unique identifier and any configured environment variables.