# Windows Macroquad benchmark pilot

2026-11-12; source f832743 (40c40f2 plus the prior Windows evidence), with the runner fixture portability change recorded in working-tree metadata. Fresh release build, Rust1.99.0 Windows GNU, Core i7-9750H,12 logical CPUs, about31.9GiB RAM.

Three5-second warmup +10-second measurement runs used the shared runner. Stable800x544 Macroquad logical units, not the requested1000x680. No concurrent agent builds or emulator process during sampling; other user applications remained open. Windows loadavg zeros do not measure load. Power, thermal state, display rate and active GPU were not verified. Installed Intel UHD630 driver31.0.101.2145 and Microsoft Basic Display Adapter10.0.26100.1 do not prove which renderer was active.

| Run | Samples | p50 ms | p95 ms | p99 ms | Max ms |
|---|---:|---:|---:|---:|---:|
|1|1440|6.9453|7.5843|7.6833|9.6233|
|2|1440|6.9472|7.4897|7.6887|8.4230|
|3|1440|6.9420|7.3086|7.6285|9.1462|

[Raw samples](macroquad-windows-pilot-20261112.json); machine hostname redacted. Callback intervals are not displayed FPS or GPU-time metrics. Different hardware, resolution, pacing and OS make cross-machine ranking invalid. No manual gameplay, package/install, input latency or power claim follows from this pilot. Processes exited normally.

Current Windows portable-snapshot test fails on small floating-point differences from the Omarchy baseline; core9, sampler3 and scenario replay5 unit tests pass. The mismatch is explicitly retained as an open correctness criterion rather than silently treating cross-platform floats as bit-identical.
