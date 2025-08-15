import { defineConfig } from "rollup";
import typescript from "@rollup/plugin-typescript";
import resolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import { builtinModules } from "module";

export default defineConfig({
	input: "index.ts",
	output: {
		file: "index.js",
		format: "es",
		sourcemap: true,
	},
	plugins: [
		// Resolve node modules
		resolve({
			preferBuiltins: true,
			exportConditions: ["node"],
		}),

		// Convert CommonJS modules to ES6
		commonjs(),

		// Handle JSON imports
		json(),

		// TypeScript compilation
		typescript({
			tsconfig: "./tsconfig.json",
			sourceMap: true,
			inlineSources: true,
		}),
	],

	// External dependencies that should not be bundled
	external: [
		// Node.js built-in modules
		...builtinModules,
		...builtinModules.map((m) => `node:${m}`),
	],

	// Suppress warnings for certain cases
	onwarn(warning, warn) {
		// Suppress "this is undefined" warnings
		if (warning.code === "THIS_IS_UNDEFINED") return;

		// Suppress circular dependency warnings for known safe cases
		if (warning.code === "CIRCULAR_DEPENDENCY") return;

		// Use default for everything else
		warn(warning);
	},
});
