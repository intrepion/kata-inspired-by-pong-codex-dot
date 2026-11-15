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

## Wall-clock fix requalification on Omarchy

At source 27760d3 (includes fixed clock8439f30), all seven Piston tests and release build passed. Plain replay exited0 in12.092 seconds; capture replay exited0 in12.128 seconds, each with13 states and final tick1440/finished=true. Six PNGs were saved; tick600 was inspected for paddles, ball, centerline and score marker. No new real-input/focus or comprehensive visual-quality pass is claimed.

Three post-fix callback runs were valid and exited0 at stable1474×1864. Original pre-fix results above remain preserved rather than overwritten.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 594 | 16.718 | 18.076 | 20.160 |
| 2 | 595 | 16.714 | 17.936 | 18.766 |
| 3 | 597 | 16.706 | 17.630 | 18.319 |

[Post-fix allowlisted evidence](piston-omarchy-postfix-20261112.json). Same callback boundary and default pacing; no engine ranking or displayed-FPS claim.
