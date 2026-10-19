# jilebi SDKs

Client SDKs for the [jilebi API](https://jilebi.ai/api/openapi.json), generated
with [OpenAPI Generator](https://openapi-generator.tech) from the vendored
spec at [`sdks/openapi.json`](./openapi.json).

| Language   | Generator         | Package            | Registry                |
| ---------- | ----------------- | ------------------ | ----------------------- |
| Rust       | `rust`            | `jilebi-sdk`       | [crates.io](https://crates.io) |
| TypeScript | `typescript-fetch`| `@jilebi/sdk`      | [npm](https://www.npmjs.com) |
| Python     | `python`          | `jilebi-sdk`       | [PyPI](https://pypi.org) |

## Layout

```
sdks/
├── openapi.json          # vendored OpenAPI 3.1 document
├── scripts/
│   └── fetch-spec.mjs    # refreshes openapi.json from jilebi.ai
├── rust/                 # cargo workspace member (crate jilebi-sdk)
├── typescript/           # npm package @jilebi/sdk
└── python/               # PyPI package jilebi-sdk
```

The Rust crate is a member of the root cargo workspace (`sdks/rust`).

## Regenerating

The generator is a dev dependency of the repo (`@openapitools/openapi-generator-cli`,
pinned in `openapitools.json`) and requires a Java runtime.

```bash
# 1. Refresh the vendored spec (optional)
npm run sdk:spec

# 2. Regenerate every SDK
npm run sdk:generate

# ...or a single language
npm run sdk:generate:rust
npm run sdk:generate:typescript
npm run sdk:generate:python
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
```

## Releasing

Releases are handled by
[`.github/workflows/sdk_release.yaml`](../.github/workflows/sdk_release.yaml),
which is triggered by a tag matching `sdk-v*` (for example `sdk-v1.0.0`) or by a
manual `workflow_dispatch`. The version is taken from the tag and written into
each package manifest before publishing.

Required repository secrets:

| Secret                 | Used for                       |
| ---------------------- | ------------------------------ |
| `CARGO_REGISTRY_TOKEN` | `cargo publish` to crates.io   |
| `NPM_TOKEN`            | `npm publish` to npmjs.com     |
| `PYPI_API_TOKEN`       | upload to PyPI                 |

```bash
git tag sdk-v1.0.0
git push origin sdk-v1.0.0
```
