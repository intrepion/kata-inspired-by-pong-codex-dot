# Blue Engine Windows held-input release timing pilot

On Windows, the Blue Engine windowed adapter was built from checkpoint `ae0ad4da30da5931a93780d7e3a09c0ba0f2ff66`, including held-input fix `582ab29`, with locked offline dependencies and Rust 1.99.1 MSVC. All four optimized adapter/benchmark tests passed. The separately built release executable SHA-256 was `457bd2be5448e899bd44936e1b6bf102804fa58af2a6d9f22df4a5dae5dcfd82`; its source blob was `613b25e8d280bed7956741850e547e10dfc2f4a5`. The runner records the checkout revision and caller-supplied profile; neither field independently proves binary provenance. The build and hash provide the separate record.

Three fresh processes completed normally with five seconds of warmup and nominal ten-second samples. The requested and reported surface was 1000x680, with no reported resize. Existing AutoVsync/default backend and pre-render callback boundary were retained; actual graphics driver and display refresh were not queried. The bounded process check found no matching build, emulator or Pong process immediately before measurement. User applications remained open; power, thermals and system-wide idleness were not controlled.

| Run | Intervals | Mean ms | Median ms | p95 ms | p99 ms | Maximum ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1440 | 6.9436 | 6.9479 | 7.6693 | 8.1803 | 9.6264 |
| 2 | 1440 | 6.9437 | 6.9404 | 7.6389 | 8.1937 | 9.2602 |
| 3 | 1440 | 6.9442 | 6.9291 | 7.6639 | 8.2500 | 9.4418 |

[Sanitized interval data](blue-engine-windows-release-held-20261113.json) retains exact samples and hashes. The private raw report remains in task workspace `blue-held-benchmark-release-windows-20261113`; earlier binaries and the debug pilot are preserved. These are CPU callback intervals for a mixed scripted workload, not GPU execution time, displayed FPS, physical gameplay, accessibility verification or package qualification. Unknown graphics conditions and uncontrolled host load make this a local pilot, not a controlled framework or OS ranking. The earlier debug pilot is not treated as a controlled comparison.
