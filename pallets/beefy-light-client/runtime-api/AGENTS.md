# AGENTS: beefy-light-client-runtime-api

Quick guide for contributors and AI coding agents working in this crate.

## Scope
This crate defines the runtime API for the beefy-light-client. See `Cargo.toml` for details.

## Required Rules
- Add at least one unit test for every new function you introduce.
- Keep documentation in sync with code changes (Rustdoc and any READMEs).

## Local Commands
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --features runtime-benchmarks -- -D warnings`
- Test (this crate): `cargo test -p beefy-light-client-runtime-api --features runtime-benchmarks`

## Notes
- Focus tests on type/codec stability and API helper logic.
- Runtime API crates are `no_std` by default; ensure tests enable the right features if required.

See the repository root `AGENTS.md` for full policies and checklist.

