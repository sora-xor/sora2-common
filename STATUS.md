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

## Testing Snapshot
Counts below reflect simple grep of `#[test]` and `mod tests` at commit time.
- Good coverage: `bridge-signer`, `data-signer`, `parachain-app`, `substrate-app`, `ton-bridge`, `channel`, `multisig-verifier`, `evm-fungible-app`.
- Minimal/none: `leaf-provider` (pallet), `leaf-provider` runtime-api, `leaf-provider` rpc, `liberland-bridge-provider`, `beefy-light-client` runtime-api/rpc.
- Targeted unit tests exist in `bridge-common` and `types` modules.

Note: Policy requires at least one unit test per newly added function going forward; legacy gaps remain and are tracked in the roadmap.

## Documentation
- Root README, crate READMEs, and rustdoc include attributes are in place.
- Some pallets have inline module docs for extrinsics/storage; others rely mainly on the README. Expanding rustdoc for extrinsics/storage per pallet is beneficial.

## Known TODOs/FIXMEs (by file)
- `pallets/substrate-channel/src/outbound/mod.rs:117` — TODO: Select interval (requires a configurable constant or runtime parameter).
- `pallets/beefy-light-client/src/tests.rs:92` — TODO: Heavy test disabled until #372 is done (consider optimization/bench gating).
- `pallets/parachain-app/src/lib.rs:263,317,336,490` — TODO: make benchmarks (add proper `frame-benchmarking` benches for extrinsics).

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

