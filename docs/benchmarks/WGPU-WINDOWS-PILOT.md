# Raw wgpu Windows callback-timing pilot

Fresh release from bb0e922 built in 1m42s; two release adapter tests passed. [Sanitized evidence](wgpu-windows-pilot-20261112.json). Three valid 5s-warmup/10s-sample runs, all exit 0, no resize, actual surface 1125x625.

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 8143 | 1.1236 | 2.3391 | 2.8153 |
| 2 | 7825 | 1.1753 | 2.4408 | 2.9784 |
| 3 | 6672 | 1.3350 | 3.1651 | 3.8644 |

Default backend and existing AutoNoVsync/Poll policy retained. Sampling after successful submission/present excludes skipped frames. Callback intervals are not display FPS or GPU time; the increasing variability is retained, not filtered. No cross-engine speed conclusion. Driver, thermals and power unmeasured; other user applications open, no concurrent agent build during timing.

Replay/capture regression finished in 12.474s, exit 0 without stderr, 13 state records ending finished and six PNGs. Tick600 app-owned render target inspected: paddles, ball, score pip and divider visible. This does not validate OS keyboard, focus or resize. All processes exited; packaging gates unchanged.
