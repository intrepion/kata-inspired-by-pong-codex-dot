# bracket-lib Mac callback-timing pilot

2026-11-12 UTC. Offline locked release build from `cfc6e3a` passed in 19.28 seconds; 8 adapter tests and 16 shared runner/sanitizer tests passed. Cargo warned that upstream block 0.1.6 has future Rust incompatibilities; this build still succeeded. [Sanitized evidence](bracket-lib-mac-pilot-20261112.json) retains binary hash and exact timing samples; private diagnostics remain local.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 625 | 15.999 | 16.134 | 16.155 |
| 2 | 625 | 15.999 | 16.135 | 16.176 |
| 3 | 625 | 15.999 | 16.143 | 16.212 |

Three valid repeats, stable 1456×976 physical window dimensions, all exits 0. Existing 60fps cap retained. Samples are the graphical CP437 tile tick callback before framework rendering, not terminal output or displayed FPS. Actual driver and display timing were unqueried. Different pacing, callback boundaries and dimensions prevent an engine ranking.

Each repeat used five seconds warm-up and ten seconds measured. Builds/tests completed first and a worker quiet window was reserved; user apps remained open, power/thermal/whole-machine idle unverified. No new manual-input or installer qualification is claimed; all benchmark processes exited.

Separate replay/capture completed in 12.279 seconds, exit 0, 13 states, tick 1440 and six PNGs. Tick 600 was inspected: score/instructions were readable and paddles/ball visible; the dark centre divider remains low contrast. This is bounded visual evidence, not full visual acceptance. Raw screenshots/log remain local.
