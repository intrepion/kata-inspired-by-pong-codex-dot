# Portable scenario snapshot comparison

## Diagnosis (2026-11-12, Windows GNU)

The original test compared JSONL bytes against an Omarchy capture. The Windows run had identical checkpoint count, tick/state/score/seed/finish fields but tiny numeric differences after paddle bounces. The core uses f32 `hypot`, `sin` and `cos`; cross-platform bit-identical transcendental results are not a documented contract. This is not proof of deterministic lockstep support.

## Test contract

The reference capture and game physics are unchanged. The portable test now parses a strict schema (no missing or extra fields; fixed array lengths), checks all discrete values exactly and applies an absolute **0.001** bound only to the six position/velocity values. This is one thousandth of a world unit (or unit/second), not a relative tolerance that grows with coordinate size. It is a small explicit portability budget, not an inferred universal platform maximum. Greater drift, scoring/state changes or changed checkpoint counts still fail. Non-finite values fail.

Negative tests exercise all six numeric fields, both signs of material error, NaN/infinity, all discrete fields, missing/extra keys and malformed ball arrays. The separate same-runtime repeat test still compares exact reports across different rendering batches. This semantic cross-platform comparison does not certify bitwise reproducibility, visual correctness, input integration or longer simulations.

## Verified evidence

Windows GNU Rust 1.99.0: three portable tests passed. Maximum absolute difference over the unchanged 13 captured snapshots and 78 float values was **0.000020000000006348273**. The allowance is fixed rather than recalibrated on each run. Full core/scenario/benchmark tests were rerun after the change. Linux/Mac validation of this updated test is delegated separately; do not infer those results from Windows.

### Linux ARM64 and macOS ARM64 requalification

At exact source `42f1135`, both Omarchy ARM64 and Mac ARM64 passed all **20 Rust tests** (core9, sampler3, portable3, replay5), plus **5 Node benchmark-adapter/runner tests**. The three portable tests on each reported maximum absolute baseline delta **0**, within the unchanged 0.001 budget. Commands used locked offline Cargo dependencies; no simulation or baseline edits were made during requalification.

```
cargo test --offline --locked -p pong-core -p pong-scenario -p pong-benchmark
cargo test --offline --locked -p pong-scenario --test portable -- --nocapture
node --test scripts/benchmarks/adapters.test.mjs
```

This closes the delegated Linux/Mac checks for this test revision only. It does not turn semantic agreement into cross-platform bitwise determinism or add native gameplay, package-launch or performance evidence.
