# SDL3 benchmark adapter

Based on `4088089`. This batch implements and builds the adapter; measurement
requires the coordinator's quiet window. No performance ranking is implied.

`--benchmark` reuses the shared `Frames` sampler and manifest-driven `Scenario`.
The game loops with the shared seed, capped 100ms accumulator and fixed simulation
step. No timeline/physics copy. Replay/capture flags are mutually exclusive with
benchmark mode. Normal play and existing scripted capture paths remain unchanged.

Samples are taken after `canvas.present()` returns. The existing 8ms sleep remains,
so intervals include sleep, scheduling, CPU rendering/submission and any renderer
wait. Vsync is not explicitly configured. This is not GPU time or displayed FPS.
The existing requested size is 900×500; actual dimensions and resize invalidation
come from the shared sampler. Compare only controlled matching dimensions/pacing.

Before warmup, the app prints `PONG_BENCHMARK_BACKEND` using SDL's public
`video.current_video_driver()` and `canvas.renderer_name`. The runner preserves
this per-run runtime metadata separately from caller-provided backend notes.
Missing metadata remains null for older adapters; no backend is guessed.

Quit/escape can stop the app early. Without a completed sample the runner marks
that run invalid, even with exit zero. The runner bounds every process at 30s.
No app framebuffer reads, screenshot exports or per-tick output occur during timing.

```
cargo build --release --locked --manifest-path experiments/sdl3/Cargo.toml
node scripts/benchmarks/run.mjs <release-pong-sdl3> NEW.json 'see per-run reported_backend' 'quiet-window conditions' --adapter sdl3
```

Build/test evidence is recorded separately. A compiled release is not a measured
run, interactive test or tested package. No OS control/capture or settings changes.

## Coordinator pilot completed

Three bounded release processes exited 0 and passed the shared validity checks. Actual dimensions 737×932 stayed stable. Runtime SDL reports {"video_driver":"wayland","renderer":"opengl"}. Five runner tests were rerun successfully after integration atop c2dacef.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
|---|---:|---:|---:|---:|
| 1 | 1167 | 8.374 | 9.604 | 10.847 |
| 2 | 1170 | 8.379 | 9.417 | 10.649 |
| 3 | 1187 | 8.302 | 9.042 | 10.045 |

[Redacted measurement evidence](sdl3-omarchy-pilot-20261112.json) retains binary hash, runtime backend, source base and all samples. Machine identity, environment details and process diagnostics are omitted from this publication copy; original evidence stays local. This checkpoint contains the measured patch. Other workers were asked to remain quiet; full machine idleness/power/thermal state remain unverified. The retained 8ms sleep and different callback boundaries preclude engine-speed ranking. No GPU timing, displayed FPS, new visual/input or packaging result is implied. All three processes exited.

SDL output size was 737×932, unlike the earlier 1474×1864 samples; resolution is another explicit reason these pilots are not a controlled comparison.
