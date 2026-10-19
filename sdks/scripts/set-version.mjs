#!/usr/bin/env node
/**
 * Stamp a single version across every generated SDK.
 *
 * The version defaults to the most recent git tag, i.e. the MCP server release
 * tag created by `.github/workflows/generate_versions.yaml`, so all SDKs stay in
 * lockstep with the server:
 *
 *   node sdks/scripts/set-version.mjs            # use the latest tag
 *   node sdks/scripts/set-version.mjs 1.2.3      # or an explicit version
 *
 * `openapitools.json` is updated too so a later regeneration does not reset the
 * version back to the generator default.
 */
import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..");

function latestTag() {
	// Mirrors the other workflows: newest tag by creation date, regardless of
	// whether it is reachable from the current commit.
	const tags = execFileSync("git", ["tag", "-l", "--sort=-creatordate"], {
		cwd: repoRoot,
		encoding: "utf8",
	})
		.split("\n")
		.map((tag) => tag.trim())
		.filter(Boolean);

	if (tags.length === 0) {
		throw new Error("No git tags found; pass a version explicitly.");
	}

	return tags[0];
}

const requested = (process.argv[2] ?? "").trim() || latestTag();
const version = requested.replace(/^v/, "");

if (!/^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.-]+)?$/.test(version)) {
	console.error(`Invalid semver version: ${requested}`);
	process.exit(1);
}

// [file, pattern, replacement]
const updates = [
	// Rust
	["sdks/rust/Cargo.toml", /^version = ".*"$/m, `version = "${version}"`],
	["sdks/rust/README.md", /^(- Package version: ).*$/m, `$1${version}`],
	// TypeScript
	["sdks/typescript/package.json", /("version":\s*")[^"]*(")/, `$1${version}$2`],
	["sdks/typescript/README.md", /^(# @jilebi\/sdk@).*$/m, `$1${version}`],
	[
		"sdks/typescript/README.md",
		/^(- Package version: `)[^`]*(`)$/m,
		`$1${version}$2`,
	],
	// Python
	["sdks/python/pyproject.toml", /^version = ".*"$/m, `version = "${version}"`],
	["sdks/python/setup.py", /^VERSION = ".*"$/m, `VERSION = "${version}"`],
	[
		"sdks/python/jilebi_sdk/__init__.py",
		/^__version__ = ".*"$/m,
		`__version__ = "${version}"`,
	],
	[
		"sdks/python/jilebi_sdk/api_client.py",
		/(OpenAPI-Generator\/)[^']*(\/python')/,
		`$1${version}$2`,
	],
	[
		"sdks/python/jilebi_sdk/configuration.py",
		/SDK Package Version: [^\s"]+/,
		`SDK Package Version: ${version}`,
	],
	["sdks/python/README.md", /^(- Package version: ).*$/m, `$1${version}`],
	// Go
	[
		"sdks/go/configuration.go",
		/(UserAgent:\s*"OpenAPI-Generator\/)[^"]*(\/go")/,
		`$1${version}$2`,
	],
	["sdks/go/README.md", /^(- Package version: ).*$/m, `$1${version}`],
	// Generator config, so regeneration keeps the same version
	["openapitools.json", /"packageVersion": "[^"]*"/g, `"packageVersion": "${version}"`],
	["openapitools.json", /"npmVersion": "[^"]*"/g, `"npmVersion": "${version}"`],
];

console.log(`Setting the SDK version to ${version}`);

for (const [relativePath, pattern, replacement] of updates) {
	const file = resolve(repoRoot, relativePath);
	const before = readFileSync(file, "utf8");

	if (!before.match(pattern)) {
		throw new Error(`Version pattern not found in ${relativePath}`);
	}

	const after = before.replace(pattern, replacement);
	if (after === before) {
		continue;
	}

	writeFileSync(file, after);
	console.log(`  updated ${relativePath}`);
}
