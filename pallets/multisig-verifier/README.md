# multisig-verifier

Pallet utilities to verify multisignature decisions for bridge operations across networks.

## What it does
- Tracks/validates multisignature approvals for bridge governance actions.
- Provides helper types for EVM, Substrate, and TON peer sets.

## Feature flags
- `std`: Standard library support.
- `runtime-benchmarks`: Enables benchmarking helpers.
- `try-runtime`: Enables try-runtime hooks if used by the runtime.

## Testing
- Run: `cargo test -p multisig-verifier --features runtime-benchmarks`

See repository root `AGENTS.md` for contribution rules (tests per function and doc-sync).
