# bridge-signer

Bridge signer pallet that manages peers and verifies multisignature approvals for inbound bridge calls.

## What it does
- Maintains peer sets for multiple networks (Substrate, EVM, TON).
- Verifies and finalizes add/remove peer operations and provides signing-related helpers for bridge calls.

## Feature flags
- `std`: Standard library support.
- `runtime-benchmarks`: Enables benchmarking helpers.
- `try-runtime`: Enables try-runtime hooks if used by the runtime.

## Testing
- Run: `cargo test -p bridge-signer --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
