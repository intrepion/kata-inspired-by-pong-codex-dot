# Softbuffer Windows callback-timing pilot

2026-11-12 08:17 UTC. Fresh locked release build from7994382 completed in20.88s. Two release adapter tests and13 shared runner/sanitizer tests passed. [Sanitized timing evidence](softbuffer-windows-pilot-20261112.json).

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1005 | 9.3143 | 15.7249 | 17.4108 |
| 2 | 998 | 9.3301 | 16.2087 | 17.3083 |
| 3 | 1013 | 9.2760 | 15.5476 | 17.1068 |

All three repeats valid:5s warmup/10s sample, no resize, exit0, actual1125x625 surface. Existing8ms WaitUntil pacing retained; CPU raster followed by softbuffer present submission. This measures callback cadence, not displayed FPS or GPU time. No backend overrides. Power/thermals unknown, other user apps open; builds completed before timing.

Replay/capture regression completed in12.126s, exit0/no stderr,13 ordered state records through1440/finished and6 PNG captures. Tick600 inspected: both paddles, ball, divider and0:1 score rendered. App-owned buffer evidence is not OS keyboard/focus/resize acceptance. All processes exited. No installation/distribution qualification added.
