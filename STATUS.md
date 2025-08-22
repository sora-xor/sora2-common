# Project Status — sora2-common

This document reflects the current, practical status of the codebase so engineers can quickly understand what’s solid, what needs love, and where to focus next. Keep this file updated with code changes.

## Summary
- Workspace of Substrate pallets, runtime APIs, RPC crates, and shared types for SORA bridge functionality.
- Formatting, linting, and tests are enforced in CI (clippy with `-D warnings`; tests run with `runtime-benchmarks`).
- Crate-level documentation exists for all crates and is wired into rustdoc via README includes.

## Build & CI
- GitHub Actions: cargo fmt check and clippy (with `runtime-benchmarks`).
- Jenkins: runs `housekeeping/tests.sh` which executes `cargo test --release --features runtime-benchmarks`.
- Toolchain: `rust-toolchain.toml` is present; Substrate deps are pinned to `polkadot-v0.9.38`.
- Runtime benchmarks: local builds for benchmarked pallets succeed (enabled `frame-benchmarking/std` in `std` features).
- Clippy: workspace is clean (no warnings) with `--features runtime-benchmarks` at this commit.

## Testing Snapshot
Counts below reflect simple grep of `#[test]` and `mod tests` at commit time.
- Good coverage: `bridge-signer`, `data-signer`, `parachain-app`, `substrate-app`, `ton-bridge`, `channel`, `multisig-verifier`, `evm-fungible-app`.
- Improved: `leaf-provider` (pallet) now has baseline tests; `leaf-provider` runtime-api has codec/invariant tests; `liberland-bridge-provider` has refund test; `beefy-light-client` runtime-api/rpc have serialization tests.
- Partial: `leaf-provider` rpc has serialization test; request-routing mock can be added later if needed.
- Targeted unit tests exist in `bridge-common` and `types` modules.

Note: Policy requires at least one unit test per newly added function going forward; legacy gaps remain and are tracked in the roadmap.

## Documentation
- Root README, crate READMEs, and rustdoc include attributes are in place.
- Some pallets have inline module docs for extrinsics/storage; others rely mainly on the README. Expanding rustdoc for extrinsics/storage per pallet is beneficial.

## Known TODOs/FIXMEs (by file)
- `pallets/leaf-provider/rpc/src/lib.rs` — Optional: add full ProvideRuntimeApi mock to exercise request routing.

## Dependencies
- Substrate and ORML crates pinned to `polkadot-v0.9.38`. An upgrade plan to newer Substrate/Polkadot releases should be considered (API churn expected).

## Risks / Gaps
- Missing baseline tests in several crates (see Testing Snapshot).
- Benchmarks missing for multiple pallets despite `runtime-benchmarks` feature being enabled in CI.
- Runtime API and RPC crates rely mostly on integration; lack of unit tests for serialization/argument validation.
- Inconsistent `#![warn(missing_docs)]` usage across pallets; only some enforce it.
- Potential performance issues with very heavy tests (see BEEFY light client note above).

## Overall Health
- Codebase composes and tests across many pallets with good coverage in core areas.
- Documentation and contribution guidelines are now consistent.
- Technical debt is manageable; prioritized remediation is in the roadmap.
