# bracket-lib graphical tile callback timing

`--benchmark` repeats the shared deterministic scenario and samples at the end of the graphical CP437 tile-recording tick, before bracket-lib rendering. This is not a terminal application. Existing OpenGL backend selection, scoped process-local XWayland fallback, fixed tile grid and 60 fps cap remain unchanged. Actual runtime driver and presentation timing are unqueried.

The shared sampler measures five seconds warm-up and ten seconds intervals, records BTerm's pixel dimensions and rejects measured resizes. Gameplay keyboard/focus/resizing events do not alter benchmark simulation; close still exits. Benchmark mode rejects replay/capture flags and saves no screenshots or per-tick telemetry. Shared simulation uses capped100ms catch-up with120Hz fixed steps; the final report is emitted once before framework quit.

Three release repeats are bounded at30seconds per process. These tile-recording callback intervals are not GPU time/displayed FPS and do not support rankings against adapters with different rendering/pacing.

`cargo build --release --offline --locked --manifest-path experiments/bracket-lib/Cargo.toml`

`node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter bracket-lib`

## Omarchy ARM64 pilot

Three valid release repeats exited0 at stable 1920×1280 pixel dimensions.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 624 | 16.003 | 16.659 | 17.288 |
| 2 | 624 | 15.997 | 16.698 | 17.416 |
| 3 | 624 | 16.002 | 16.632 | 17.858 |

[Allowlisted evidence](bracket-lib-omarchy-pilot-20261112.json) retains samples and binary hash; source revision identifies this patch base. Raw reports remain local. Worker quiet window requested; whole-machine idle, power and thermal state are unverified.

Eight Rust adapter tests, sixteen shared Node tests, strict application Clippy and offline release build passed. Existing vendored FreeType deprecation/lifetime/static-mut warnings remain; this is not dependency-warning clearance.

The same release binary completed scripted replay with13 states, tick1440/finished=true and six framework screenshot PNGs, exit0. Tick600 inspected: readable header/controls, paddles, quantized ball and centerline. This remains a tile-quantized presentation, not smooth-vector geometry; no real-input/focus or full visual acceptance claim.
