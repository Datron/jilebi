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
			? `Created ${newEntities.length} new entities: ${newEntities.map(e => e.name).join(', ')}`
			: 'No new entities created (all entities already exist)';

		return {
			content: [
				{
					type: "text",
					text: `${summary}\n\nCreated entities:\n${JSON.stringify(newEntities, null, 2)}`
				}
			]
		};

	} catch (error) {
		console.error("Error occurred while creating entities:", error instanceof Error);
		return {
			content: [
				{
					type: "text",
					text: `Error creating entities: ${error instanceof Error ? error.message : String(error)}`
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
				throw new Error(`Entity '${relation.from}' does not exist`);
			}
			if (!toExists) {
				throw new Error(`Entity '${relation.to}' does not exist`);
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
			? `Created ${newRelations.length} new relations`
			: 'No new relations created (all relations already exist)';

		return {
			content: [
				{
					type: "text",
					text: `${summary}\n\nCreated relations:\n${JSON.stringify(newRelations, null, 2)}`
				}
			]
		};

	} catch (error) {
		return {
			content: [
				{
					type: "text",
					text: `Error creating relations: ${error instanceof Error ? error.message : String(error)}`
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
				throw new Error(`Entity '${obs.entityName}' not found`);
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
		const summary = `Added ${totalAdded} new observations across ${results.length} entities`;
		if (!setState(env, "knowledgeGraph", knowledgeGraph)) {
			console.log("State was not set, check logs for the error");
		}
		return {
			content: [
				{
					type: "text",
					text: `${summary}\n\nResults:\n${JSON.stringify(results, null, 2)}`
				}
			]
		};

	} catch (error) {
		return {
			content: [
				{
					type: "text",
					text: `Error adding observations: ${error instanceof Error ? error.message : String(error)}`
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
					text: `Entities deleted successfully. Removed ${deletedEntities} entities and ${deletedRelations} associated relations.`
				}
			]
		};

	} catch (error) {
		return {
			content: [
				{
					type: "text",
					text: `Error deleting entities: ${error instanceof Error ? error.message : String(error)}`
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
					text: `Observations deleted successfully. Removed ${totalDeleted} observations.`
				}
			]
		};

	} catch (error) {
		return {
			content: [
				{
					type: "text",
					text: `Error deleting observations: ${error instanceof Error ? error.message : String(error)}`
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
					text: `Relations deleted successfully. Removed ${deletedCount} relations.`
				}
			]
		};

	} catch (error) {
		return {
			content: [
				{
					type: "text",
					text: `Error deleting relations: ${error instanceof Error ? error.message : String(error)}`
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

		const summary = `Knowledge Graph Summary:\n• ${knowledgeGraph.entities.length} entities\n• ${knowledgeGraph.relations.length} relations`;

		return {
			content: [
				{
					type: "text",
					text: `${summary}\n\nComplete Knowledge Graph:\n${JSON.stringify(knowledgeGraph, null, 2)}`
				}
			]
		};

	} catch (error) {
		return {
			content: [
				{
					type: "text",
					text: `Error reading graph: ${error instanceof Error ? error.message : String(error)}`
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

		const summary = `Search Results for "${query}":\n• ${result.entities.length} matching entities\n• ${result.relations.length} related connections`;

		return {
			content: [
				{
					type: "text",
					text: `${summary}\n\nSearch Results:\n${JSON.stringify(result, null, 2)}`
				}
			]
		};

	} catch (error) {
		return {
			content: [
				{
					type: "text",
					text: `Error searching nodes: ${error instanceof Error ? error.message : String(error)}`
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

		let summary = `Opened ${result.entities.length} entities with ${result.relations.length} relations`;
		if (notFoundNames.length > 0) {
			summary += `\nEntities not found: ${notFoundNames.join(', ')}`;
		}

		return {
			content: [
				{
					type: "text",
					text: `${summary}\n\nRequested Nodes:\n${JSON.stringify(result, null, 2)}`
				}
			]
		};

	} catch (error) {
		return {
			content: [
				{
					type: "text",
					text: `Error opening nodes: ${error instanceof Error ? error.message : String(error)}`
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