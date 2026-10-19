#!/usr/bin/env node
// Vendor the jilebi OpenAPI document at sdks/openapi.json.
//
// The public endpoint sits behind Cloudflare which fingerprints the TLS client,
// so this shells out to curl instead of using the built-in fetch. The license is
// injected here because the API does not advertise one but every generated SDK
// needs it.
import { execFileSync } from "node:child_process";
import { writeFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const SPEC_URL =
	process.env.JILEBI_OPENAPI_URL ?? "https://jilebi.ai/api/openapi.json";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..", "..");
const outFile = resolve(repoRoot, "sdks", "openapi.json");

const raw = execFileSync(
	"curl",
	[
		"-fsSL",
		"-H",
		"User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0 Safari/537.36",
		"-H",
		"Accept: application/json,text/plain,*/*",
		SPEC_URL,
	],
	{ encoding: "utf8", maxBuffer: 64 * 1024 * 1024 },
);

const spec = JSON.parse(raw);

spec.info = {
	...spec.info,
	license: {
		name: "AGPL-3.0",
		url: "https://www.gnu.org/licenses/agpl-3.0.html",
	},
};

writeFileSync(outFile, `${JSON.stringify(spec, null, 2)}\n`);

console.log(
	`Wrote ${outFile} (openapi ${spec.openapi}, version ${spec.info.version})`,
);
