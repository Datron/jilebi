class SequentialThinkingProcessor {
    thoughtHistory = [];
    branches = {};
    disableThoughtLogging;
    constructor() {
        this.disableThoughtLogging = true;
    }
    validateThoughtData(input) {
        if (!input.thought || typeof input.thought !== 'string') {
            throw new Error('Invalid thought: must be a string');
        }
        if (!input.thoughtNumber || typeof input.thoughtNumber !== 'number') {
            throw new Error('Invalid thoughtNumber: must be a number');
        }
        if (!input.totalThoughts || typeof input.totalThoughts !== 'number') {
            throw new Error('Invalid totalThoughts: must be a number');
        }
        if (typeof input.nextThoughtNeeded !== 'boolean') {
            throw new Error('Invalid nextThoughtNeeded: must be a boolean');
        }
        return {
            thought: input.thought,
            thoughtNumber: input.thoughtNumber,
            totalThoughts: input.totalThoughts,
            nextThoughtNeeded: input.nextThoughtNeeded,
            isRevision: input.isRevision,
            revisesThought: input.revisesThought,
            branchFromThought: input.branchFromThought,
            branchId: input.branchId,
            needsMoreThoughts: input.needsMoreThoughts,
        };
    }
    formatThought(thoughtData) {
        const { thoughtNumber, totalThoughts, thought, isRevision, revisesThought, branchFromThought, branchId } = thoughtData;
        let prefix = '';
        let context = '';
        if (isRevision) {
            prefix = '🔄 Revision';
            context = ` (revising thought ${revisesThought})`;
        }
        else if (branchFromThought) {
            prefix = '🌿 Branch';
            context = ` (from thought ${branchFromThought}, ID: ${branchId})`;
        }
        else {
            prefix = '💭 Thought';
            context = '';
        }
        const header = `${prefix} ${thoughtNumber}/${totalThoughts}${context}`;
        const border = '─'.repeat(Math.max(header.length, thought.length) + 4);
        return `
┌${border}┐
│ ${header} │
├${border}┤
│ ${thought.padEnd(border.length - 2)} │
└${border}┘`;
    }
    processThought(input) {
        try {
            const validatedInput = this.validateThoughtData(input);
            if (validatedInput.thoughtNumber > validatedInput.totalThoughts) {
                validatedInput.totalThoughts = validatedInput.thoughtNumber;
            }
            this.thoughtHistory.push(validatedInput);
            if (validatedInput.branchFromThought && validatedInput.branchId) {
                if (!this.branches[validatedInput.branchId]) {
                    this.branches[validatedInput.branchId] = [];
                }
                this.branches[validatedInput.branchId].push(validatedInput);
            }
            if (!this.disableThoughtLogging) {
                const formattedThought = this.formatThought(validatedInput);
                console.error(formattedThought);
            }
            return {
                content: [{
                        type: "text",
                        text: JSON.stringify({
                            thoughtNumber: validatedInput.thoughtNumber,
                            totalThoughts: validatedInput.totalThoughts,
                            nextThoughtNeeded: validatedInput.nextThoughtNeeded,
                            branches: Object.keys(this.branches),
                            thoughtHistoryLength: this.thoughtHistory.length
                        }, null, 2)
                    }]
            };
        }
        catch (error) {
            return {
                content: [{
                        type: "text",
                        text: JSON.stringify({
                            error: error instanceof Error ? error.message : String(error),
                            status: 'failed'
                        }, null, 2)
                    }],
                isError: true
            };
        }
    }
}
// Global instance for processing thoughts
const processor = new SequentialThinkingProcessor();
/**
 * Sequential thinking tool for dynamic and reflective problem-solving
 * @param request - The request object containing thinking parameters
 * @param env - Environment object (unused in this implementation)
 * @returns MCP-compliant result with thinking process information
 */
function sequential_thinking(request, env) {
    try {
        // Validate required parameters
        if (!request.thought || typeof request.thought !== 'string') {
            throw new Error('Thought parameter is required and must be a string');
        }
        if (typeof request.nextThoughtNeeded !== 'boolean') {
            throw new Error('nextThoughtNeeded parameter is required and must be a boolean');
        }
        if (!request.thoughtNumber || typeof request.thoughtNumber !== 'number') {
            throw new Error('thoughtNumber parameter is required and must be a number');
        }
        if (!request.totalThoughts || typeof request.totalThoughts !== 'number') {
            throw new Error('totalThoughts parameter is required and must be a number');
        }
        return processor.processThought(request);
    }
    catch (error) {
        const errorMessage = error instanceof Error ? error.message : String(error);
        return {
            content: [
                {
                    type: "text",
                    text: `Error processing sequential thinking: ${errorMessage}`
                }
            ],
            isError: true
        };
    }
}
// Export the functions and types for use in other modules
export { sequential_thinking, };
