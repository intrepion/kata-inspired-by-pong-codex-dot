# SDL3 macOS callback-timing pilot

2026-11-12 UTC. Offline locked release build succeeded; shared runner completed three valid repeats,5seconds warm-up plus10seconds measurement each. [Timing JSON](sdl3-mac-pilot-20261112.json) retains source revision, binary hash, raw intervals and runtime backend, with private host/environment/process diagnostics omitted.

SDL reports `cocoa` video driver and `metal` renderer. Reported dimensions900×500; no resize; all exits0. Existing8ms sleep retained; measurement follows present-return, vsync not explicitly configured. Other user apps remained open; power/thermals/whole-machine idleness unverified. Builds completed before measurement.

| Run | Intervals | Median ms | p95 ms | p99 ms |
|---|---:|---:|---:|---:|
|1|1001|10.206|10.424|10.512|
|2|996|10.238|10.477|10.545|
|3|1000|10.222|10.448|10.544|

These are **frame-callback intervals, not displayed FPS or GPU time**. Pacing and dimensions differ from the other adapters, so these numbers do not establish an engine ranking. No new manual input or installer pass is implied. All benchmark child processes exited.

Post-measurement regression checks: SDL3 adapter2/2 and shared Node runner5/5 passed on this Mac. These correctness checks are separate from benchmark samples.
