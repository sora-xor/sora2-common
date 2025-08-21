# AGENTS: leaf-provider-rpc

Quick guide for contributors and AI coding agents working in this crate.

## Scope
JSON-RPC glue for the `leaf-provider` runtime API. See `Cargo.toml` for details.

## Required Rules
- Add at least one unit test for every new function you introduce.
- Keep documentation in sync with code changes (Rustdoc and any READMEs).

## Local Commands
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --features runtime-benchmarks -- -D warnings`
- Test (this crate): `cargo test -p leaf-provider-rpc --features runtime-benchmarks`

## Notes
- Add unit tests for request/response helpers and serialization boundaries.
- Avoid network-dependent tests; prefer pure unit tests and mocks.

See the repository root `AGENTS.md` for full policies and checklist.

