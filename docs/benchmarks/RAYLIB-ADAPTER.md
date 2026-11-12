# raylib-rs callback benchmark adapter

The `--benchmark` path uses the existing shared `pong-benchmark::Frames` sampler and `pong-scenario::Scenario`; gameplay physics and timeline are not duplicated. Fixed-step catch-up is capped at100ms per callback and the scenario repeats with the shared seed. Benchmark flags cannot be combined with scripted replay/capture flags.

The sample boundary occurs after dropping the draw handle, which invokes `EndDrawing`. It therefore includes raylib's existing120fps pacing wait along with CPU rendering/submission and scheduling. Native rendering is unchanged. User gameplay input and focus-based pausing do not perturb the benchmark simulation; window close/Escape can terminate early, which the runner rejects if no complete sample arrives.

No framebuffer readback, captures or per-tick telemetry occur during measurement. The five-second warm-up and ten-second measurement are shared with the earlier pilots. Requested window size remains900×500; actual raylib screen dimensions are recorded and resize invalidates a measured run. Runtime GPU driver identity is unqueried and is not inferred from the binary name.

```
cargo build --release --offline --locked --manifest-path experiments/raylib-rs/Cargo.toml
node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter raylib
```

Release build, strict Clippy, three Rust adapter/viewport tests and six Node runner/metadata tests pass on Linux ARM64. This is neither displayed FPS nor GPU time, and retained target pacing prevents using it as an engine-throughput ranking.

## Linux ARM64 pilot

Three bounded release runs completed with exit0 and valid reports. All used 1474×1864 raylib-reported screen dimensions without measured resize.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1180 | 8.401 | 8.902 | 9.483 |
| 2 | 1181 | 8.400 | 8.910 | 9.619 |
| 3 | 1178 | 8.401 | 9.015 | 9.451 |

[Redacted measurement evidence](raylib-omarchy-pilot-20261112.json) retains all timing samples and binary hash. Other worker jobs were asked to stay quiet; global idleness, power and thermal state remain unverified. Full original metadata remains local. Different dimensions/backend/pacing prevent a controlled comparison with other pilots.

Post-change app-owned scripted replay also exited0 with13 state records and finished tick1440. This checks the existing replay path, not real keyboard interaction or visual quality.
