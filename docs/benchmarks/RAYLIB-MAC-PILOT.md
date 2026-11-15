# raylib-rs macOS callback-timing pilot

2026-11-12 UTC. Offline locked release build at `5482db1` succeeded;3 adapter tests and6 shared-runner tests passed. The runner at `6ef14d2` (only sanitizer additions since the build) completed3 valid repeats of5seconds warm-up and10seconds measured. [Sanitized raw timing](raylib-mac-pilot-20261112.json) retains binary hash, exact intervals and run results. All children exited0; no resize.

| Run | Intervals | Median ms | p95 ms | p99 ms |
|---|---:|---:|---:|---:|
|1|1097|9.211|9.366|9.418|
|2|1098|9.197|9.385|9.439|
|3|1097|9.211|9.384|9.431|

raylib-rs GPU; runtime driver not queried. Existing target120fps pacing retained; callback sampled after EndDrawing including its pacing wait. Builds/tests finished before timing; other user apps remained open; power, thermals and full-machine idleness unverified. These are **callback intervals, not displayed FPS, GPU time or an engine ranking**. Different pacing/resolutions/hardware prevent simplistic cross-adapter rankings. No manual-gameplay or installation claim added by this benchmark.
