# Shared benchmark pilot

`pong-benchmark` measures wall-clock intervals between actual adapter frame callbacks.
It does **not** count fixed simulation steps as rendered frames, measure GPU execution,
or prove frames reached the display. The first adapter is native Macroquad.

## Reproduce

Build once, outside the timed run:

```sh
cargo test -p pong-benchmark -p pong-core -p pong-scenario
cargo build --release -p pong-macroquad
node scripts/benchmarks/run.mjs ./target/release/pong-macroquad NEW-results.json 'actual graphics backend' 'hardware, power/window settings, background activity and limitations' --profile release
```

On Windows use the `.exe` binary path. The shared runner needs Node and uses only
built-ins. Desktop display access is required; inability to open a display is a
failed run, not a zero-performance result. Do not change OS security settings to
run this. Do not pass scripted-test/capture flags with `--benchmark`.

The runner rejects multiple sample or backend markers in one process output as ambiguous. [Windows regression evidence](DUPLICATE-MARKER-GUARD.md).

## Protocol v1

- Three fresh process runs, each bounded externally at 30 seconds.
- Five seconds warmup, then nominal ten-second sample window.
- Existing `smoke-v1` action sequence loops, using a fixed 120Hz simulation clock
  and seed. This is a repeatable mixed playing/paused workload, not a saturated
  rendering throughput test. Accumulator caps remain 100ms per frame.
- Actual app-frame completion/yield boundaries supply `Instant` timestamps.
  Interval includes app CPU work, scheduling and any swap/vsync wait. Screenshots,
  per-tick telemetry and file I/O are absent from measurement. Samples are buffered
  in RAM and output once at exit; sampler overhead is included.
- Requested window is 1000×680, high-DPI enabled. Actual dimensions are recorded;
  any resize during measurement marks the run invalid. Match actual dimensions across comparisons.
- Nearest-rank p50/p95/p99 and raw intervals retained. No FPS or GPU-time claim.
- Binary SHA-256, checkout revision and compiler observed at run time, caller-supplied profile, machine/OS, backend description, load averages and failures are saved. The checkout revision and compiler do not establish how the binary was built. Profile defaults to `unverified`; use `--profile release` or `--profile debug` only when independently known. The runner does not verify the supplied profile. Caller must accurately supply backend/driver and environmental notes; unknowns remain unknown.
- A quiet period from other test/build workers is required for comparison. Load
  averages alone do not prove idleness. Three runs establish harness repeatability,
  not statistical confidence or a cross-engine ranking.

CPU utilization, memory, startup latency, power, thermals and true presentation
metrics remain **unmeasured**. No package/install result is implied. Cross-machine
numbers confound hardware, OS and drivers. This tiny Pong workload cannot predict
performance of a large game. Next step: repeat with verified driver/display/power
metadata, then add a second adapter through this same sampler and protocol.

## Omarchy pilot, 2026-11-12

Source `1bcc704` (based on checkpoint `71c513a`), release Rust 1.99.1,
Macroquad 0.4.16 / Miniquad 0.4.11, Apple M1 Pro 14-inch MacBook Pro,
10 CPU cores, approximately 31.1 GiB RAM, aarch64 Linux 7.2.12-3-1-ARCH.
OpenGL/X11 default path; GPU driver/version, power mode and thermal state were not
queried. The compositor tiled the app to **1474×1864 Macroquad screen units**, not
the requested 1000×680. This size was stable during all measured windows. Future
comparisons must match this or rerun at a common controlled resolution.

| Repeat | Interval samples | Mean ms | p50 ms | p95 ms | p99 ms | Maximum ms |
|---|---:|---:|---:|---:|---:|---:|
| 1 | 1200 | 8.333 | 8.332 | 10.155 | 12.045 | 17.377 |
| 2 | 1199 | 8.335 | 8.329 | 10.743 | 12.632 | 16.652 |
| 3 | 1199 | 8.345 | 8.330 | 10.335 | 13.230 | 26.984 |

[Raw successful runs](macroquad-omarchy-pilot-final-20261112.json) retain timestamps,
binary hash, exact compiler, samples and limits. All three processes exited 0 and
passed the sampling validity checks. Median callback cadence was about 8.33ms,
but **this is not a claim of 120 displayed FPS**. Vsync/scheduling may dominate.
The regression worker confirmed it was idle; global background load remained
about 2.7–2.9 and process visibility was restricted, so full machine idleness is
not established. Treat this as a functioning measurement pilot, not a controlled
cross-framework or cross-OS ranking.

The [initial sandbox attempt](macroquad-omarchy-pilot-20261112.json) failed to open
X11. The supported native-display permission path allowed execution without any
settings changes or desktop input/capture. A [first display-enabled attempt](macroquad-omarchy-pilot-display-20261112.json)
was rejected by the sampler because it incorrectly counted startup compositor
resizing during warmup. The sampler now records settled warmup dimensions and
rejects changes during measurement; failed data is preserved, not averaged in.

Verification: three sampler unit tests, nine core tests and six scenario tests
passed; Macroquad release build and Node syntax check passed. This does not
replace visual gameplay testing, packaging or installation tests.

## Prepare a public measurement copy

The runner's raw local output can contain host identity, filesystem paths, environment details and process diagnostics. Keep that original local. Use the explicit allowlist sanitizer before committing evidence:

```
node scripts/benchmarks/public-report.mjs PRIVATE_INPUT.json NEW_PUBLIC_OUTPUT.json
node --test scripts/benchmarks/public-report.test.mjs
```

This helper retains source/binary hashes, adapter/build profile, validity/exit status and exact timing values. It omits host/environment/toolchain inventory, free-form notes, backend descriptions and process diagnostics, including unknown future fields. Document non-sensitive methodology/backend limitations separately. It does not make an unapproved publication authorized or establish comparable hardware conditions.

Three tests pass for nested privacy filtering, failed-run preservation and malformed-data rejection. The actual raylib three-repeat report was converted without altering any timing sample; a second write to the same destination was rejected and left its bytes unchanged. The original raw report was untouched. No historical evidence is rewritten automatically.
