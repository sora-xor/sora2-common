# bridge-common

Shared bridge utilities and simplified proof helpers reused by multiple pallets.

## What it does
- Exposes shared types, simplified MMR proof helpers, and utilities used by bridge pallets.
- Keeps common logic in one place to reduce duplication.

## Feature flags
- `std`: Standard library support.

## Testing
- Run: `cargo test -p bridge-common --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
