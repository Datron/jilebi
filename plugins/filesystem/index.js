// Jilebi Filesystem Plugin - Deno Implementation with MCP Compatible Returns
// All functions take (request, env) parameters as required by jilebi
// Utility functions
function formatSize(bytes) {
    const units = ['B', 'KB', 'MB', 'GB', 'TB'];
    if (bytes === 0)
        return '0 B';
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    if (i === 0)
        return `${bytes} ${units[i]}`;
    return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${units[i]}`;
}
function normalizeLineEndings(text) {
    return text.replace(/\r\n/g, '\n');
}
function createUnifiedDiff(originalContent, newContent, filepath = 'file') {
    const originalLines = normalizeLineEndings(originalContent).split('\n');
    const newLines = normalizeLineEndings(newContent).split('\n');
    const diffLines = [];
    diffLines.push(`--- ${filepath}`);
    diffLines.push(`+++ ${filepath}`);
    let originalIndex = 0;
    let newIndex = 0;
    while (originalIndex < originalLines.length || newIndex < newLines.length) {
        if (originalIndex < originalLines.length && newIndex < newLines.length) {
            if (originalLines[originalIndex] === newLines[newIndex]) {
                diffLines.push(` ${originalLines[originalIndex]}`);
                originalIndex++;
                newIndex++;
            }
            else {
                diffLines.push(`-${originalLines[originalIndex]}`);
                diffLines.push(`+${newLines[newIndex]}`);
                originalIndex++;
                newIndex++;
            }
        }
        else if (originalIndex < originalLines.length) {
            diffLines.push(`-${originalLines[originalIndex]}`);
            originalIndex++;
        }
        else {
            diffLines.push(`+${newLines[newIndex]}`);
            newIndex++;
        }
    }
    return diffLines.join('\n');
}
// File reading functions
export async function read_text_file(request, env) {
    try {
        const { path, head, tail } = request;
        if (head && tail) {
            throw new Error("Cannot specify both head and tail parameters simultaneously");
        }
        let content;
        if (tail) {
            content = await tailFile(path, tail);
        }
        else if (head) {
            content = await headFile(path, head);
        }
        else {
            // @ts-ignore
            content = await Deno.readTextFile(path);
        }
        return {
            content: [
                {
                    type: "text",
                    text: content
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to read file: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
export async function read_media_file(request, env) {
    try {
        const { path } = request;
        // @ts-ignore
        const data = await Deno.readFile(path);
        const base64Data = btoa(String.fromCharCode(...data));
        // Determine MIME type based on extension
        const extension = path.toLowerCase().split('.').pop();
        const mimeTypes = {
            'png': 'image/png',
            'jpg': 'image/jpeg',
            'jpeg': 'image/jpeg',
            'gif': 'image/gif',
            'webp': 'image/webp',
            'bmp': 'image/bmp',
            'svg': 'image/svg+xml',
            'mp3': 'audio/mpeg',
            'wav': 'audio/wav',
            'ogg': 'audio/ogg',
            'flac': 'audio/flac',
        };
        const mimeType = mimeTypes[extension || ''] || 'application/octet-stream';
        const result = {
            data: base64Data,
            mimeType: mimeType,
            size: data.length
        };
        return {
            content: [
                {
                    type: "text",
                    text: JSON.stringify(result, null, 2)
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to read media file: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
export async function read_multiple_files(request, env) {
    try {
        const { paths } = request;
        const results = [];
        for (const filePath of paths) {
            try {
                // @ts-ignore
                const content = await Deno.readTextFile(filePath);
                results.push(`${filePath}:\n${content}\n`);
            }
            catch (error) {
                // @ts-ignore
                results.push(`${filePath}: Error - ${error instanceof Error ? error.message : String(error)}`);
            }
        }
        return {
            content: [
                {
                    type: "text",
                    text: results.join("\n---\n")
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to read multiple files: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
// File writing functions
export async function write_file(request, env) {
    try {
        const { path, content } = request;
        // @ts-ignore
        await Deno.writeTextFile(path, content);
        return {
            content: [
                {
                    type: "text",
                    text: `Successfully wrote to ${path}`
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to write file: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
export async function edit_file(request, env) {
    try {
        const { path, edits, dryRun = false } = request;
        // Read file content
        // @ts-ignore
        const content = normalizeLineEndings(await Deno.readTextFile(path));
        // Apply edits sequentially
        let modifiedContent = content;
        for (const edit of edits) {
            const normalizedOld = normalizeLineEndings(edit.oldText);
            const normalizedNew = normalizeLineEndings(edit.newText);
            if (modifiedContent.includes(normalizedOld)) {
                modifiedContent = modifiedContent.replace(normalizedOld, normalizedNew);
                continue;
            }
            // Try line-by-line matching with flexibility for whitespace
            const oldLines = normalizedOld.split('\n');
            const contentLines = modifiedContent.split('\n');
            let matchFound = false;
            for (let i = 0; i <= contentLines.length - oldLines.length; i++) {
                const potentialMatch = contentLines.slice(i, i + oldLines.length);
                const isMatch = oldLines.every((oldLine, j) => {
                    const contentLine = potentialMatch[j];
                    return oldLine.trim() === contentLine?.trim();
                });
                if (isMatch) {
                    const originalIndent = contentLines[i]?.match(/^\s*/)?.[0] || '';
                    const newLines = normalizedNew.split('\n').map((line, j) => {
                        if (j === 0)
                            return originalIndent + line.trimStart();
                        const oldIndent = oldLines[j]?.match(/^\s*/)?.[0] || '';
                        const newIndent = line.match(/^\s*/)?.[0] || '';
                        if (oldIndent && newIndent) {
                            const relativeIndent = newIndent.length - oldIndent.length;
                            return originalIndent + ' '.repeat(Math.max(0, relativeIndent)) + line.trimStart();
                        }
                        return line;
                    });
                    contentLines.splice(i, oldLines.length, ...newLines);
                    modifiedContent = contentLines.join('\n');
                    matchFound = true;
                    break;
                }
            }
            if (!matchFound) {
                throw new Error(`Could not find exact match for edit:\n${edit.oldText}`);
            }
        }
        // Create unified diff
        const diff = createUnifiedDiff(content, modifiedContent, path);
        if (!dryRun) {
            // @ts-ignore
            await Deno.writeTextFile(path, modifiedContent);
        }
        return {
            content: [
                {
                    type: "text",
                    text: `\`\`\`diff\n${diff}\`\`\`\n\n${dryRun ? 'Dry run - changes not applied' : 'Changes applied successfully'}`
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to edit file: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
// Directory operations
export async function create_directory(request, env) {
    try {
        const { path } = request;
        // @ts-ignore
        await Deno.mkdir(path, { recursive: true });
        return {
            content: [
                {
                    type: "text",
                    text: `Successfully created directory ${path}`
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to create directory: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
export async function list_directory(request, env) {
    try {
        const { path } = request;
        const entries = [];
        // @ts-ignore
        for await (const entry of Deno.readDir(path)) {
            const prefix = entry.isDirectory ? "[DIR]" : "[FILE]";
            entries.push(`${prefix} ${entry.name}`);
        }
        return {
            content: [
                {
                    type: "text",
                    text: entries.join("\n")
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to list directory: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
export async function list_directory_with_sizes(request, env) {
    try {
        const { path, sortBy = 'name' } = request;
        const entries = [];
        // @ts-ignore
        for await (const entry of Deno.readDir(path)) {
            const entryPath = `${path}/${entry.name}`;
            let size = 0;
            try {
                if (entry.isFile) {
                    // @ts-ignore
                    const stat = await Deno.stat(entryPath);
                    size = stat.size;
                }
            }
            catch {
                // Ignore stat errors
            }
            entries.push({
                name: entry.name,
                isDirectory: entry.isDirectory,
                size: size
            });
        }
        // Sort entries
        if (sortBy === 'size') {
            entries.sort((a, b) => b.size - a.size);
        }
        else {
            entries.sort((a, b) => a.name.localeCompare(b.name));
        }
        // Format output
        const formattedEntries = entries.map(entry => {
            const prefix = entry.isDirectory ? "[DIR]" : "[FILE]";
            const sizeStr = entry.isDirectory ? "" : formatSize(entry.size).padStart(10);
            return `${prefix} ${entry.name.padEnd(30)} ${sizeStr}`;
        });
        const totalFiles = entries.filter(e => !e.isDirectory).length;
        const totalDirs = entries.filter(e => e.isDirectory).length;
        const totalSize = entries.reduce((sum, entry) => sum + (entry.isDirectory ? 0 : entry.size), 0);
        const summary = [
            "",
            `Total: ${totalFiles} files, ${totalDirs} directories`,
            `Combined size: ${formatSize(totalSize)}`
        ];
        return {
            content: [
                {
                    type: "text",
                    text: [...formattedEntries, ...summary].join("\n")
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to list directory with sizes: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
export async function directory_tree(request, env) {
    try {
        const { path } = request;
        async function buildTree(currentPath) {
            const result = [];
            // @ts-ignore
            for await (const entry of Deno.readDir(currentPath)) {
                const entryData = {
                    name: entry.name,
                    type: entry.isDirectory ? 'directory' : 'file'
                };
                if (entry.isDirectory) {
                    const subPath = `${currentPath}/${entry.name}`;
                    entryData.children = await buildTree(subPath);
                }
                result.push(entryData);
            }
            return result;
        }
        const treeData = await buildTree(path);
        return {
            content: [
                {
                    type: "text",
                    text: JSON.stringify(treeData, null, 2)
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to create directory tree: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
// File operations
export async function move_file(request, env) {
    try {
        const { source, destination } = request;
        // @ts-ignore
        await Deno.rename(source, destination);
        return {
            content: [
                {
                    type: "text",
                    text: `Successfully moved ${source} to ${destination}`
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to move file: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
export async function search_files(request, env) {
    try {
        const { path, pattern, excludePatterns = [] } = request;
        const results = [];
        async function search(currentPath) {
            // @ts-ignore
            for await (const entry of Deno.readDir(currentPath)) {
                const fullPath = `${currentPath}/${entry.name}`;
                // Check if should exclude
                const shouldExclude = excludePatterns.some((excludePattern) => {
                    return entry.name.toLowerCase().includes(excludePattern.toLowerCase());
                });
                if (shouldExclude) {
                    continue;
                }
                // Check if matches pattern
                if (entry.name.toLowerCase().includes(pattern.toLowerCase())) {
                    results.push(fullPath);
                }
                // Recurse into directories
                if (entry.isDirectory) {
                    try {
                        await search(fullPath);
                    }
                    catch {
                        // Skip directories we can't access
                    }
                }
            }
        }
        await search(path);
        return {
            content: [
                {
                    type: "text",
                    text: results.length > 0 ? results.join("\n") : "No matches found"
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to search files: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
export async function get_file_info(request, env) {
    try {
        const { path } = request;
        // @ts-ignore
        const stat = await Deno.stat(path);
        const info = {
            size: stat.size,
            created: stat.birthtime || stat.mtime,
            modified: stat.mtime,
            accessed: stat.atime || stat.mtime,
            isDirectory: stat.isDirectory,
            isFile: stat.isFile,
            permissions: stat.mode ? stat.mode.toString(8).slice(-3) : '644'
        };
        const infoText = Object.entries(info)
            .map(([key, value]) => `${key}: ${value}`)
            .join("\n");
        return {
            content: [
                {
                    type: "text",
                    text: infoText
                }
            ]
        };
    }
    catch (error) {
        return {
            content: [
                {
                    type: "text",
                    text: `Failed to get file info: ${error instanceof Error ? error.message : String(error)}`
                }
            ],
            isError: true
        };
    }
}
// Helper functions for head/tail operations
async function headFile(filePath, numLines) {
    // @ts-ignore
    const content = await Deno.readTextFile(filePath);
    const lines = content.split('\n');
    return lines.slice(0, numLines).join('\n');
}
async function tailFile(filePath, numLines) {
    // @ts-ignore
    const content = await Deno.readTextFile(filePath);
    const lines = content.split('\n');
    return lines.slice(-numLines).join('\n');
}
