# Piston Windows callback-timing pilot

2026-11-12 UTC. Fresh locked release build completed in6m33s. Binary built before any changes to the Piston adapter after ebdc9da; runner provenance91b3fd0. Two release adapter tests and12 shared runner/sanitizer tests passed. [Sanitized timing evidence](piston-windows-pilot-20261112.json).

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 591 | 16.8600 | 17.4238 | 17.6637 |
| 2 | 590 | 16.8663 | 17.4737 | 17.7215 |
| 3 | 591 | 16.8533 | 17.4656 | 17.8281 |

Three valid runs,5s warmup/10s measurement, all exits0, no resize, actual1250x850 surface. Existing defaults retained; no backend environment override. GPU/driver not queried. Callback timing is not displayed FPS, GPU time or engine ranking. Builds finished before measurements; other user apps remained open, power/thermals unmeasured.

The same release binary completed scripted replay with six app-owned captures in12.568s: exit0, no stderr,13 state records ending1440/finished. A90s external timeout was selected up front in light of the Mac pacing observation, but Windows completed well within30s. Tick600 inspected: both paddles, ball, score pip and court visible. This is not native keyboard/focus/resize qualification. No assertion that the slower Mac replay is fixed. All benchmark/replay processes exited.
