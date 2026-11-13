# SDL3 revised-physics Omarchy timing pilot

On 2026-11-13, Omarchy ARM64 ran a release-profile SDL3 pilot from checkpoint `25d32e273738a65d9eadee24d8915b922b31d539`, containing contact-timing physics `c77bf3a`, with binary SHA-256 `ffc91cbe99bee7b75e44558a7b03f40a6bb4d0ffb85da0558279d8ced39e1af3`. An offline locked release build and both SDL3 adapter tests passed. The existing binary was preserved under `/tmp/pong-sdl3-benchmark-prior-20261113/`; the shared runner/adapter and public-report test files passed. No tracked source was modified during measurement.

Three bounded processes exited 0, reported no resize, and passed current structural and interval-statistics validation. SDL3 reported the Wayland video driver and OpenGL renderer in all three runs; the underlying GPU driver/version was not queried. Actual reported dimensions were 737×932, different from the 1474×1864 reported by other pilots; do not infer equivalent drawable or display conditions. Each used five seconds warmup and a nominal ten-second sample window.

| Run | Samples | Mean ms | p50 ms | p95 ms | p99 ms | Max ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1173 | 8.521 | 8.374 | 9.483 | 10.373 | 13.876 |
| 2 | 1180 | 8.471 | 8.321 | 9.207 | 10.081 | 12.920 |
| 3 | 1178 | 8.488 | 8.367 | 9.125 | 9.852 | 11.984 |

[Allowlisted raw intervals](sdl3-c77-omarchy-20261113.json) preserve timing and source/binary provenance; full private runner output remains at `/tmp/pong-sdl3-c77-benchmark-20261113.json`. Display access was permitted. The existing 8 ms sleep remains; callbacks are sampled after `canvas.present` returns and vsync is not explicitly configured. Power, thermal state and background activity were not controlled or verified, and process visibility was limited. These callback intervals are not displayed FPS, GPU completion, a cross-engine ranking, or proof of gameplay or installation. Earlier pilots remain historical.
