# ggez callback benchmark adapter

Opt-in `--benchmark` repeats the shared seeded scenario and samples shared wall-clock intervals after `Canvas::finish()` in the draw callback. This boundary precedes ggez's framework `end_frame`; it is not proof of completed GPU work or presented frames. The pinned ggez uses wgpu; runtime adapter/driver identity is not queried.

The existing 120 Hz remainder sleep and 120 Hz fixed simulation step, capped at 100 ms catch-up, are preserved. Benchmark mode ignores normal gameplay controls/focus pausing and rejects replay/capture flags. The completed sampler is removed before requesting framework quit so it prints once. Normal gameplay and capture paths retain their existing behavior.

The common protocol uses five seconds warm-up and ten seconds measured with a 30-second per-process limit, actual drawable dimensions and resize invalidation. No readback or per-tick telemetry occurs during the timed runs. Measurements include normal rendering and scheduling effects, not isolated engine throughput.

`cargo build --release --offline --locked --manifest-path experiments/ggez/Cargo.toml`

`node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter ggez`

## Omarchy ARM64 pilot

Three valid release repeats exited 0 at stable 1474×1864 drawable dimensions.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1176 | 8.408 | 9.197 | 9.951 |
| 2 | 1181 | 8.412 | 8.937 | 9.526 |
| 3 | 1181 | 8.408 | 8.937 | 9.533 |

[Allowlisted public evidence](ggez-omarchy-pilot-20261112.json) preserves samples and executable hash. Source revision is the base of this adapter patch. Full private reports remain local. A quiet window was requested; whole-system idle, power and thermal state are unverified.

Two Rust adapter tests, thirteen shared Node runner/sanitizer tests, strict Clippy and release build passed. The same executable completed scripted replay with 13 states, tick 1440, finished=true, and six app-owned PNGs. Tick 600 was visually inspected: visible paddles, ball, centerline, score and controls; large portrait-window letterboxing retains the court aspect ratio. This is not a real keyboard/focus or full visual acceptance pass.
