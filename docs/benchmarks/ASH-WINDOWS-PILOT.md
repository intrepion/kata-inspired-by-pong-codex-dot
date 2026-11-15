# ash Windows callback-timing pilot

Fresh release built in 2.62s; four release tests passed. [Sanitized evidence](ash-windows-pilot-20261112.json).

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1031 | 9.0156 | 16.4055 | 17.3465 |
| 2 | 1049 | 9.0100 | 14.3192 | 17.4535 |
| 3 | 1039 | 9.0190 | 15.2428 | 17.0999 |

Three valid runs: 5s warmup/10s sample, exit 0, no resize, surface1125x625. Vulkan device log identifies Intel UHD Graphics630. Existing serialized FIFO path retained; no driver overrides. Submission/present callback cadence is not displayed FPS or GPU timing. Builds complete first, no concurrent agent builds; user apps open, power/thermals unknown.

Replay/capture finished in 13.028s, exit 0 with device-info stderr only, 13 records ending finished and six PNGs. Tick600 inspected: readable score, paddles, ball and divider. These are app-owned captures, not native OS input/focus/resize qualification. All processes exited; installer/distribution gates unchanged.
