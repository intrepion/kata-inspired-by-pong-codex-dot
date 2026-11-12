# Miniquad callback benchmark adapter

`--benchmark` repeats the shared seeded scenario and records shared wall-clock frame intervals after `commit_frame()` in the draw callback. The existing rendering buffers, draw submission and Miniquad configuration remain unchanged. This callback boundary is not proof of completed GPU work or presentation to the display.

Five seconds warm-up, ten seconds measured, dimensions/resize checks and30-second per-process timeout are shared with other adapters. Actual `window::screen_size()` dimensions are recorded. The app reports Miniquad's actual backend enum before timing; driver identity and actual vsync/refresh configuration are unqueried. No pacing change is made.

Benchmark mode excludes replay/capture flags and ignores normal gameplay controls/minimize pausing; window-close can end early. There is no buffer export or per-tick telemetry during timing. Simulation catch-up remains capped at100ms and uses the existing120Hz step.

```
cargo build --release --offline --locked --manifest-path experiments/miniquad/Cargo.toml
node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter miniquad
```

Three Rust tests (geometry, opt-in, mode separation), release build and strict Clippy passed. Eleven shared Node runner/sanitizer tests passed. These are limited correctness checks, not manual gameplay or package-install qualification.

## Rejected first attempt and automatic completion fix

The first run produced a sample but was **invalid**: `request_quit()` did not immediately end the Wayland run, so the completed sample was printed repeatedly until the bounded runner hit `ENOBUFS`. Its [redacted rejected evidence](miniquad-omarchy-rejected-20261112.json) remains marked invalid; it is not silently dropped or treated as a successful benchmark.

Automated completion now takes the sampler before printing (one report only) and calls Miniquad's supported `order_quit()` rather than its cancellable request path. The same direct completion is used for finished scripted replay; normal user-triggered quit behavior remains unchanged. No OS kill/control workaround is part of the application. Rust tests, release build and strict Clippy were rerun after the fix.

## Post-fix Linux ARM64 pilot

Three bounded release runs exited0 and were valid; stable 1474×1864 output dimensions. Runtime backend enum: OpenGl.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1200 | 8.332 | 9.284 | 10.656 |
| 2 | 1199 | 8.329 | 9.597 | 11.146 |
| 3 | 1199 | 8.332 | 9.148 | 11.398 |

[Allowlisted public report](miniquad-omarchy-pilot-20261112.json) preserves timing samples/provenance without private host/environment/process details. Worker quiet window was requested; whole-system idle/power/thermal state remain unverified. Default pacing and different callback boundaries prevent an engine-throughput ranking.

Post-fix scripted replay also exited0 with13 states and final tick1440. No normal-keyboard or visual-quality pass is implied.
