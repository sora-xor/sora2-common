# ton-bridge

Bridge manager pallet for TON networks.

## What it does
- Manages bridging of fungible Jetton/native TON assets.
- Coordinates with channel and signer pallets to execute cross-chain flows.

## Feature flags
- `std`, `runtime-benchmarks`, `try-runtime`.

## Testing
- Run: `cargo test -p ton-bridge --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
