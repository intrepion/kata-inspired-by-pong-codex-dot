# Blue Engine Windows held-input debug timing pilot

On Windows, the Blue Engine windowed debug executable from checkpoint `85d423dbed3050261c3b996118a8fc439e27562b`, including held-input fix `582ab29`, was measured with the shared callback-interval runner at checkout `8bef3b70aa279873596ddca9f3ce43e9f8a1fe9a`. The executable SHA-256 was `1eca7e983a6f47c7b23f8ab165a406330bf2fd721ae3f2f36c8e04e96e377a20`, matching the [build recheck](../WINDOWS-BLUE-CONTACT-TIMING.md). The runner records its checkout revision, which alone does not prove binary provenance; the build record and hash provide the separate provenance. Profile was explicitly supplied as debug.

Three fresh processes completed normally with five seconds of warmup and nominal ten-second samples. The requested and reported surface was 1000x680, with no reported resize. The app's existing AutoVsync/default backend and pre-render callback boundary were retained; actual driver and display refresh were not queried. User applications remained open; power, thermals and system-wide idleness were not controlled.

| Run | Intervals | Mean ms | Median ms | p95 ms | p99 ms | Maximum ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 805 | 12.4153 | 12.2231 | 16.5694 | 16.9614 | 18.8713 |
| 2 | 790 | 12.6512 | 12.5383 | 16.6755 | 17.2069 | 18.3759 |
| 3 | 777 | 12.8615 | 12.8143 | 17.3308 | 18.1138 | 21.8615 |

[Sanitized interval data](blue-engine-windows-debug-held-20261113.json) retains exact samples and hashes. The private raw runner report remains in task workspace `blue-held-benchmark-windows-20261113` and was not published. These are CPU callback intervals for a mixed scripted workload, not GPU execution time, displayed FPS, physical gameplay, accessibility verification or a clean-machine/package test. The debug build, unknown graphics conditions and uncontrolled host load make this a harness pilot, not a cross-framework, OS or release-build comparison. The earlier release pilot remains historical and is not directly compared.
