# Fyrox Mac pre-render callback pilot

2026-11-12 UTC. Offline locked release build from `17bedab` passed in 5m27s; 2 adapter tests and 19 shared runner/sanitizer tests passed. Harness snapshot `c16c59c` changes only README activity. [Sanitized evidence](fyrox-mac-pilot-20261112.json) identifies binary hash and exact intervals.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1104 | 8.584 | 16.694 | 16.870 |
| 2 | 1107 | 8.472 | 16.718 | 16.815 |
| 3 | 1139 | 8.364 | 16.674 | 16.827 |

Three valid native repeats with stable 1600×1200 dimensions, exit 0, five seconds warm-up and ten seconds measurement. Existing desired update rate 120 Hz and renderer defaults retained. Sampling is in before_rendering immediately before engine.render, **not GPU completion or displayed FPS**. Runtime driver unqueried. Differing callback boundaries/pacing/dimensions prevent rankings.

Build/tests completed before a worker quiet window; other user apps remained open, power/thermal/whole-machine idle unverified. Private diagnostics remain local. All benchmark processes exited.

Separate scripted replay/capture completed in 12.148 seconds, exit 0, 13 states, tick 1440 and six PNGs. Tick 600 was inspected: court, paddles and ball visible; score/instruction text is small relative to the 1600×1200 capture and remains a UI-polish concern. This bounded app-owned UI render-target sample is not full visual QA, native-input or package acceptance.
