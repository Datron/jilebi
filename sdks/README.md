# jilebi SDKs

Client SDKs for the [jilebi API](https://jilebi.ai/api/openapi.json), generated
with [OpenAPI Generator](https://openapi-generator.tech) from the vendored
spec at [`sdks/openapi.json`](./openapi.json).

| Language   | Generator         | Package / module                        | Registry                       |
| ---------- | ----------------- | --------------------------------------- | ------------------------------ |
| Rust       | `rust`            | `jilebi-sdk`                            | [crates.io](https://crates.io) |
| TypeScript | `typescript-fetch`| `@jilebi/sdk`                           | [npm](https://www.npmjs.com)   |
| Python     | `python`          | `jilebi-sdk`                            | [PyPI](https://pypi.org)       |
| Go         | `go`              | `github.com/Datron/jilebi/sdks/go`      | not published                  |

> The Go SDK is generated for completeness but is **not published** — it is
> consumed directly from this repository.

## Layout

```
sdks/
├── openapi.json          # vendored OpenAPI 3.1 document
├── scripts/
│   ├── fetch-spec.mjs    # refreshes openapi.json from jilebi.ai
│   └── set-version.mjs   # stamps one version across every SDK
├── rust/                 # cargo workspace member (crate jilebi-sdk)
├── typescript/           # npm package @jilebi/sdk
├── python/               # PyPI package jilebi-sdk
└── go/                   # Go module github.com/Datron/jilebi/sdks/go (not published)
```

The Rust crate is a member of the root cargo workspace (`sdks/rust`).

## Versioning

Every SDK shares **one version**, and that version is the MCP server release
tag created by [`.github/workflows/generate_versions.yaml`](../.github/workflows/generate_versions.yaml)
(plain semver such as `0.6.2`).

```bash
# Stamp the SDKs with the latest MCP server tag
npm run sdk:version

# ...or with an explicit version
npm run sdk:version 0.7.0
```

`set-version.mjs` writes the version into the Rust `Cargo.toml`, the TypeScript
`package.json`, the Python `pyproject.toml` / `setup.py` / `__init__.py`, the Go
client, and `openapitools.json` (so a regeneration does not reset it).
`npm run sdk:generate` runs it automatically at the end.

## Regenerating

The generator is a dev dependency of the repo (`@openapitools/openapi-generator-cli`,
pinned in `openapitools.json`) and requires a Java runtime.

```bash
# 1. Refresh the vendored spec (optional)
npm run sdk:spec

# 2. Regenerate every SDK (and re-stamp the shared version)
npm run sdk:generate

# ...or a single language
npm run sdk:generate:rust
npm run sdk:generate:typescript
npm run sdk:generate:python
npm run sdk:generate:go
```

All generator options live in [`openapitools.json`](../openapitools.json) under
`generator-cli.generators`, so the npm scripts stay tiny.

## Building locally

```bash
# Rust
cargo build -p jilebi-sdk

# TypeScript
cd sdks/typescript && npm install && npm run build

# Python
cd sdks/python && python -m build

# Go
cd sdks/go && go build ./...
```

## Releasing

Releases are handled by
[`.github/workflows/sdk_release.yaml`](../.github/workflows/sdk_release.yaml).
It runs whenever an MCP server release tag (a `[0-9]*` semver tag) is pushed —
the same tag that `cog bump` produces — and stamps the SDKs with that version
before publishing. It can also be dispatched manually, in which case the latest
tag (or a `version` input) is used.

The Go SDK is intentionally left out of the release pipeline and is not
published anywhere.

Required repository secrets:

| Secret                 | Used for                       |
| ---------------------- | ------------------------------ |
| `CARGO_REGISTRY_TOKEN` | `cargo publish` to crates.io   |
| `PYPI_API_TOKEN`       | upload to PyPI                 |

npm uses **trusted publishing (OIDC)** — no `NPM_TOKEN` secret. The workflow
grants `id-token: write` and the trusted publisher must be registered for
`@jilebi/sdk` on npmjs.com:

| Field | Value |
| --- | --- |
| Organization or user | `Datron` |
| Repository | `jilebi` |
| Workflow filename | `sdk_release.yaml` |
| Environment | *(leave blank)* |
| Allowed actions | **Allow `npm publish`** (new configs default to `npm stage publish` only) |

A newly created trusted publisher must complete its first successful publish
within 2 days or it expires. Provenance attestations are generated
automatically; no `--provenance` flag is needed.

```bash
# Normal flow: bump the MCP server version, which tags and releases the SDKs
# (run the "Create jilebi version" workflow). Or, to release manually:
git tag 0.7.0
git push origin 0.7.0
```
