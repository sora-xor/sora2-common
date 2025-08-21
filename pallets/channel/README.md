# bridge-channel

Message channel primitives used by bridge apps for inbound/outbound message handling.

## What it does
- Provides `inbound` and `outbound` modules for channel management.
- Used by higher-level bridge apps to send/receive messages over lanes.

## Testing
- Run: `cargo test -p bridge-channel --features runtime-benchmarks`

See root `AGENTS.md` for contribution rules (tests per function and doc-sync).
