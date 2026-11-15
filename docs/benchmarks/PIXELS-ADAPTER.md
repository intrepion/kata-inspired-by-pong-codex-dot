# pixels/winit benchmark adapter

`--benchmark` uses the shared frame sampler and deterministic scenario; no separate physics/timeline implementation. The existing CPU900×500 RGBA raster is uploaded/presented through pixels/wgpu, with the native window surface dimensions recorded separately. Do not mistake the fixed source-buffer resolution for output resolution.

The sample boundary is after successful `pixels.render()` in the redraw callback. A rendering error exits without manufacturing a valid sample. Existing winit Poll/request-redraw behavior remains; presentation mode/driver are not queried or changed. This captures CPU raster/upload/submission, scheduling and any present blocking, not GPU execution time or displayed FPS.

Benchmark mode ignores normal gameplay input/focus pausing and repeats the shared seeded scenario with capped100ms catch-up. Window close still exits early. It is mutually exclusive with scripted replay/capture flags. No app buffer export or per-tick telemetry occurs during measurement. Window title updates remain part of the existing loop.

```
cargo build --release --offline --locked --manifest-path experiments/pixels/Cargo.toml
node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter pixels
```

Three Rust tests (framebuffer correctness, opt-in mode, mixed-mode rejection), release build and strict Clippy pass. Ten shared Node runner/sanitizer tests pass. These checks alone are not an interactive gameplay or installation qualification.

## Linux ARM64 pilot

Three bounded release runs exited0 and were valid, with stable 1474×1864 output dimensions.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1198 | 8.327 | 9.637 | 11.083 |
| 2 | 1199 | 8.328 | 9.630 | 11.521 |
| 3 | 1198 | 8.326 | 9.562 | 10.719 |

[Allowlisted public evidence](pixels-omarchy-pilot-20261112.json) retains exact intervals and provenance hashes, excluding private host/environment/diagnostics. Original raw report remains local. Other worker jobs were asked to stay quiet; whole-machine idleness/power/thermal conditions are not proven. Different render work, dimensions and presentation behavior preclude an engine-speed ranking.

The post-change existing scripted replay exited0, emitted13 state records and completed tick1440; no new manual-input or visual-quality pass is claimed.
