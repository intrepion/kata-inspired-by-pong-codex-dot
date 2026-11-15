# Nannou Windows callback-timing pilot

Fresh release completed in 24m39s; the default Nannou0.20/Bevy0.19 dependency build accounts for the long wait. Two release adapter tests passed. Updated runner/sanitizer suite: 21 passed, one POSIX-only signal test skipped on Windows. [Sanitized samples](nannou-windows-pilot-20261112.json).

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1200 | 8.3394 | 8.8193 | 8.9635 |
| 2 | 1199 | 8.3573 | 8.8134 | 9.0534 |
| 3 | 1200 | 8.3341 | 8.7790 | 8.9497 |

Three valid runs: 5s warmup/10s measurement, exit 0, no resize, surface1125x625. Runtime log identifies Intel UHD Graphics630/Vulkan. Mailbox presentation was unavailable and framework fell back to Immediate in all runs. This was automatic, with no backend/security changes. Existing app pacing retained; draw-recording callback intervals are not completed GPU/display FPS. Builds/tests finished first, no concurrent agent builds; user apps open, power/thermals unknown.

Replay/capture finished in 12.870s, exit 0 with informational renderer/capture logs, 13 state records and six PNGs. Tick600 inspected: score0:1, paddles, ball and divider visible. This does not prove OS input/focus/resize or resolve historical issues with other Bevy versions. All test processes exited.
