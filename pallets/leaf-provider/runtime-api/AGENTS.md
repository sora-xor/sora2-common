# AGENTS: leaf-provider-runtime-api

Quick guide for contributors and AI coding agents working in this crate.

## Scope
Runtime API for the `leaf-provider` pallet. See `Cargo.toml` for details.

## Required Rules
- Add at least one unit test for every new function you introduce.
- Keep documentation in sync with code changes (Rustdoc and any READMEs).

## Local Commands
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --features runtime-benchmarks -- -D warnings`
- Test (this crate): `cargo test -p leaf-provider-runtime-api --features runtime-benchmarks`

## Notes
- Focus tests on type/codec stability and helper logic.
- Runtime API crates are often `no_std`; ensure features are set correctly for tests.

See the repository root `AGENTS.md` for full policies and checklist.

