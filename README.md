# sora2-common

Common crates and Substrate pallets used across the SORA bridge stack. This is a Rust workspace containing pallets, runtime APIs, and RPC adapters for cross‑chain messaging and verification.

See `AGENTS.md` for contributor guidelines (tests per new function and doc-sync requirements).

## Workspace
- Workspace members are defined in the root `Cargo.toml` and live in `pallets/*`.
- Many pallets have companion `runtime-api` and `rpc` crates.
- Most crates support `std`, `runtime-benchmarks`, and `try-runtime` feature flags.

Quick commands:
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --features runtime-benchmarks -- -D warnings`
- Test: `cargo test --all --features runtime-benchmarks`

## Status & Roadmap
- Current status: see `STATUS.md`.
- Prioritized tasks: see `ROADMAP.md`.

## Release process
The repository adopts [GitHub Flow](https://docs.github.com/en/get-started/quickstart/github-flow) model with `develop` branch as default.
The `develop` branch is anticipated to be the latest stable version.
- Create branch `release/X.Y.Z` from `develop`, use [semver](https://semver.org/)
- `cargo update`
- Merge `release/X.Y.Z` into `develop`
- Create GitHub Release on `develop` branch, tag is `X.Y.Z`
