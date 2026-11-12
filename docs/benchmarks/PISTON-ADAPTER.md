# Piston callback benchmark adapter

Opt-in `--benchmark` repeats the shared seeded scenario and samples shared wall-clock intervals after `draw_2d` on render events only. Update events are not samples. The current pinned piston_window 0.148 uses wgpu, not the historical OpenGL renderer; the runtime GPU/driver is unqueried.

Existing 120 UPS simulation scheduling and default rendering configuration are unchanged. 120 UPS is not a displayed frame-rate claim. The simulation catches up in fixed 120 Hz steps with a 100 ms cap. Sampling uses actual render-event draw dimensions after five seconds warm-up and ten seconds measurement. The runner bounds each process to 30 seconds. Resize invalidates a run. Benchmark and scripted replay/capture modes are mutually exclusive; normal gameplay controls do not perturb the benchmark.

These are draw-submission callback intervals, not measured GPU completion, displayed FPS or input latency. Different callback boundaries, dimensions and default pacing prevent a throughput ranking.

`cargo build --release --offline --locked --manifest-path experiments/piston/Cargo.toml`

`node scripts/benchmarks/run.mjs RELEASE_BINARY NEW.json 'backend notes' 'conditions' --adapter piston`

## Omarchy ARM64 pilot

Three valid release runs exited 0; dimensions stayed 1474×1864. A worker quiet window was requested, but whole-machine idle/power/thermal state was not verified.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 595 | 16.714 | 18.105 | 19.665 |
| 2 | 594 | 16.726 | 18.208 | 21.826 |
| 3 | 595 | 16.704 | 17.847 | 19.206 |

[Allowlisted evidence](piston-omarchy-pilot-20261112.json) retains timing/provenance fields without private host/environment data. The source revision identifies the base; the adapter changes are included in this checkpoint and the measured executable is identified by SHA-256. Full raw evidence is retained locally.

Two Rust adapter tests, twelve shared Node runner/sanitizer tests, strict Clippy and release build passed. The same release binary's scripted replay exited 0 with 13 state records, final tick 1440 and finished=true. This is not a new normal-input, visual-quality or package-install pass.
