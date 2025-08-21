# leaf-provider

Substrate pallet that produces a standardized BEEFY+MMR leaf with extra data for bridging.

## What it does
- Collects auxiliary digest items from other pallets and encodes them into a hash included in the MMR leaf extra.
- Provides randomness derived from the MMR leaf extra for use by other pallets (e.g., randomized bitfields).
- Clears the digest per block and implements the BEEFY data provider trait.

## Feature flags
- `std`, `runtime-benchmarks`, `try-runtime`.

## Testing
- Run: `cargo test -p leaf-provider --features runtime-benchmarks`
- Tests should use a mock runtime and `sp_io::TestExternalities` to exercise hooks and data provider behavior.

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
