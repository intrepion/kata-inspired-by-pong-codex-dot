# Miniquad Windows callback-timing pilot

2026-11-12 08:04 UTC. Fresh locked release build from `aa123ca` passed (Rust1.99.0 GNU). Three adapter tests and11 runner/sanitizer tests passed. Three valid repeats each used5s warm-up and10s measured. [Sanitized evidence](miniquad-windows-pilot-20261112.json) retains exact samples and binary hash; raw diagnostics stay local.

| Run | Intervals | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1440 | 6.9444 | 7.5869 | 7.8270 |
| 2 | 1440 | 6.9552 | 7.6448 | 7.7853 |
| 3 | 1441 | 6.9553 | 7.6807 | 7.8053 |

All exits0, no resize; surface900x500. Private runtime backend reports OpenGl for all three runs. GPU/driver and actual vsync configuration not queried. Existing Miniquad defaults retained; measurements follow commit_frame in the draw callback. Builds finished before timing; no concurrent agent builds. User apps remained open; power/thermals/display rate unmeasured. Windows loadavg values are not CPU load measurements.

These are callback intervals, not displayed FPS/GPU time or an engine ranking. The Mac result near16.7ms versus Windows near6.95ms does not establish engine speed; pacing and hardware differ.

Post-timing regression: the exact release binary completed scripted replay/capture with exit0, no stderr,13 records ending tick1440/finished and6 PNG checkpoints. Tick600 inspected: both paddles, ball, center divider and score pip render. App-owned capture is not OS input testing or full visual acceptance. Completion fix emitted a single benchmark report and quit normally. All app processes exited; no installation qualification claimed.
