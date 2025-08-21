# AGENTS: beefy-light-client

Quick guide for contributors and AI coding agents working in this crate.

## Scope
See the `[package]` section in this crate’s `Cargo.toml` for the package name and description.

## Required Rules
- Add at least one unit test for every new function you introduce.
- Keep documentation in sync with code changes (Rustdoc and any READMEs).

## Local Commands
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --features runtime-benchmarks -- -D warnings`
- Test (this crate): `cargo test -p <crate_name> --features runtime-benchmarks`

## Notes
- If testing pallet-like logic, use a mock runtime and `sp_io::TestExternalities` in `#[cfg(test)]` modules.
- For RPC/runtime-API companion crates in this folder, add unit tests for pure helpers and serialization/deserialization where applicable.

See the repository root `AGENTS.md` for full policies and checklist.

