# Miniquad winner-fix Omarchy timing pilot

On 2026-11-13, Omarchy ARM64 ran a release-profile Miniquad pilot from checkpoint `d7d115e0a509fe34d0e847a1d60a779852f07c36`, containing contact-timing physics `c77bf3a` and winner-label fix `1fd1a6984812e5f103046793982cb880ae263a92`. Binary SHA-256 was `8f3d13a124ad2a9a649f05bc9762fc4cf9c4422f1fc4d654d2bbcb813d7fe828`. Offline locked release build and eight Miniquad tests passed, as did the shared runner/adapter and public-report test files. The existing binary was preserved at `/tmp/pong-miniquad-benchmark-prior-20261113/pong-miniquad`. No tracked source was modified during measurement.

Three bounded processes exited 0, reported no resize and passed current structural and interval-statistics validation. Miniquad reported its OpenGL backend in each run; GPU driver/version was not queried. Actual reported dimensions were 1474×1864. Each used five seconds warmup and a nominal ten-second sample window.

| Run | Samples | Mean ms | p50 ms | p95 ms | p99 ms | Max ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1200 | 8.334 | 8.333 | 9.281 | 11.036 | 13.065 |
| 2 | 1200 | 8.337 | 8.332 | 9.008 | 10.103 | 13.132 |
| 3 | 1200 | 8.339 | 8.331 | 9.022 | 10.800 | 15.332 |

[Allowlisted raw intervals](miniquad-winner-omarchy-20261113.json) preserve timing and source/binary provenance; full private runner output remains at `/tmp/pong-miniquad-winner-benchmark-20261113.json`. Display access was permitted. Samples are after Miniquad `commit_frame`, using its existing pacing; they do not measure GPU completion or display presentation. The benchmark scenario does not reach game over, so these runs do not measure winner-label rendering cost. Power, thermal state and background activity were not controlled or verified, and process visibility was limited. These callback intervals are not displayed FPS, a cross-engine ranking, or proof of physical gameplay or installation. Prior pilots remain historical.
