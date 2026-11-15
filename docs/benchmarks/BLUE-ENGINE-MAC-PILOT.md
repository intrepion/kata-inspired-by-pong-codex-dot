# Blue Engine Mac native update timing

2026-11-12 UTC. Offline locked native release build from `6f2dc03` passed in 82 seconds; 2 adapter tests and 18 shared runner/sanitizer tests passed. Cargo retained the upstream block 0.1.6 future-incompatibility warning. [Sanitized evidence](blue-engine-mac-pilot-20261112.json) identifies binary SHA and exact timing samples.

| Repeat | Samples | p50 ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 600 | 16.739 | 18.436 | 19.017 |
| 2 | 600 | 16.758 | 18.328 | 19.120 |
| 3 | 600 | 16.757 | 18.236 | 18.840 |

Three valid native-windowed repeats, configured surface 1000×680, no resize, normal exit 0. This includes the shared window-lifetime teardown guard; no forced termination was used to obtain successful timing exits. The existing AutoVsync request remains unchanged. Actual backend/driver and display pacing were not queried.

Sampling follows object-state changes before framework rendering; object upload precedes the callback, so changes reach a later render pass. It is **not GPU completion or displayed FPS**, and differing pacing/callback boundaries prohibit rankings. Each repeat used five seconds warm-up and ten seconds measurement. Builds/tests completed before a worker quiet window; user apps remained open and power/thermal/whole-machine idle were unverified. Private diagnostics remain local. All benchmark processes exited.

Separate native scripted replay completed in 12.173 seconds: exit 0, 13 state records, tick 1440 and finished=true. Its existing replay termination path is not a graceful-shutdown test. No new native screenshot or visual/input acceptance is claimed; the separate headless screenshot feature was not used as a substitute. Packaging remains separately unqualified.
