/**
 * Formats a search result into a human-readable string representation.
 * Only shows code snippet count and GitHub stars when available (not equal to -1).
 *
 * @param result The SearchResult object to format
 * @returns A formatted string with library information
 */
function formatSearchResult(result) {
    // Always include these basic details
    const formattedResult = [
        `- Title: ${result.title}`,
        `- Context7-compatible library ID: ${result.id}`,
        `- Description: ${result.description}`,
    ];
    // Only add code snippets count if it's a valid value
    if (result.totalSnippets !== -1 && result.totalSnippets !== undefined) {
        formattedResult.push(`- Code Snippets: ${result.totalSnippets}`);
    }
    // Only add trust score if it's a valid value
    if (result.trustScore !== -1 && result.trustScore !== undefined) {
        formattedResult.push(`- Trust Score: ${result.trustScore}`);
    }
    // Only add versions if it's a valid value
    if (result.versions !== undefined && result.versions.length > 0) {
        formattedResult.push(`- Versions: ${result.versions.join(", ")}`);
    }
    // Join all parts with newlines
    return formattedResult.join("\n");
}
/**
 * Formats a search response into a human-readable string representation.
 * Each result is formatted using formatSearchResult.
 *
 * @param searchResponse The SearchResponse object to format
 * @returns A formatted string with search results
 */
function formatSearchResults(searchResponse) {
    if (!searchResponse.results || searchResponse.results.length === 0) {
        return "No documentation libraries found matching your query.";
    }
    const formattedResults = searchResponse.results.map(formatSearchResult);
    return formattedResults.join("\n----------\n");
}

const CONTEXT7_API_BASE_URL = "https://context7.com/api";
const DEFAULT_TYPE = "txt";
const DEFAULT_MINIMUM_TOKENS = 10000;
function generateHeaders(_clientIp, extraHeaders = {}) {
    const headers = { ...extraHeaders };
    return headers;
}
/**
 * Jilebi plugin for Context7 documentation access
 * Converts MCP server functionality to jilebi plugin format
 */
// Tool function: Resolve library ID
async function resolve_library_id(request, _env) {
    try {
        const url = new URL(`${CONTEXT7_API_BASE_URL}/v1/search`);
        url.searchParams.set("query", request.libraryName);
        const headers = generateHeaders();
        const response = await fetch(url, { headers });
        if (!response.ok) {
            const errorCode = response.status;
            if (errorCode === 429) {
                return {
                    content: [{ type: "text", text: "Rate limited due to too many requests. Please try again later." }]
                };
            }
            return {
                content: [{ type: "text", text: `Failed to search libraries. Please try again later. Error code: ${errorCode}` }]
            };
        }
        const searchResponse = await response.json();
        if (!searchResponse.results || searchResponse.results.length === 0) {
            return {
                content: [{ type: "text", text: searchResponse.error || "No libraries found matching your query." }]
            };
        }
        const resultsText = formatSearchResults(searchResponse);
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
    }
    catch (error) {
        return {
            content: [{ type: "text", text: `Error searching libraries: ${error}` }]
        };
    }
}
// Tool function: Get library documentation
async function get_library_documentation(request, _env) {
    const tokens = request.tokens && request.tokens >= DEFAULT_MINIMUM_TOKENS
        ? request.tokens
        : DEFAULT_MINIMUM_TOKENS;
    const topic = request.topic || "";
    try {
        let libraryId = request.context7CompatibleLibraryID;
        if (libraryId.startsWith("/")) {
            libraryId = libraryId.slice(1);
        }
        const url = new URL(`${CONTEXT7_API_BASE_URL}/v1/${libraryId}`);
        if (tokens)
            url.searchParams.set("tokens", tokens.toString());
        if (topic)
            url.searchParams.set("topic", topic);
        url.searchParams.set("type", DEFAULT_TYPE);
        const headers = generateHeaders(undefined, { "X-Context7-Source": "jilebi-plugin" });
        const response = await fetch(url, { headers });
        if (!response.ok) {
            const errorCode = response.status;
            if (errorCode === 429) {
                const errorMessage = "Rate limited due to too many requests. Please try again later.";
                return {
                    content: [{ type: "text", text: errorMessage }]
                };
            }
            const errorMessage = `Failed to fetch documentation. Please try again later. Error code: ${errorCode}`;
            return {
                content: [{ type: "text", text: errorMessage }]
            };
        }
        const text = await response.text();
        if (!text || text === "No content available" || text === "No context data available") {
            return {
                content: [{
                        type: "text",
                        text: "Documentation not found or not finalized for this library. This might have happened because you used an invalid Context7-compatible library ID. To get a valid Context7-compatible library ID, use the 'resolve-library-id' with the package name you wish to retrieve documentation for."
                    }]
            };
        }
        return {
            content: [{ type: "text", text: text }]
        };
    }
    catch (error) {
        return {
            content: [{ type: "text", text: `Error fetching library documentation. Please try again later. ${error}` }]
        };
    }
}

export { generateHeaders, get_library_documentation, resolve_library_id };
//# sourceMappingURL=index.js.map
