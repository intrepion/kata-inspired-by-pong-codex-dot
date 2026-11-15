# Softbuffer revised-physics Omarchy timing pilot

On 2026-11-13, Omarchy ARM64 ran a release-profile softbuffer pilot from checkpoint `23e8a50abdfe1da425aaf57b365523ab019024c5`, containing contact-timing physics `c77bf3a`, with binary SHA-256 `006b1d240a4a48fbbeba18a6a44b8ce84b3cc4025c41f6b93d29881a0443995e`. An offline locked release build and all four adapter tests passed. The existing binary was preserved under `/tmp/pong-softbuffer-benchmark-prior-20261113/`; the shared runner/adapter and public-report test files passed. No tracked source was modified during measurement.

Three bounded processes exited 0, recorded no resize, and passed the current structural and interval-statistics validation. Actual reported dimensions were 1474×1864. Each used five seconds warmup and a nominal ten-second sample window.

| Run | Samples | Mean ms | p50 ms | p95 ms | p99 ms | Max ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 893 | 11.200 | 11.018 | 13.895 | 15.401 | 17.843 |
| 2 | 885 | 11.305 | 10.967 | 14.062 | 15.431 | 17.920 |
| 3 | 887 | 11.269 | 10.997 | 13.984 | 15.040 | 21.331 |

[Allowlisted raw intervals](softbuffer-c77-omarchy-20261113.json) preserve timing and source/binary provenance; the full private runner output remains at `/tmp/pong-softbuffer-c77-benchmark-20261113.json`. Display access was permitted. Softbuffer uses CPU raster and a winit 8 ms WaitUntil, with samples taken after buffer-present submission; this is not a GPU completion or display timestamp. Driver, power, thermal state and background activity were not controlled or verified, and process visibility was limited. These callback intervals are not displayed FPS, a cross-engine ranking, or proof of gameplay or installation. Earlier pilots remain historical.
