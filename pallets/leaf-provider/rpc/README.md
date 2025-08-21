# leaf-provider-rpc

JSON-RPC server/client for the Leaf Provider runtime API.

## What it does
- Exposes the `leafProvider_latestDigest` RPC to query the latest auxiliary digest.

## Usage
- Integrate in node service to register RPC module and forward to the runtime API.

## Testing
- Run: `cargo test -p leaf-provider-rpc --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
