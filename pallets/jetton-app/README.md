# jetton-app

Pallet to bridge fungible TON assets (Jettons and native) with SORA.

## What it does
- Handles incoming TON messages to mint assets on SORA.
- Sends outgoing messages over bridge channels for redemptions.

## Feature flags
- `std`, `runtime-benchmarks`, `try-runtime`.

## Testing
- Run: `cargo test -p jetton-app --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
