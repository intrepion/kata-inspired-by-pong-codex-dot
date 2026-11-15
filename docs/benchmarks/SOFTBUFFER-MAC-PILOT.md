# Softbuffer macOS callback-timing pilot

2026-11-12 UTC. Offline locked release build succeeded. Three bounded runs completed successfully using the shared harness,5seconds warm-up then10seconds measurement per run. [Raw timing evidence](softbuffer-mac-pilot-20261112.json) records revision, binary hash, dimensions and individual intervals; host identity/environment diagnostics were removed from publication.

| Run | Intervals | Median ms | p95 ms | p99 ms |
|---|---:|---:|---:|---:|
|1|645|15.542|16.057|16.472|
|2|645|15.540|16.039|16.320|
|3|649|15.493|15.974|16.232|

Native macOS CPU raster + softbuffer, winit8ms WaitUntil, sampling after buffer-present submission. All samples valid, no resize, all child exits0. No build ran concurrently in this task; other user apps remained open. Power, thermals, complete system idleness and runtime driver string were not measured. These are **callback intervals**, not displayed FPS, GPU timings or an engine ranking. Different hardware and pacing make cross-machine rankings invalid. No new manual gameplay or installer qualification is implied.
