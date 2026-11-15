# Nannou Mac callback-timing pilot

2026-11-12 UTC. Offline locked release build from `f34a14f` passed in 9m52s; 2 adapter tests and 14 shared runner/sanitizer tests passed. Harness snapshot `d49992e` adds only README activity. Runtime framework logs reported Metal. GPU time and display presentation were not measured.

## Initial short sample retained

The [initial batch](nannou-mac-initial-20261112.json) had one invalid run: normal exit, stable dimensions, but only 8.496 seconds of sampled intervals, below the runner's 9.5-second minimum. Its later two runs were valid. Startup/callback delay may explain the short first interval span, but the cause was not isolated. This batch is retained and is not reported as 3/3 or silently overwritten.

## Separately named repeat batch

[Repeat evidence](nannou-mac-repeat-20261112.json) contains three valid runs. Each used five seconds nominal warm-up and ten seconds nominal measurement, stable 1800×1000 dimensions and exit 0.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1198 | 8.327 | 9.275 | 9.773 |
| 2 | 1201 | 8.331 | 9.261 | 9.481 |
| 3 | 1172 | 8.433 | 9.823 | 12.863 |

Nannou 0.20 uses Bevy/wgpu; sampling occurs at the end of view draw-command recording before framework rendering/presentation. Existing 120 Hz update pacing remains unchanged. Builds/tests completed first, worker quiet windows were reserved, but user applications remained open and power/thermal/whole-machine idle conditions were unverified. Reports are allowlisted copies; private machine/environment diagnostics remain local. Different callback boundaries, pacing and dimensions prevent an engine-performance ranking.

A separate capture replay completed in 13.132 seconds: exit 0, 13 states, tick 1440 and six PNGs. The tick-600 app-owned frame was inspected: score, court, paddles and ball visible without an obvious rendering failure. No new real-input or installer qualification is claimed. All processes exited.
