# leaf-provider-runtime-api

Runtime API for the Leaf Provider pallet.

## What it does
- Exposes the latest auxiliary digest collected by the pallet.

## Methods
- `latest_digest() -> Option<AuxiliaryDigest>`

## Testing
- Run: `cargo test -p leaf-provider-runtime-api --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
