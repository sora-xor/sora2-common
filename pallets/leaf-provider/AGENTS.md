# AGENTS: leaf-provider

Quick guide for contributors and AI coding agents working in this crate.

## Scope
Provides a BEEFY+MMR leaf with extra data for bridging. See `Cargo.toml` and `src/lib.rs` for details.

## Required Rules
- Add at least one unit test for every new function you introduce.
- Keep documentation in sync with code changes (Rustdoc and any READMEs).

## Local Commands
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --features runtime-benchmarks -- -D warnings`
- Test (this crate): `cargo test -p leaf-provider --features runtime-benchmarks`

## Notes
- This is a Substrate pallet. Use a mock runtime and `sp_io::TestExternalities` for tests.
- Keep `#![warn(missing_docs)]` satisfied by updating Rustdoc when changing public items.

See the repository root `AGENTS.md` for full policies and checklist.

