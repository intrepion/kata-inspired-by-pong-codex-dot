# Raw wgpu Mac callback-timing pilot

2026-11-12 UTC. Offline locked release build passed in 34.85 seconds; 2 adapter tests and 16 shared runner/sanitizer tests passed. Runtime source is the shared wgpu adapter from `27760d3`; report snapshot `b4dfb6a` also includes unrelated reports/bracket-lib changes. [Sanitized timing evidence](wgpu-mac-pilot-20261112.json) records exact binary hash and samples.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 599 | 16.708 | 17.656 | 18.230 |
| 2 | 600 | 16.713 | 17.896 | 18.182 |
| 3 | 600 | 16.696 | 17.817 | 18.146 |

Three valid repeats with stable 1800×1000 surface, exit 0, five seconds warm-up and ten seconds measured. Existing winit Poll/default surface configuration retained. Only successful present requests count; skipped/recovered surfaces do not. Runtime backend/driver and actual display pacing were not queried. This is not displayed FPS or measured GPU completion.

Build/tests completed before a reserved worker quiet window; user apps remained open and power/thermal/whole-machine idle conditions were unverified. Different dimensions, callback boundaries and pacing prevent an engine-performance ranking. No new real-input or packaging qualification is claimed. All benchmark processes exited; private diagnostics remain local.

Separate scripted replay/capture completed in 12.355 seconds: exit 0, 13 records, tick 1440 and six PNGs. The tick-600 app-owned frame was inspected: court, paddles, ball and score pip visible without an obvious rendering failure. This bounded framebuffer sample is not exhaustive visual QA or an OS-input pass. Raw capture artifacts remain local.
