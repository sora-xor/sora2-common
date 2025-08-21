# beefy-light-client-runtime-api

Runtime API for the Beefy Light Client pallet.

## What it does
- Exposes helpers for off-chain callers, such as computing a random validator bitfield from a prior bitfield and validator count.

## Methods
- `get_random_bitfield(network_id, prior, num_of_validators) -> Bitfield`

## Testing
- Run: `cargo test -p beefy-light-client-runtime-api --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
