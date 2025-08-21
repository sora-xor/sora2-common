# Roadmap — sora2-common

This roadmap lists prioritized tasks to close known gaps, finish unfinished work, and reduce technical debt. Update alongside code changes.

## Priorities
- P0: Critical path for build health, test gaps, and correctness.
- P1: Important improvements for maintainability and reliability.
- P2: Nice-to-have enhancements and forward-looking work.

## P0 — Immediate
- Tests: Add baseline unit tests where missing
  - leaf-provider (pallet): add tests for `Hooks::on_initialize`, `AuxiliaryDigestHandler::add_item`, and `BeefyDataProvider::extra_data` happy paths and edge cases.
  - leaf-provider-runtime-api: add unit tests for API type invariants and simple codec round-trips.
  - leaf-provider-rpc: add tests for request routing (mock runtime API) and result serialization.
  - liberland-bridge-provider: add at least one unit test per public helper.
  - beefy-light-client runtime-api/rpc: add minimal unit tests covering argument validation and serialization.
- Benchmarks: Implement missing benches
  - parachain-app: add `frame-benchmarking` benches for key extrinsics (e.g., burn/mint equivalents) replacing TODO markers.
- TODO cleanup
  - substrate-channel/outbound: replace “TODO: Select interval” with a `#[pallet::constant]` config param (e.g., `MessageInterval`) and use it in scheduling.
  - beefy-light-client: evaluate and re-enable the heavy test by optimizing fixture generation or gating under `expensive_tests`.

## P1 — Near Term
- Docs: Expand rustdoc for pallets
  - For each pallet, document extrinsics and storage items explicitly (purpose, parameters, errors, events, and invariants).
  - Enable `#![warn(missing_docs)]` on more crates after filling gaps.
- CI/DevX: Reinforce policies
  - Add a PR template checklist to require: at least one test per new function, docs updated, STATUS/ROADMAP updated.
  - Optionally add a job that builds docs (`cargo doc -Z rustdoc-map` if available) to catch doc errors.
- Tests: Add serialization/failure-path tests
  - RPC/runtime-API crates: cover invalid inputs, boundary conditions, and codec round-trips.

## P2 — Medium Term
- Dependency strategy
  - Plan upgrade path from `polkadot-v0.9.38` to a newer Substrate release (API audit, feature parity, migration plan).
- Fuzzing & property tests
  - Add proptests/fuzz targets for critical codecs and verifiers (e.g., simplified proof verification, bitfield operations).
- Performance
  - Audit heavy tests for runtime and memory; add feature flags or fixture sharding to keep CI fast.

## Task Prompts (ready-to-use)
- Implement benches in parachain-app
  - File: `pallets/parachain-app/src/lib.rs`
  - Action: Add `#[cfg(feature = "runtime-benchmarks")] mod benchmarking;` module with benches for each public extrinsic. Verify `weights` generation integrates.
  - Tests: Add at least one unit test per helper function introduced by benches (if any) and ensure `cargo test --features runtime-benchmarks` passes.
- Add interval config to substrate-channel outbound
  - File: `pallets/substrate-channel/src/outbound/mod.rs`
  - Action: Introduce `#[pallet::constant] type MessageInterval: Get<u32>;` and replace the TODO site to use the constant. Add a test that the interval drives expected behavior.
- Add leaf-provider baseline tests
  - File: `pallets/leaf-provider/src/lib.rs`
  - Action: Create `#[cfg(test)] mod tests` with a mock runtime to test digest clearing in `on_initialize`, adding items via `AuxiliaryDigestHandler`, and `extra_data()` output structure.
- Add RPC unit tests for leaf-provider
  - File: `pallets/leaf-provider/rpc/src/lib.rs`
  - Action: Use a mock of `ProvideRuntimeApi` to simulate `latest_digest` responses; assert RPC returns serialized values and error propagation.
- Stabilize beefy-light-client heavy test
  - File: `pallets/beefy-light-client/src/tests.rs`
  - Action: Wrap expensive test with `#[cfg(feature = "expensive_tests")]` or reduce fixture size. Ensure default CI remains fast.

Keep this roadmap current: check off items in PRs and add new ones as you identify gaps.

