# AGENTS: sora2-common

This document is a concise guide for contributors and AI coding agents working on this Rust workspace. It explains the layout, required workflows, and the minimum quality bar for changes.

## Overview
- Monorepo of Substrate pallets and companion crates used across SORA’s bridge/common components.
- Rust workspace defined in `Cargo.toml` with multiple crates under `pallets/`.
- CI runs formatting and linting, and expects tests to pass with feature `runtime-benchmarks` enabled.

## Workspace Layout
Key crates (directories under `pallets/`):
- `beefy-light-client` (+ `runtime-api`, `rpc`)
- `bridge-common`
- `bridge-signer`
- `channel` (crate name: bridge-channel)
- `data-signer`
- `dispatch`
- `evm-fungible-app`
- `jetton-app`
- `leaf-provider` (+ `runtime-api`, `rpc`)
- `liberland-bridge-provider`
- `multisig-verifier`
- `parachain-app`
- `substrate-app`
- `substrate-channel`
- `ton-bridge`
- `types` (shared types and traits)

When adding/removing crates, update this list and the workspace members in the root `Cargo.toml`.

## Non‑Negotiable Rules
- Tests per function: Every time you add a new function (public or private), add at least one unit test that exercises it. Changes that add functions without tests are not acceptable.
- Documentation in sync: Keep Rustdoc comments, crate/module docs, and READMEs/AGENTS docs in sync with code behavior and APIs in the same pull request.

## Development Flow
1. Understand the crate: skim its `Cargo.toml`, `src/lib.rs`, and any existing tests.
2. Implement the change (keep scope tight and feature flags consistent).
3. Add tests for each new/changed function.
4. Update docs (Rustdoc, crate README if present, and lists here if structure changed).
5. Run checks locally (format, clippy, and tests).
6. Ensure CI parity: run with the same feature set used by CI.

## Commands
- Format: `cargo fmt --all`
- Lint: `cargo clippy --all-targets --features runtime-benchmarks -- -D warnings`
- Test (workspace): `cargo test --all --features runtime-benchmarks`
- Test (single crate): `cargo test -p <crate_name> --features runtime-benchmarks`
- Jenkins helper: `housekeeping/tests.sh` (runs tests in release with `runtime-benchmarks`).

## Feature Flags
- Most crates default to `std`; CI and local checks use `runtime-benchmarks` for broader coverage.
- Some pallets also define `try-runtime` and other optional features—enable as needed for local verification, but ensure tests pass with CI’s features.

## Pallet Testing Tips
- Place unit tests in a `#[cfg(test)] mod tests` alongside the code, or in `tests/` when an integration harness is needed.
- Substrate pallets typically use `sp_io::TestExternalities` and a mock runtime to exercise extrinsics and storage.
- Keep tests deterministic and avoid relying on global state.

## Style & Quality
- Keep PRs small and focused; prefer clear names and minimal public API surface.
- Do not introduce warnings: clippy runs with `-D warnings`.
- Avoid gratuitous dependencies; prefer workspace crates and Substrate primitives.

## Releases
- Follow the release process in `README.md` (GitHub Flow with `develop` as default, semver tags). Bump versions where needed and keep changelogs/docs updated.

## Change Checklist
- [ ] Added at least one unit test per new function
- [ ] Updated Rustdoc and READMEs/AGENTS as needed
- [ ] Ran `cargo fmt`, `cargo clippy` (with `runtime-benchmarks`), and `cargo test`
- [ ] Considered feature gates and no_std where applicable

## Status & Roadmap Hygiene
- Update `STATUS.md` and `ROADMAP.md` whenever code changes alter scope, testing, docs, or TODOs. Keep them in lockstep with the repository state.

If anything here becomes outdated during your change, update this file in the same PR.
