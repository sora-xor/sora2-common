# beefy-light-client-rpc

JSON-RPC server/client for the Beefy Light Client runtime API.

## What it does
- Exposes RPC methods to fetch helper data from the runtime (e.g., random validator bitfields).
- Wraps `beefy-light-client-runtime-api` for network access.

## Usage
- Integrate in node service to register RPC module.
- Avoid network-dependent tests; focus on serialization and request/response helpers.

## Testing
- Run: `cargo test -p beefy-light-client-rpc --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
