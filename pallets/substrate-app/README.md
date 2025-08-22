# substrate-bridge-app

Pallet for bridging Substrate-based assets with SORA.

## What it does
- Handles bridged transfers for tokens on Substrate networks.
- Emits and reacts to bridge events; uses channels for messaging.

## Feature flags
- `std`, `runtime-benchmarks`, `try-runtime`.

## Testing
- Run: `cargo test -p substrate-bridge-app --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
