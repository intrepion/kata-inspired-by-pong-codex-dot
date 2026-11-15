# Macroquad Mac callback-timing pilot

Recovered and validated measurement evidence from the completed 2026-11-12 06:56 UTC run: [JSON](macroquad-mac-pilot-20261112.json). Exact source revision and binary hash are in the publication record. No new timings were inferred from packaging work.

macOS ARM64; release; 1000×680 recorded drawable; Miniquad macOS default OpenGL path source-verified, runtime driver string not queried. Refresh rate, machine idleness and thermal state unverified. Private machine inventory and process diagnostics omitted; original evidence retained locally.

Three runs each used5seconds warm-up and10seconds measured,600 callback intervals per run, no resize, all exited0 and valid.

| Run | Median ms | p95 ms | p99 ms |
|---|---:|---:|---:|
|1|16.664|17.267|17.685|
|2|16.666|17.311|17.663|
|3|16.663|17.611|17.873|

These are application frame-callback intervals, **not displayed FPS, GPU time, or a cross-platform engine ranking**. Other hardware, pacing and presentation conditions differ. Native GPU benchmarks could not be rerun during the later packaging execution session because OpenGL pixel-format initialization failed; retain the timestamp and scope of the successful earlier measurement.
