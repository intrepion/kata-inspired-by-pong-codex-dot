# ggez Mac callback-timing pilot

2026-11-12 UTC. Offline locked release build from `aa7354d` passed in 58 seconds; 2 adapter tests and 13 shared runner/sanitizer tests passed. Harness revision `0a87203` adds only README activity. Three valid repeats each used 5 seconds warm-up and 10 seconds measured. [Sanitized timing data](ggez-mac-pilot-20261112.json) preserves binary hash, exact intervals and dimensions; private host/environment diagnostics remain local.

| Run | Intervals | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 600 | 16.711 | 17.728 | 18.088 |
| 2 | 600 | 16.715 | 17.698 | 18.119 |
| 3 | 600 | 16.712 | 17.749 | 18.035 |

Reported drawable surface 1000×680; no resize, all exits 0. ggez uses wgpu; actual runtime backend/driver were not queried. Existing 120 Hz remainder sleep remains unchanged. Measurement follows Canvas::finish before framework end_frame, not GPU completion or final presentation. Builds/tests completed first and a worker quiet window was reserved. Other user applications remained open; power, thermals and whole-machine idleness remain unverified.

These are **callback intervals**, not displayed FPS or GPU execution time. Different pacing, dimensions and callback boundaries prevent an engine ranking. No new manual-input or installer qualification is claimed. All benchmark processes exited.

The same release binary completed a separate scripted replay/capture run in 12.320 seconds: exit 0, 13 records, final tick 1440, six PNGs. The tick-600 app-owned frame was inspected: both paddles, ball, court, score and instructions were legible with no obvious rendering failure. This is a bounded frame sample, not exhaustive visual or native-input acceptance. Raw replay/capture artifacts remain local.
