# Bevy main-world frame callback timing

`--benchmark` repeats the shared seeded scenario and samples the main world's `Last` schedule. This is a main-application callback interval, **not a rendered-frame, GPU-completion or displayed-FPS measurement**. Bevy's render sub-app runs separately; the measurement does not assert one rendered frame per sample.

Default window/renderer/event-loop configuration remains unchanged. Physical primary-window dimensions are recorded and measured resize invalidates a run. Shared simulation uses120Hz steps with100ms maximum catch-up; keyboard/focus gameplay mutations are skipped in benchmark mode. Replay/capture flags are rejected together with benchmark. Normal gameplay preserves its existing time-based seed. A completion flag permits exactly one report before `AppExit::Success`.

The shared runner requests three bounded release runs:5seconds warm-up and10seconds measurement,30seconds maximum per process. No screenshot readback or per-tick logging occurs during timing. Differing callback boundaries and pacing prohibit ranking these results against other engines.

`cargo build --release --offline --locked -p pong-bevy`

`node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter bevy`

## Omarchy ARM64 pilot

Three valid release repeats exited0 at stable 1474×1864 physical primary-window dimensions.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1199 | 8.248 | 12.686 | 16.070 |
| 2 | 1199 | 8.255 | 13.012 | 16.490 |
| 3 | 1197 | 8.255 | 12.716 | 17.922 |

[Allowlisted report](bevy-omarchy-pilot-20261112.json) preserves timing samples and binary SHA-256; revision identifies the patch base. Raw reports remain local. Worker quiet window requested; whole-system idle, power and thermal conditions unverified.

Two adapter Rust tests, seventeen shared Node runner/sanitizer tests, strict Clippy and offline release build passed.

The same release executable completed scripted replay with13 states, final tick1440/finished=true and six app-owned GPU screenshots, exit0. Tick600 inspected: readable score/controls, paddles, ball and centerline. This separate render evidence does not convert main-world timing into displayed-frame timing. No new real-input/focus or full visual acceptance claim.
