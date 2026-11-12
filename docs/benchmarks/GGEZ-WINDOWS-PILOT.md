# ggez Windows callback-timing pilot

Fresh release built from 4d29d2d in 3m40s; two release adapter tests passed. [Sanitized evidence](ggez-windows-pilot-20261112.json). Three valid runs: 5s warmup/10s sampling, exit 0, no resize, 1000x680 surface.

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1143 | 8.7400 | 9.1924 | 9.3484 |
| 2 | 1143 | 8.7395 | 9.1829 | 9.3239 |
| 3 | 1142 | 8.7601 | 9.1912 | 9.3613 |

Existing framework pacing retained. Sampling after Canvas.finish is a callback/submission boundary, not final swap completion, displayed FPS or GPU time. Driver unqueried; no backend override. No concurrent agent builds during timing, other user apps open; power/thermals unmeasured.

Replay/capture finished in 12.503s, exit 0 and no stderr, 13 state records and six PNGs. Tick600 inspected: paddles, ball, court, readable header/score and footer controls visible. App-owned render target capture does not certify OS keyboard/focus/resize. All test processes exited; packaging remains separate.
