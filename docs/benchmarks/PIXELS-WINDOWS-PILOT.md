# Pixels Windows callback-timing pilot

2026-11-12 UTC. Fresh locked release build from f57d76e completed in 2m35s. Three release adapter tests passed. [Sanitized samples](pixels-windows-pilot-20261112.json); raw diagnostics remain local. The only uncommitted change during timing was a README link correction.

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1441 | 6.6275 | 10.2000 | 10.4608 |
| 2 | 1440 | 6.8782 | 9.4993 | 9.7036 |
| 3 | 1440 | 6.8417 | 9.4906 | 9.9002 |

All three repeats valid: 5s warmup and 10s sampled, exit 0, no resize, reported surface 1125x625. CPU raster with wgpu presentation, existing Poll/default backend retained. GPU/driver identity not queried. These are callback intervals after pixels.render, not displayed FPS or an engine ranking. Builds finished first; other user apps open and power/thermals unmeasured.

Scripted replay/capture completed in 12.540s, exit 0 without stderr: 13 records ending finished, six PNG checkpoints. Tick600 inspected: paddles, ball and divider visible. This is the app-owned 900x500 CPU buffer, not the scaled desktop surface or OS-input acceptance. All processes exited. No installer qualification added.
