# parachain-app

Pallet for bridging parachain/relaychain assets with SORA.

## What it does
- Handles bridged transfers for parachain/relaychain tokens.
- Emits and reacts to bridge events; uses channels for messaging.

## Feature flags
- `std`, `runtime-benchmarks`, `try-runtime`.

## Testing
- Run: `cargo test -p parachain-app --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
