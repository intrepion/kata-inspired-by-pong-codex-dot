# pixels/winit macOS callback-timing pilot

2026-11-12 UTC. Offline locked release build from `938abc2` passed;3 adapter tests and10 shared-runner/sanitizer tests passed. Harness snapshot `22431c0` differs only by README activity. Three valid repeats each used5seconds warm-up and10seconds measured. [Sanitized timing data](pixels-mac-pilot-20261112.json) preserves binary hash, exact intervals and dimensions; private host/environment diagnostics remain local.

| Run | Intervals | Median ms | p95 ms | p99 ms |
|---|---:|---:|---:|---:|
|1|600|16.724|17.435|17.724|
|2|600|16.713|17.596|18.033|
|3|600|16.708|17.573|18.103|

Reported surface1800×1000; no resize, all exits0. CPU raster plus pixels/wgpu presentation, runtime GPU/backend/driver not queried. Existing winit Poll retained; measurement follows successful `pixels.render` return. Builds/tests completed first; other user applications remained open, power/thermals/full-machine idleness unverified.

These are **callback intervals**, not displayed FPS or GPU execution time. Differing pacing, dimensions and machines prevent simple engine rankings. No new manual-input or installer qualification is claimed. All benchmark processes exited.
