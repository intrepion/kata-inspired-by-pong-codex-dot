# Nannou callback timing adapter

`--benchmark` repeats the shared deterministic scenario without keyboard/focus changes and records shared wall-clock intervals at the end of the view callback's draw-command recording. The current Nannou 0.20 renderer is Bevy/wgpu. Rendering/presentation occurs later in the framework; these samples do not measure displayed FPS or GPU completion.

The existing 120 Hz update limit is unchanged. Fixed-step simulation retains its 100 ms catch-up cap. Sampling starts with the first view callback and reports actual physical client dimensions, rejecting resize during measurement. A completed flag prevents duplicate reports while framework quit is deferred. Replay/capture flags cannot be combined with benchmark mode.

The runner requests three bounded release runs with five seconds warm-up and ten seconds measurement each, at most thirty seconds per process. Timed runs perform no app-owned screenshot readback or per-tick logging. Default pacing and differing callback boundaries prevent performance rankings against other adapters.

`cargo build --release --offline --locked --manifest-path experiments/nannou/Cargo.toml`

`node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter nannou`

## Omarchy ARM64 pilot

Three valid release repeats exited 0 at stable 1474×1864 physical client dimensions.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1139 | 8.467 | 11.552 | 14.373 |
| 2 | 1139 | 8.525 | 11.245 | 13.910 |
| 3 | 1145 | 8.496 | 11.163 | 14.367 |

[Allowlisted evidence](nannou-omarchy-pilot-20261112.json) retains samples and binary SHA-256 without private host/environment data. Source revision identifies the base of this adapter patch. Raw reports remain local. Whole-system idle/power/thermal state were not verified despite a requested worker quiet window.

Two Rust adapter tests, fourteen shared Node runner/sanitizer tests, strict Clippy and offline release build passed.

Same-release scripted replay exited 0 with 13 states, final tick 1440/finished=true, and six app-owned PNG captures. Tick 600 was inspected: paddles, ball, centerline and score were visible at correct aspect ratio within a portrait window. This is not real keyboard/focus testing or full visual acceptance.
