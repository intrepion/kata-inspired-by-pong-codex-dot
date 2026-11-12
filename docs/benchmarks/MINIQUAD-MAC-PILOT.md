# Miniquad Mac callback-timing pilot

2026-11-12 UTC. Offline locked release build from `64aab08` passed; 3 adapter tests and 11 shared runner/sanitizer tests passed. Three valid repeats each used 5 seconds warm-up and 10 seconds measured. [Sanitized timing data](miniquad-mac-pilot-20261112.json) preserves binary hash, exact intervals and dimensions; private host/environment diagnostics remain local.

| Run | Intervals | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 600 | 16.656 | 17.629 | 17.760 |
| 2 | 600 | 16.657 | 17.622 | 17.868 |
| 3 | 600 | 16.664 | 17.594 | 17.800 |

Reported surface 900×500; no resize, all exits 0. Actual Miniquad runtime enum was OpenGl for all three runs; GPU/driver identity and actual vsync configuration were not queried. Existing Miniquad pacing retained; measurement follows commit_frame in the draw callback. Builds/tests completed first and a worker quiet window was reserved. Other user applications remained open; power, thermals and whole-machine idleness remain unverified.

These are **callback intervals**, not displayed FPS or GPU execution time. Differing pacing, dimensions and machines prevent simple engine rankings. No new manual-input or installer qualification is claimed. All benchmark processes exited. The shared emit-once/order_quit completion fix also terminated these Mac runs normally.
