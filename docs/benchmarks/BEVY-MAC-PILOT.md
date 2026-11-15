# Bevy Mac main-world callback pilot

2026-11-12 UTC. Offline locked release build from `592fe18` passed in 8m20s; 2 adapter tests and 17 shared runner/sanitizer tests passed. Harness snapshot `0eee28c` adds only README activity. Runtime framework logs reported Metal. [Sanitized evidence](bevy-mac-pilot-20261112.json) records binary hash and exact intervals.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 601 | 16.744 | 17.868 | 18.682 |
| 2 | 600 | 16.745 | 17.896 | 18.702 |
| 3 | 600 | 16.731 | 17.896 | 18.155 |

Three valid repeats, stable physical window 2000×1440, all exits 0, five seconds warm-up and ten seconds measured. Samples are the main world's Last schedule, **not rendered-frame/GPU-completion/displayed-FPS measurements**; Bevy renders separately. Existing default window/renderer/event-loop configuration retained. Different callback boundaries and pacing prevent an engine ranking.

Build/tests completed before a reserved worker quiet window; user apps remained open, power/thermal/whole-machine idle unverified. Private diagnostics remain local. All timing processes exited.

Separate scripted replay/capture completed in 12.993 seconds, exit 0, 13 states, tick 1440 and six PNGs. Tick 600 was inspected: court, both paddles, ball, score and instructions visible without an obvious rendering failure. This is bounded framebuffer evidence, not new real-input or packaging qualification.
