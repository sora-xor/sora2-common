# AGENTS: types (bridge-types)

Quick guide for contributors and AI coding agents working in this crate.

## Scope
Shared types and traits for bridge-related pallets. See `Cargo.toml` and `src/` for details.

## Required Rules
- Add at least one unit test for every new function you introduce.
- Keep documentation in sync with code changes (Rustdoc and any READMEs).

## Local Commands
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --features runtime-benchmarks -- -D warnings`
- Test (this crate): `cargo test -p bridge-types --features runtime-benchmarks`

## Notes
- Many items are `no_std` compatible; ensure feature flags are correct in tests.
- Favor pure, deterministic tests and codec round-trip assertions.

See the repository root `AGENTS.md` for full policies and checklist.

