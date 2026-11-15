# Softbuffer benchmark adapter and bounded pilot

Source based on `5121b5b`, Omarchy ARM64, 2026-11-12.
Initial adapter checks did not launch the app. The coordinator subsequently built release and completed three bounded native-display runs; results below are not a ranking.

`experiments/softbuffer` now supports `--benchmark`, mutually exclusive with
scripted replay/capture flags. It reuses `pong-benchmark::Frames` and the shared
manifest-driven `pong-scenario::Scenario`; no new action timeline or physics.
Normal gameplay and `--scripted-test` paths are preserved.

During the opt-in benchmark, normal keyboard/focus gameplay changes are ignored;
closing the window still exits. The scenario loops at its existing end with a
fresh seeded game. Existing CPU rasterization and buffer presentation run normally.
The sampler records after each nonzero-size frame's present submission, with five
seconds warmup and nominal ten seconds measurement. It never captures images.

This is a **callback interval**, not GPU execution, displayed FPS or throughput.
Softbuffer retains its existing winit 8ms `WaitUntil` pacing. Its requested window
is 900×500, unlike Macroquad's 1000×680 request; actual compositor size is recorded.
Match actual dimensions and record pacing differences before interpreting any
comparison. A zero-size/minimized surface does not produce samples; the runner's
external timeout handles lack of progress. No OS configuration was changed.

## Build outside the quiet window, then coordinate before running

```
cargo build --release --locked --manifest-path experiments/softbuffer/Cargo.toml
node scripts/benchmarks/run.mjs <release-pong-softbuffer> NEW.json 'actual backend' 'conditions and limitations' --adapter softbuffer
```

The runner now selects explicit adapter/presentation/renderer/pacing metadata.
Existing pilot command lines still default to Macroquad. Unknown adapters and
malformed arguments fail before launch; existing output is never overwritten.
The caller remains responsible for supplying a release binary and truthful backend
and environmental notes. The runner's profile field is not binary introspection.

## Checks completed

- Locked/offline debug build and strict all-target Clippy: pass.
- Two Softbuffer mode-selection tests: pass.
- Eighteen shared sampler/core/scenario tests: pass.
- Four Node metadata/runner tests: pass. The runner test launches only a tiny
  synthetic fixture process, not Pong, and discards its fixture output. It checks
  three-run metadata and overwrite refusal; it is not performance evidence.
- Lockfile change adds only the local sampler crate; no registry version changes.

No new native rendering, real-input QA, release timing or packaging pass is implied.

## Coordinator pilot

Release locked/offline build and four Node tests passed. Three fresh processes exited 0 with valid 5-second warmup/10-second sample windows. No capture or OS-input automation was used. Actual size was 1474×1864 physical window pixels throughout each measured window.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
|---|---:|---:|---:|---:|
| 1 | 896 | 10.930 | 13.582 | 15.011 |
| 2 | 888 | 10.959 | 13.976 | 15.243 |
| 3 | 897 | 10.910 | 13.883 | 15.288 |

[Raw results](softbuffer-omarchy-pilot-20261112.json) preserve all intervals, binary hash, base revision and dirty source list. This checkpoint contains the measured source changes. Workers were asked to remain quiet, but whole-machine idleness, driver/power/thermal state are unverified. Retained 8ms pacing and different rendering/callback boundaries prevent an engine-speed ranking against Macroquad. Callback intervals are not displayed FPS. All test processes exited.
