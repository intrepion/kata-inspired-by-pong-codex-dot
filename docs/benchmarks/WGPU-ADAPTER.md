# Raw wgpu callback timing adapter

`--benchmark` repeats the shared seeded scenario and samples shared wall-clock intervals after a successful surface-present request. The renderer now returns whether it actually submitted a surface frame: lost/outdated/timeout/occluded recovery paths remain unchanged but do not count as successful samples. A successful request is not proof of display presentation or completed GPU work.

The existing winit Poll loop and default surface configuration remain unchanged. Runtime driver and actual display timing are unqueried. Fixed-step simulation remains 120 Hz with 100 ms catch-up cap. Benchmark mode excludes replay/capture flags and ignores gameplay keyboard/focus state. It samples actual configured surface dimensions, with resize invalidation; no screenshot readback/per-tick logging occurs during measurements.

Common protocol: five seconds warm-up, ten seconds measurement, three bounded release repeats with thirty-second per-process timeout. Callback boundaries, hardware and pacing differ across adapters, so this is not an engine-throughput ranking.

`cargo build --release --offline --locked --manifest-path experiments/wgpu/Cargo.toml`

`node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter wgpu`

## Omarchy ARM64 pilot

Three valid release repeats exited 0 at stable 1474×1864 surface dimensions.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 18927 | 0.285 | 1.482 | 2.562 |
| 2 | 18972 | 0.286 | 1.503 | 2.760 |
| 3 | 18172 | 0.292 | 1.596 | 2.617 |

The very short intervals are successful present-request callback intervals, **not thousands of displayed frames per second**. No explicit frame cap was added and presentation completion was not measured. This result cannot be ranked against capped adapters.

[Allowlisted evidence](wgpu-omarchy-pilot-20261112.json) retains timing samples and executable SHA-256; the source revision identifies the base of this patch. Full raw reports stay local. Worker quiet window requested, whole-system idle/power/thermal state unverified.

Two adapter Rust tests, fifteen shared Node tests, strict Clippy and offline release build passed.

The same executable completed scripted replay with 13 states, tick 1440/finished=true, and six app-owned surface screenshots, exit 0. Tick 600 inspected: paddles, ball, centerline and the score marker visible; portrait window letterboxing retains court aspect ratio. This is not a real-input/focus or full visual acceptance pass.
