---
sidebar_position: 2
title: Understanding the Manifest file
description: Learn about the manifest file and what it does
---

# The manifest file

:::info
Jilebi used version 2025-03-26 of the MCP specification
:::

The manifest file defines resources, tools and prompts that a plugin offers. It also defines the envs and secrets that the plugin needs to function along with permissions for that resources and tools can request from the user. It also contains some metadata about the plugin. Let us discuss the metadata first.

The manifest file is written in a configuration language called [toml](https://toml.io/en/). 

## Metadata

The metadata for a plugin is simple. It consists of the following fields:

- name
- version
- homepage
- create
- contact


:::info
Make sure the `name` of the plugin matches the directory name the plugin is placed in. If you made a mistake with the directory recreate the plugin.
:::

Below is an example for `sequential-thinking`:

```toml
name = "sequential-thinking"
version = "0.2.0"
homepage = "github.com"
creator = "jilebi"
contact = "support@jilebi.ai"
```

## Environment Variables

This is a table/section in the toml file. Here is an example from the `grafana` plugin:

```toml
[env]
GRAFANA_URL = { schema = { type = "string" }, default = "http://localhost:3000" }
GRAFANA_USERNAME = { schema = { type = "string" } }
GRAFANA_PASSWORD = { schema = { type = "string" } }
```

Every env defines two things:

- schema: takes jsonschema to define the type of the env. Jilebi will validate the type against what the user enters to avoid runtime issues
- default: a default value for the env so that the user doesn't need to configure all values for envs

## Secrets


This is a table/section in the toml file. Here is an example from the `grafana` plugin:

```toml
[secrets]
GRAFANA_SERVICE_ACCOUNT_TOKEN = { schema = { type = "string" } }
GRAFANA_API_KEY = { schema = { type = "string" } }
```

It is the same format as the envs section without a default - this means that the user has to specify a value for a secret

## Permissions

Every resource and tool supports a permissions definition. The format of the permissions declaration is as follows:

```toml
[entity.name.permissions]
hosts = [string]
urls = [string]
read_files = [string]
write_files = [string]
read_dirs = [string]
write_dirs = [string]
```

If you specify the string `user_defined`, jilebi will ask the user to specify the value for the permission. If you specify the permission, the user is just asked to grant or deny access.

### Permission Types
- `hosts` - Array of allowed host domains
- `urls` - Array of specific allowed URLs
- `read_files` - Array of file paths/patterns the plugin can read
- `write_files` - Array of file paths/patterns the plugin can write
- `read_dirs` - Array of directory paths the plugin can read from
- `write_dirs` - Array of directory paths the plugin can write to


Eg from the file-system plugin:

```toml
[tools.read-text-file]
name = "read-text-file"
description = "Read the complete contents of a file as text with optional head/tail support"
input_schema = { type = "object", properties = { path = { type = "string", description = "File path to read" }, head = { type = "number", description = "Read only first N lines", minimum = 1 }, tail = { type = "number", description = "Read only last N lines", minimum = 1 } }, required = [
	"path",
] }
function = "read_text_file"
[tools.read-text-file.permissions]
read_dirs = ["user_defined"]
```

## Resources

[MCP page](https://modelcontextprotocol.io/specification/2025-03-26/server/resources)

Resources are passive data sources that provide read-only access to information for context, such as file contents, database schemas, or API documentation. Jilebi expects a function to return data for a resource. The simplified format is as follows:

```toml
[resources.name]
name = "name"
description = "description"
mime_type = "mime-type"
function = "function_name"
```

Jilebi supports the all fields defined in the [MCP specification](https://modelcontextprotocol.io/specification/2025-03-26/server/resources) (since it uses rmcp for types) so you can specify anything defined for resources

- name: the name of the resource. The name defined here and the name in `[resources.name]` should be the same. Use hypen '-' as a word separator.
- description: A description for the resource
- mime-type: the type of the resource
- function: The function in the JS/TS file to call

:::info
Make sure the `name` of the resource matches the key declared in the resources table/section. Use hypen ('-') as a word separator, as jilebi uses '_' internally for some operations
:::

Here is an example of of a resource from `jilebi-gen`, a plugin that helps with converting MCP servers to jilebi plugins

```toml
[resources.manifest-schema]
name = "manifest schema"
description = "The schema for the plugin manifest"
mime_type = "text/plain"
function = "get_manifest_format"
```
The function called should return a response that is expected by the mcp resource response type. An example is:

```js
return {
contents: [
  {
    uri: "",
    text: "",
  }
]}

```

## Tools

[MCP page](https://modelcontextprotocol.io/specification/2025-03-26/server/tools)

Expose tools that can be invoked by language models. Tools enable models to interact with external systems, such as querying databases, calling APIs, or performing computations. Each tool is uniquely identified by a name and includes metadata describing its schema. Jilebi models tools as follows:

```toml
[tools.name]
name = "name"
description = "description"
input_schema = { type = "object" } # jsonschena definition of inputs
function = "function_name"
[tools.name.annotations]
title = "title"
read_only_hint = true
destructive_hint = false
idempotent_hint = true
open_world_hint = false
```

Here is an example from `context7`:

```toml
[tools.resolve-library-id]
name = "resolve-library-id"
description = "Resolves a package/product name to a Context7-compatible library ID and returns a list of matching libraries"
input_schema = { type = "object", properties = { libraryName = { type = "string", description = "Library name to search for and retrieve a Context7-compatible library ID" } }, required = [
	"libraryName",
] }
function = "resolve_library_id"
[tools.resolve-library-id.annotations]
title = "resolve library ID"
read_only_hint = true
destructive_hint = false
idempotent_hint = true
open_world_hint = false
[tools.resolve-library-id.permissions]
hosts = ["https://context7.com"]
```

:::info
Make sure the `name` of the tool matches the key declared in the tools table/section. Use hypen ('-') as a word separator, as jilebi uses '_' internally for some operations
:::

The return type of the function for a tool should be the same as the ToolResponse type defined by the MCP specification. Here is an example:
```js
return {
	content: [{
		type: "text",
		text: `Available Libraries (top matches):

Each result includes:
- Library ID: Context7-compatible identifier (format: /org/project)
- Name: Library or package name
- Description: Short summary
- Code Snippets: Number of available code examples
- Trust Score: Authority indicator
- Versions: List of versions if available. Use one of those versions if and only if the user explicitly provides a version in their query.

For best results, select libraries based on name match, trust score, snippet coverage, and relevance to your use case.

----------

${resultsText}`
	}]
};
```

## Prompts

[MCP page](https://modelcontextprotocol.io/specification/2025-03-26/server/prompts)

the format of a prompt is as follows:

```toml
[prompts.name]
name = "name"
description = "description"
arguments = [
	{ name = "arg1", description = "desc1", required = true },
	{ name = "arg2", description = "desc2", required = false },
]
messages = [
	{ role = "user", content = { type = "text", content = "" } },
	{ role = "user", content = { type = "text", content = "" } },
	{ role = "assistant", content = { type = "text", content = "" } },
]
```

Example from the `met-museum` plugin:

```toml
[prompts.museum-educator]
name = "museum-educator"
description = "Act as a museum educator to provide educational information about artworks"
arguments = [
	{ name = "age_group", description = "Target age group (e.g., children, teenagers, adults)", required = true },
	{ name = "learning_focus", description = "Educational focus area", required = false },
]
messages = [
	{ role = "user", content = { type = "text", content = "You are an experienced museum educator at the Metropolitan Museum of Art, specializing in making art accessible and engaging for {{age_group}}{{#if learning_focus}} with a focus on {{learning_focus}}{{/if}}." } },
	{ role = "user", content = { type = "text", content = "Adapt your teaching style for {{age_group}}: use age-appropriate language, ask engaging questions, focus on relevant aspects, and make connections to their experiences. Use the Met Museum tools to select and present artworks that align with your educational goals." } },
	{ role = "assistant", content = { type = "text", content = "I'm excited to be your museum educator guide for {{age_group}}{{#if learning_focus}} focusing on {{learning_focus}}{{/if}}! Let me explore the Met's collection to find perfect artworks for our educational journey and present them in an engaging, age-appropriate way." } },
]
```

:::info
Prompts use [handlerbar syntax](https://handlebarsjs.com/guide/) to weave arguments into the prompt. You can follow the guide to embed logic in your prompt definitions
:::