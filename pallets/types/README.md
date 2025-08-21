# bridge-types

Shared types and traits used by bridge pallets and utilities.

## What it includes
- Network identifiers (`GenericNetworkId`, `SubNetworkId`), EVM/Ton types, address and balance newtypes.
- Multisig and message types, codec helpers, and utilities.

## Feature flags
- `std`: Standard library support.
- `test`: Enables test utilities.
- `runtime-benchmarks`: Available for dependent crates.

## Testing
- Run: `cargo test -p bridge-types --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
