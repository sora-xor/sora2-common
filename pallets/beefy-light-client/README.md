# beefy-light-client

Beefy (BEEFY) light client pallet for verifying BEEFY commitments and tracking MMR roots across bridged networks.

## What it does
- Stores the latest MMR roots per bridged network and the latest verified BEEFY block.
- Verifies signature commitments from external BEEFY validators, including bitfields and proofs.
- Applies validator set changes and exposes randomness derived from the MMR leaf extra.

## Key concepts
- Storage: recent MMR roots, latest BEEFY block number, current/next validator sets, per-network randomness seeds.
- Extrinsics: initialize pallet state; submit signature commitment with proof and latest MMR leaf.
- Events/Errors: emitted on successful verification and when proof/parameters are invalid.

## Feature flags
- `std`: Standard library support for off-chain environments.
- `runtime-benchmarks`: Enables benchmarking helpers.
- `try-runtime`: Enables try-runtime hooks if used by the runtime.

## Testing
- Run: `cargo test -p beefy-light-client --features runtime-benchmarks`
- Typical tests cover: proof verification, validator bitfields, MMR root tracking.

## Integration
- Include the pallet in a runtime and provide `Randomness` and storage weights.
- Use the runtime API/RPC companion crates to query helper functions (e.g., random bitfields from a prior bitfield).

See repository root `AGENTS.md` for contribution rules (tests per function and doc-sync).
