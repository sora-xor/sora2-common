# evm-fungible-app

Pallet to bridge fungible EVM assets (ERC-20 and native) with SORA.

## What it does
- Burns/mints balances to reflect cross-chain transfers.
- Emits and reacts to events relayed by an external relayer.
- Uses bridge channels for messaging to and from EVM chains.

## Feature flags
- `std`, `runtime-benchmarks`, `try-runtime`.

## Testing
- Run: `cargo test -p evm-fungible-app --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
