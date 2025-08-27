
export function get_manifest_format(request, env) {
	return {
		contents: [
			{
				uri: "file://plugin_manifest_schema.json",
				text: `
				{
					"$schema": "http://json-schema.org/draft-07/schema#",
					"title": "Jilebi Plugin Manifest",
					"type": "object",
					"required": ["name", "resources", "tools", "prompts"],
					"properties": {
						"name": {
						"type": "string",
						"pattern": "^[a-z0-9\\-].+$",
						"description": "The name of the plugin"
						},
						"version": {
						"type": "string",
						"description": "Plugin version"
						},
						"homepage": {
						"type": "string",
						"description": "Plugin homepage URL"
						},
						"creator": {
						"type": "string",
						"description": "Plugin creator"
						},
						"contact": {
						"type": "string",
						"description": "Contact information"
						},
						"resources": {
						"type": "object",
						"patternProperties": {
							"^[a-z0-9\\-].+$": {
							"$ref": "#/definitions/Resource"
							}
						},
						"additionalProperties": false
						},
						"tools": {
						"type": "object",
						"patternProperties": {
							"^[a-z0-9\\-].+$": {
							"$ref": "#/definitions/Tool"
							}
						},
						"additionalProperties": false
						},
						"prompts": {
						"type": "object",
						"patternProperties": {
							"^[a-z0-9\\-].+$": {
							"$ref": "#/definitions/Prompt"
							}
						},
						"additionalProperties": false
						}
					},
					"additionalProperties": true,
					"definitions": {
						"Resource": {
						"type": "object",
						"required": ["name", "function"],
						"properties": {
							"name": {
							"type": "string",
							"description": "Resource name"
							},
							"description": {
							"type": "string",
							"description": "Resource description"
							},
							"mime_type": {
							"type": "string",
							"description": "MIME type of the resource"
							},
							"size": {
							"type": "integer",
							"minimum": 0,
							"description": "Size of the resource"
							},
							"function": {
							"type": "string",
							"description": "Function name to call for this resource"
							},
							"permissions": {
							"$ref": "#/definitions/Permissions"
							}
						},
						"additionalProperties": false
						},
						"Tool": {
						"type": "object",
						"required": ["name", "input_schema", "function"],
						"properties": {
							"name": {
							"type": "string",
							"description": "Tool name"
							},
							"description": {
							"type": "string",
							"description": "Tool description"
							},
							"input_schema": {
							"type": "object",
							"description": "JSON schema for tool input parameters"
							},
							"function": {
							"type": "string",
							"description": "Function name to call for this tool"
							},
							"annotations": {
							"$ref": "#/definitions/ToolAnnotations"
							},
							"permissions": {
							"$ref": "#/definitions/Permissions"
							}
						},
						"additionalProperties": false
						},
						"Prompt": {
						"type": "object",
						"required": ["name", "messages"],
						"properties": {
							"name": {
							"type": "string",
							"description": "Prompt name"
							},
							"description": {
							"type": "string",
							"description": "Prompt description"
							},
							"arguments": {
							"type": "array",
							"items": {
								"$ref": "#/definitions/PromptArgument"
							},
							"description": "Prompt arguments"
							},
							"messages": {
							"type": "array",
							"items": {
								"$ref": "#/definitions/PromptMessage"
							},
							"description": "Prompt messages"
							}
						},
						"additionalProperties": false
						},
						"PromptArgument": {
						"type": "object",
						"required": ["name"],
						"properties": {
							"name": {
							"type": "string",
							"description": "Argument name"
							},
							"description": {
							"type": "string",
							"description": "Argument description"
							},
							"required": {
							"type": "boolean",
							"description": "Whether the argument is required"
							}
						},
						"additionalProperties": false
						},
						"PromptMessage": {
						"type": "object",
						"required": ["role", "content"],
						"properties": {
							"role": {
							"type": "string",
							"enum": ["user", "assistant"],
							"description": "Message role"
							},
							"type": {
							"type": "string",
							"enum": ["text"],
							"description": "Message content type"
							},
							"content": {
							"type": "string",
							"description": "Message content"
							}
						},
						"additionalProperties": false
						},
						"ToolAnnotations": {
						"type": "object",
						"properties": {
							"title": {
							"type": "string",
							"description": "Tool title"
							},
							"read_only_hint": {
							"type": "boolean",
							"description": "Whether the tool is read-only"
							},
							"destructive_hint": {
							"type": "boolean",
							"description": "Whether the tool is destructive"
							},
							"idempotent_hint": {
							"type": "boolean",
							"description": "Whether the tool is idempotent"
							},
							"open_world_hint": {
							"type": "boolean",
							"description": "Whether the tool has open world behavior"
							}
						},
						"additionalProperties": false
						},
						"Permissions": {
						"type": "object",
						"properties": {
							"hosts": {
							"type": "array",
							"items": {
								"type": "string"
							},
							"description": "Allowed hosts"
							},
							"urls": {
							"type": "array",
							"items": {
								"type": "string"
							},
							"description": "Allowed URLs"
							},
							"config_keys": {
							"type": "array",
							"items": {
								"type": "string"
							},
							"description": "Allowed configuration keys"
							},
							"read_files": {
							"type": "array",
							"items": {
								"type": "string"
							},
							"description": "Files allowed to read"
							},
							"write_files": {
							"type": "array",
							"items": {
								"type": "string"
							},
							"description": "Files allowed to write"
							},
							"read_dirs": {
							"type": "array",
							"items": {
								"type": "string"
							},
							"description": "Directories allowed to read"
							},
							"write_dirs": {
							"type": "array",
							"items": {
								"type": "string"
							},
							"description": "Directories allowed to write"
							}
						},
						"additionalProperties": false
						}
					}
				}
				`
			}
		]
	}
}


export function get_manifest_example(request, env) {
	return {
		contents: [
			{
				uri: "file://plugin_manifest_example.toml",
				text: `
				name = "ts-simple-computer-use"
					version = "1.0"
					homepage = "github.com"
					creator = "jilebi"
					contact = "support@jilebi.ai"

					[resources.list-files]
					name = "list files"
					description = "list some files from a directory"
					mime_type = "text/plain"
					function = "get_files"

					[resources.list-processes]
					name = "list processes"
					description = "list all processes running"
					mime_type = "text/plain"
					function = "get_processes"


					[tools.create-directory]
					name = "create-directory"
					description = "create a new directory using the mkdir command on linux"
					input_schema = { type = "object", properties = { name = { type = "string" } }, required = [
						"name",
					] }
					function = "create_new_directory"
					[tools.create-directory.annotations]
					title = "new folder"
					read_only_hint = false
					destructive_hint = false
					idempotent_hint = true
					open_world_hint = false

					[prompts.git-commit]
					name = "git-commit"
					description = "Generate a Git commit message"
					arguments = [
						{ name = "changes", description = "Git diff or description of changes", required = true },
					]
					messages = [
						{ role = "user", type = "text", content = "Generate a concise but descriptive commit message for these changes:\n\n${request.params.arguments?.changes}" },
					]
				`
			},
			{
				uri: "file://context7_plugin_manifest_example.toml",
				text: `
				name = "context7"
				version = "1.0.13"
				homepage = "https://github.com/upstash/context7"
				creator = "upstash"
				contact = "support@upstash.com"

				[resources]


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

				[tools.get-library-docs]
				name = "get-library-docs"
				description = "Fetches up-to-date documentation for a library using exact Context7-compatible library ID"
				input_schema = { type = "object", properties = { context7CompatibleLibraryID = { type = "string", description = "Exact Context7-compatible library ID (e.g., '/mongodb/docs', '/vercel/next.js')" }, topic = { type = "string", description = "Topic to focus documentation on (e.g., 'hooks', 'routing')" }, tokens = { type = "number", description = "Maximum number of tokens of documentation to retrieve (default: 10000)" } }, required = [
					"context7CompatibleLibraryID",
				] }
				function = "get_library_documentation"
				[tools.get-library-docs.annotations]
				title = "get documentation"
				read_only_hint = true
				destructive_hint = false
				idempotent_hint = true
				open_world_hint = false
				[tools.get-library-docs.permissions]
				hosts = ["https://context7.com"]


				[prompts.library-summary]
				name = "library-summary"
				description = "Generate a summary of a library's capabilities and usage"
				arguments = [
					{ name = "libraryId", description = "The Context7-compatible library ID", required = true },
					{ name = "focus", description = "Specific aspect to focus on (optional)", required = false },
				]
				messages = [
					{ role = "user", type = "text", content = "Provide a comprehensive summary of the library ${request.params.arguments?.libraryId}${request.params.arguments?.focus ? ' with focus on ' + request.params.arguments.focus : ''}. Include key features, installation instructions, and common use cases." },
				]
				`
			},
			{
				uri: "file://memory_plugin_manifest_example.toml",
				text: `
				name = "memory"
				version = "0.6.3"
				homepage = "github.com"
				creator = "anthropic"
				contact = "support@anthropic.com"

				[resources]

				[prompts]

				[tools.create-entities]
				input_schema = { "type" = "object", "properties" = { "entities" = { "type" = "array", "items" = { "type" = "object", "properties" = { "name" = { "type" = "string", "description" = "The name of the entity" }, "entityType" = { "type" = "string", "description" = "The type of the entity" }, "observations" = { "type" = "array", "items" = { "type" = "string" }, "description" = "An array of observation contents associated with the entity" } }, "required" = [
					"name",
					"entityType",
					"observations",
				] } } }, "required" = [
					"entities",
				] }
				name = "create-entities"
				description = "Create multiple new entities in the knowledge graph"
				function = "create_entities"
				[tools.create-entities.annotations]
				title = "Create Entities"
				read_only_hint = false
				destructive_hint = false
				idempotent_hint = true
				open_world_hint = false

				[tools.create-relations]
				input_schema = { "type" = "object", "properties" = { "relations" = { "type" = "array", "items" = { "type" = "object", "properties" = { "from" = { "type" = "string", "description" = "The name of the entity where the relation starts" }, "to" = { "type" = "string", "description" = "The name of the entity where the relation ends" }, "relationType" = { "type" = "string", "description" = "The type of the relation" } }, "required" = [
					"from",
					"to",
					"relationType",
				] } } }, "required" = [
					"relations",
				] }
				name = "create-relations"
				description = "Create multiple new relations between entities in the knowledge graph. Relations should be in active voice"
				function = "create_relations"
				[tools.create-relations.annotations]
				title = "Create Relations"
				read_only_hint = false
				destructive_hint = false
				idempotent_hint = true
				open_world_hint = false

				[tools.add-observations]
				input_schema = { "type" = "object", "properties" = { "observations" = { "type" = "array", "items" = { "type" = "object", "properties" = { "entityName" = { "type" = "string", "description" = "The name of the entity to add the observations to" }, "contents" = { "type" = "array", "items" = { "type" = "string" }, "description" = "An array of observation contents to add" } }, "required" = [
					"entityName",
					"contents",
				] } } }, "required" = [
					"observations",
				] }
				name = "add-observations"
				description = "Add new observations to existing entities in the knowledge graph"
				function = "add_observations"
				[tools.add-observations.annotations]
				title = "Add Observations"
				read_only_hint = false
				destructive_hint = false
				idempotent_hint = false
				open_world_hint = false

				[tools.delete-entities]
				input_schema = { "type" = "object", "properties" = { "entityNames" = { "type" = "array", "items" = { "type" = "string" }, "description" = "An array of entity names to delete" } }, "required" = [
					"entityNames",
				] }
				name = "delete-entities"
				description = "Delete multiple entities and their associated relations from the knowledge graph"
				function = "delete_entities"
				[tools.delete-entities.annotations]
				title = "Delete Entities"
				read_only_hint = false
				destructive_hint = true
				idempotent_hint = true
				open_world_hint = false

				[tools.delete-observations]
				input_schema = { "type" = "object", "properties" = { "deletions" = { "type" = "array", "items" = { "type" = "object", "properties" = { "entityName" = { "type" = "string", "description" = "The name of the entity containing the observations" }, "observations" = { "type" = "array", "items" = { "type" = "string" }, "description" = "An array of observations to delete" } }, "required" = [
					"entityName",
					"observations",
				] } } }, "required" = [
					"deletions",
				] }
				name = "delete-observations"
				description = "Delete specific observations from entities in the knowledge graph"
				function = "delete_observations"
				[tools.delete-observations.annotations]
				title = "Delete Observations"
				read_only_hint = false
				destructive_hint = true
				idempotent_hint = false
				open_world_hint = false

				[tools.delete-relations]
				input_schema = { "type" = "object", "properties" = { "relations" = { "type" = "array", "items" = { "type" = "object", "properties" = { "from" = { "type" = "string", "description" = "The name of the entity where the relation starts" }, "to" = { "type" = "string", "description" = "The name of the entity where the relation ends" }, "relationType" = { "type" = "string", "description" = "The type of the relation" } }, "required" = [
					"from",
					"to",
					"relationType",
				] } } }, "required" = [
					"relations",
				] }
				name = "delete-relations"
				description = "Delete multiple relations from the knowledge graph"
				function = "delete_relations"
				[tools.delete-relations.annotations]
				title = "Delete Relations"
				read_only_hint = false
				destructive_hint = true
				idempotent_hint = true
				open_world_hint = false

				[tools.read-graph]
				input_schema = { "type" = "object", "properties" = {} }
				name = "read-graph"
				description = "Read the entire knowledge graph"
				function = "read_graph"
				[tools.read-graph.annotations]
				title = "Read Graph"
				read_only_hint = true
				destructive_hint = false
				idempotent_hint = true
				open_world_hint = false

				[tools.search-nodes]
				input_schema = { "type" = "object", "properties" = { "query" = { "type" = "string", "description" = "The search query to match against entity names, types, and observation content" } }, "required" = [
					"query",
				] }
				name = "search-nodes"
				description = "Search for nodes in the knowledge graph based on a query"
				function = "search_nodes"
				[tools.search-nodes.annotations]
				title = "Search Nodes"
				read_only_hint = true
				destructive_hint = false
				idempotent_hint = true
				open_world_hint = true

				[tools.open-nodes]
				input_schema = { "type" = "object", "properties" = { "names" = { "type" = "array", "items" = { "type" = "string" }, "description" = "An array of entity names to retrieve" } }, "required" = [
					"names",
				] }
				name = "open-nodes"
				description = "Open specific nodes in the knowledge graph by their names"
				function = "open_nodes"
				[tools.open-nodes.annotations]
				title = "Open Nodes"
				read_only_hint = true
				destructive_hint = false
				idempotent_hint = true
				open_world_hint = false
				`
			},
			{
				uri: "file://sequential_plugin_manifest_example.toml",
				text: `
				name = "sequential-thinking"
				version = "0.2.0"
				homepage = "github.com"
				creator = "jilebi"
				contact = "support@jilebi.ai"

				[resources]

				[tools.sequential-thinking]
				input_schema = { "type" = "object", "properties" = { "thought" = { "type" = "string", "description" = "Your current thinking step" }, "nextThoughtNeeded" = { "type" = "boolean", "description" = "Whether another thought step is needed" }, "thoughtNumber" = { "type" = "integer", "description" = "Current thought number", "minimum" = 1 }, "totalThoughts" = { "type" = "integer", "description" = "Estimated total thoughts needed", "minimum" = 1 }, "isRevision" = { "type" = "boolean", "description" = "Whether this revises previous thinking" }, "revisesThought" = { "type" = "integer", "description" = "Which thought is being reconsidered", "minimum" = 1 }, "branchFromThought" = { "type" = "integer", "description" = "Branching point thought number", "minimum" = 1 }, "branchId" = { "type" = "string", "description" = "Branch identifier" }, "needsMoreThoughts" = { "type" = "boolean", "description" = "If more thoughts are needed" } }, "required" = [
					"thought",
					"nextThoughtNeeded",
					"thoughtNumber",
					"totalThoughts",
				] }
				name = "sequential-thinking"
				description = "A detailed tool for dynamic and reflective problem-solving through thoughts. This tool helps analyze problems through a flexible thinking process that can adapt and evolve. Each thought can build on, question, or revise previous insights as understanding deepens."
				function = "sequential_thinking"
				[tools.sequential-thinking.annotations]
				title = "Sequential Thinking"
				read_only_hint = false
				destructive_hint = false
				idempotent_hint = false
				open_world_hint = true

				[prompts]
				`
			}
		]
	}
}

export function get_index_example(request, env) {
	return {
		contents: [
			{
				uri: "file://plugin_manifest_example.toml",
				text: `
				export async function get_files(request, env) {
					console.info("get_files called with args ", request, env);
					let response = await fetch('https://jsonplaceholder.typicode.com/todos/1');
					let data = await response.json();
					console.log(data);
					console.info(env);
					// let file = Deno.readTextFileSync("/home/kartik/jilebi/plugins/ts-simple-computer-use/manifest.toml");
					// console.log("allowed file read", file);
					return {
						contents: [
							{
								uri: "file://man.js",
								text: "main.js"
							}
						]
					}
				}

				export async function get_processes(request, env) {
					console.log("get_processes called with args ", request, env);
					let file = Deno.readTextFileSync("/home/kartik/jilebi/plugins/ts-simple-computer-use/test.txt");
					console.log("allowed file read", file);
					return {
						contents: [
							{
								uri: "ps://jilebi",
								text: "jilebi"
							},
							{
								uri: "ps://bash",
								text: "bash"
							}
						]
					}
				}

				export function create_new_directory({ name }, env) {
					let response = \`create_new_directory called with args \${name}, env: \${env}\`;
					return {
						content: [
							{ type: "text", text: response }
						]
					};
				}
				`
			},
			{
				uri: "file://memory_plugin_manifest_example.toml",
				text: `
				// MCP Protocol interfaces
				interface MCPTextContent {
					type: "text";
					text: string;
				}

				interface MCPResult {
					content: MCPTextContent[];
					isError?: boolean;
				}

				// Knowledge Graph interfaces
				interface Entity {
					name: string;
					entityType: string;
					observations: string[];
				}

				interface Relation {
					from: string;
					to: string;
					relationType: string;
				}

				interface KnowledgeGraph {
					entities: Entity[];
					relations: Relation[];
				}

				// Input interfaces based on the manifest input_schema
				interface CreateEntitiesRequest {
					entities: Entity[];
				}

				interface CreateRelationsRequest {
					relations: Relation[];
				}

				interface AddObservationsRequest {
					observations: {
						entityName: string;
						contents: string[];
					}[];
				}

				interface DeleteEntitiesRequest {
					entityNames: string[];
				}

				interface DeleteObservationsRequest {
					deletions: {
						entityName: string;
						observations: string[];
					}[];
				}

				interface DeleteRelationsRequest {
					relations: Relation[];
				}

				interface ReadGraphRequest {
					// No properties needed
				}

				interface SearchNodesRequest {
					query: string;
				}

				interface OpenNodesRequest {
					names: string[];
				}

				// In-memory storage for the knowledge graph
				let templateKnowledgeGraph: KnowledgeGraph = {
					entities: [],
					relations: []
				};

				/**
				 * Create multiple new entities in the knowledge graph
				 */
				function create_entities(request: CreateEntitiesRequest, env: Environment): MCPResult {
					try {
						console.log("Knowledge graph loading -> ");
						let knowledgeGraph = getState(env, "knowledgeGraph") as KnowledgeGraph ?? templateKnowledgeGraph;
						console.log("Knowledge graph loaded -> ", knowledgeGraph);
						const { entities } = request;

						if (!entities || !Array.isArray(entities)) {
							throw new Error('entities parameter is required and must be an array');
						}

						// Filter out entities that already exist
						const newEntities = entities.filter(entity => {
							if (!entity.name || !entity.entityType || !Array.isArray(entity.observations)) {
								throw new Error('Each entity must have name, entityType, and observations array');
							}
							return !knowledgeGraph.entities.some(existing => existing.name === entity.name);
						});

						// Add new entities to the graph
						knowledgeGraph.entities.push(...newEntities);

						if (!setState(env, "knowledgeGraph", knowledgeGraph)) {
							console.log("State was not set, check logs for the error");
						}

						const summary = newEntities.length > 0
							? \`Created \${newEntities.length} new entities: \${newEntities.map(e => e.name).join(', ')}\`
							: 'No new entities created (all entities already exist)';

						return {
							content: [
								{
									type: "text",
									text: \`\${summary}\n\nCreated entities:\n\${JSON.stringify(newEntities, null, 2)}\`
								}
							]
						};

					} catch (error) {
						console.error("Error occurred while creating entities:", error instanceof Error);
						return {
							content: [
								{
									type: "text",
									text: \`Error creating entities: \${error instanceof Error ? error.message : String(error)}\`
								}
							],
							isError: true
						};
					}
				}

				/**
				 * Create multiple new relations between entities in the knowledge graph
				 */
				function create_relations(request: CreateRelationsRequest, env: Environment): MCPResult {
					try {
						let knowledgeGraph = getState(env, "knowledgeGraph") as KnowledgeGraph ?? templateKnowledgeGraph;
						const { relations } = request;

						if (!relations || !Array.isArray(relations)) {
							throw new Error('relations parameter is required and must be an array');
						}

						// Validate and filter new relations
						const newRelations = relations.filter(relation => {
							if (!relation.from || !relation.to || !relation.relationType) {
								throw new Error('Each relation must have from, to, and relationType');
							}

							// Check if entities exist
							const fromExists = knowledgeGraph.entities.some(e => e.name === relation.from);
							const toExists = knowledgeGraph.entities.some(e => e.name === relation.to);

							if (!fromExists) {
								throw new Error(\`Entity '\${relation.from}' does not exist\`);
							}
							if (!toExists) {
								throw new Error(\`Entity '\${relation.to}' does not exist\`);
							}

							// Check if relation already exists
							return !knowledgeGraph.relations.some(existing =>
								existing.from === relation.from &&
								existing.to === relation.to &&
								existing.relationType === relation.relationType
							);
						});

						// Add new relations to the graph
						knowledgeGraph.relations.push(...newRelations);

						if (!setState(env, "knowledgeGraph", knowledgeGraph)) {
							console.log("State was not set, check logs for the error");
						}

						const summary = newRelations.length > 0
							? \`Created \${newRelations.length} new relations\`
							: 'No new relations created (all relations already exist)';

						return {
							content: [
								{
									type: "text",
									text: \`\${summary}\n\nCreated relations:\n\${JSON.stringify(newRelations, null, 2)}\`
								}
							]
						};

					} catch (error) {
						return {
							content: [
								{
									type: "text",
									text: \`Error creating relations: \${error instanceof Error ? error.message : String(error)}\`
								}
							],
							isError: true
						};
					}
				}

				/**
				 * Add new observations to existing entities in the knowledge graph
				 */
				function add_observations(request: AddObservationsRequest, env: Environment): MCPResult {
					try {
						let knowledgeGraph = getState(env, "knowledgeGraph") as KnowledgeGraph ?? templateKnowledgeGraph;

						const { observations } = request;

						if (!observations || !Array.isArray(observations)) {
							throw new Error('observations parameter is required and must be an array');
						}

						const results: { entityName: string; addedObservations: string[] }[] = [];

						observations.forEach(obs => {
							if (!obs.entityName || !Array.isArray(obs.contents)) {
								throw new Error('Each observation must have entityName and contents array');
							}

							const entity = knowledgeGraph.entities.find(e => e.name === obs.entityName);
							if (!entity) {
								throw new Error(\`Entity '${obs.entityName}' not found\`);
							}

							// Filter out observations that already exist
							const newObservations = obs.contents.filter(content =>
								!entity.observations.includes(content)
							);

							// Add new observations
							entity.observations.push(...newObservations);

							results.push({
								entityName: obs.entityName,
								addedObservations: newObservations
							});
						});

						const totalAdded = results.reduce((sum, r) => sum + r.addedObservations.length, 0);
						const summary = \`Added \${totalAdded} new observations across \${results.length} entities\`;
						if (!setState(env, "knowledgeGraph", knowledgeGraph)) {
							console.log("State was not set, check logs for the error");
						}
						return {
							content: [
								{
									type: "text",
									text: \`${summary}\n\nResults:\n\${JSON.stringify(results, null, 2)}\`
								}
							]
						};

					} catch (error) {
						return {
							content: [
								{
									type: "text",
									text: \`Error adding observations: \${error instanceof Error ? error.message : String(error)}\`
								}
							],
							isError: true
						};
					}
				}

				/**
				 * Delete multiple entities and their associated relations from the knowledge graph
				 */
				function delete_entities(request: DeleteEntitiesRequest, env: Environment): MCPResult {
					try {
						let knowledgeGraph = getState(env, "knowledgeGraph") as KnowledgeGraph ?? templateKnowledgeGraph;

						const { entityNames } = request;

						if (!entityNames || !Array.isArray(entityNames)) {
							throw new Error('entityNames parameter is required and must be an array');
						}

						const initialEntityCount = knowledgeGraph.entities.length;
						const initialRelationCount = knowledgeGraph.relations.length;

						// Remove entities
						knowledgeGraph.entities = knowledgeGraph.entities.filter(
							entity => !entityNames.includes(entity.name)
						);

						// Remove relations involving deleted entities
						knowledgeGraph.relations = knowledgeGraph.relations.filter(
							relation => !entityNames.includes(relation.from) && !entityNames.includes(relation.to)
						);

						const deletedEntities = initialEntityCount - knowledgeGraph.entities.length;
						const deletedRelations = initialRelationCount - knowledgeGraph.relations.length;
						if (!setState(env, "knowledgeGraph", knowledgeGraph)) {
							console.log("State was not set, check logs for the error");
						}
						return {
							content: [
								{
									type: "text",
									text: \`Entities deleted successfully. Removed \${deletedEntities} entities and \${deletedRelations} associated relations.\`
								}
							]
						};

					} catch (error) {
						return {
							content: [
								{
									type: "text",
									text: \`Error deleting entities: \${error instanceof Error ? error.message : String(error)}\`
								}
							],
							isError: true
						};
					}
				}

				/**
				 * Delete specific observations from entities in the knowledge graph
				 */
				function delete_observations(request: DeleteObservationsRequest, env: Environment): MCPResult {
					try {
						let knowledgeGraph = getState(env, "knowledgeGraph") as KnowledgeGraph ?? templateKnowledgeGraph;

						const { deletions } = request;

						if (!deletions || !Array.isArray(deletions)) {
							throw new Error('deletions parameter is required and must be an array');
						}

						let totalDeleted = 0;

						deletions.forEach(deletion => {
							if (!deletion.entityName || !Array.isArray(deletion.observations)) {
								throw new Error('Each deletion must have entityName and observations array');
							}

							const entity = knowledgeGraph.entities.find(e => e.name === deletion.entityName);
							if (entity) {
								const initialCount = entity.observations.length;
								entity.observations = entity.observations.filter(
									obs => !deletion.observations.includes(obs)
								);
								totalDeleted += initialCount - entity.observations.length;
							}
						});

						if (!setState(env, "knowledgeGraph", knowledgeGraph)) {
							console.log("State was not set, check logs for the error");
						}

						return {
							content: [
								{
									type: "text",
									text: \`Observations deleted successfully. Removed \${totalDeleted} observations.\`
								}
							]
						};

					} catch (error) {
						return {
							content: [
								{
									type: "text",
									text: \`Error deleting observations: \${error instanceof Error ? error.message : String(error)}\`
								}
							],
							isError: true
						};
					}
				}

				/**
				 * Delete multiple relations from the knowledge graph
				 */
				function delete_relations(request: DeleteRelationsRequest, env: Environment): MCPResult {
					try {
						let knowledgeGraph = getState(env, "knowledgeGraph") as KnowledgeGraph ?? templateKnowledgeGraph;

						const { relations } = request;

						if (!relations || !Array.isArray(relations)) {
							throw new Error('relations parameter is required and must be an array');
						}

						const initialCount = knowledgeGraph.relations.length;

						// Remove specified relations
						knowledgeGraph.relations = knowledgeGraph.relations.filter(relation =>
							!relations.some(delRelation =>
								relation.from === delRelation.from &&
								relation.to === delRelation.to &&
								relation.relationType === delRelation.relationType
							)
						);

						const deletedCount = initialCount - knowledgeGraph.relations.length;
						if (!setState(env, "knowledgeGraph", knowledgeGraph)) {
							console.log("State was not set, check logs for the error");
						}
						return {
							content: [
								{
									type: "text",
									text: \`Relations deleted successfully. Removed \${deletedCount} relations.\`
								}
							]
						};

					} catch (error) {
						return {
							content: [
								{
									type: "text",
									text: \`Error deleting relations: \${error instanceof Error ? error.message : String(error)}\`
								}
							],
							isError: true
						};
					}
				}

				/**
				 * Read the entire knowledge graph
				 */
				function read_graph(request: ReadGraphRequest, env: Environment): MCPResult {
					try {
						let knowledgeGraph = getState(env, "knowledgeGraph") as KnowledgeGraph ?? templateKnowledgeGraph;
						console.log("Knowledge graph loaded in read_graph -> ", knowledgeGraph);

						const summary = \`Knowledge Graph Summary:\n• \${knowledgeGraph.entities.length} entities\n• \${knowledgeGraph.relations.length} relations\`;

						return {
							content: [
								{
									type: "text",
									text: \`${summary}\n\nComplete Knowledge Graph:\n\${JSON.stringify(knowledgeGraph, null, 2)}\`
								}
							]
						};

					} catch (error) {
						return {
							content: [
								{
									type: "text",
									text: \`Error reading graph: \${error instanceof Error ? error.message : String(error)}\`
								}
							],
							isError: true
						};
					}
				}

				/**
				 * Search for nodes in the knowledge graph based on a query
				 */
				function search_nodes(request: SearchNodesRequest, env: Environment): MCPResult {
					try {
						let knowledgeGraph = getState(env, "knowledgeGraph") as KnowledgeGraph ?? templateKnowledgeGraph;

						const { query } = request;

						if (!query || typeof query !== 'string') {
							throw new Error('query parameter is required and must be a string');
						}

						const lowerQuery = query.toLowerCase();

						// Filter entities based on query
						const filteredEntities = knowledgeGraph.entities.filter(entity =>
							entity.name.toLowerCase().includes(lowerQuery) ||
							entity.entityType.toLowerCase().includes(lowerQuery) ||
							entity.observations.some(obs => obs.toLowerCase().includes(lowerQuery))
						);

						// Get entity names for relation filtering
						const filteredEntityNames = new Set(filteredEntities.map(e => e.name));

						// Filter relations to only include those between filtered entities
						const filteredRelations = knowledgeGraph.relations.filter(relation =>
							filteredEntityNames.has(relation.from) && filteredEntityNames.has(relation.to)
						);

						const result: KnowledgeGraph = {
							entities: filteredEntities,
							relations: filteredRelations
						};

						const summary = \`Search Results for "\${query}":\n• \${result.entities.length} matching entities\n• \${result.relations.length} related connections\`;

						return {
							content: [
								{
									type: "text",
									text: \`\${summary}\n\nSearch Results:\n\${JSON.stringify(result, null, 2)}\`
								}
							]
						};

					} catch (error) {
						return {
							content: [
								{
									type: "text",
									text: \`Error searching nodes: \${error instanceof Error ? error.message : String(error)}\`
								}
							],
							isError: true
						};
					}
				}

				/**
				 * Open specific nodes in the knowledge graph by their names
				 */
				function open_nodes(request: OpenNodesRequest, env: Environment): MCPResult {
					try {
						let knowledgeGraph = getState(env, "knowledgeGraph") as KnowledgeGraph ?? templateKnowledgeGraph;

						const { names } = request;

						if (!names || !Array.isArray(names)) {
							throw new Error('names parameter is required and must be an array');
						}

						// Filter entities by requested names
						const filteredEntities = knowledgeGraph.entities.filter(entity =>
							names.includes(entity.name)
						);

						// Get entity names for relation filtering
						const filteredEntityNames = new Set(filteredEntities.map(e => e.name));

						// Filter relations to only include those between requested entities
						const filteredRelations = knowledgeGraph.relations.filter(relation =>
							filteredEntityNames.has(relation.from) && filteredEntityNames.has(relation.to)
						);

						const result: KnowledgeGraph = {
							entities: filteredEntities,
							relations: filteredRelations
						};

						const foundNames = filteredEntities.map(e => e.name);
						const notFoundNames = names.filter(name => !foundNames.includes(name));

						let summary = \`Opened \${result.entities.length} entities with \${result.relations.length} relations\`;
						if (notFoundNames.length > 0) {
							summary += \`\nEntities not found: \${notFoundNames.join(', ')}\`;
						}

						return {
							content: [
								{
									type: "text",
									text: \`\${summary}\n\nRequested Nodes:\n\${JSON.stringify(result, null, 2)}\`
								}
							]
						};

					} catch (error) {
						return {
							content: [
								{
									type: "text",
									text: \`Error opening nodes: \${error instanceof Error ? error.message : String(error)}\`
								}
							],
							isError: true
						};
					}
				}

				// Export the functions and types for use in other modules
				export {
					create_entities,
					create_relations,
					add_observations,
					delete_entities,
					delete_observations,
					delete_relations,
					read_graph,
					search_nodes,
					open_nodes,
				};
				`
			}
		]
	}
}