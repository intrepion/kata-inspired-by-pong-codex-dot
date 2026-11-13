# Macroquad revised-physics Omarchy timing pilot

On 2026-11-13, Omarchy ARM64 ran a release-profile Macroquad pilot from checkpoint `a6aae4ad83885b5526285a11347a5aeb7b98c583`, containing contact-timing physics `c77bf3a`, with binary SHA-256 `e916a0ec2665724803d0c503baa5d2a674e028d67ea0dda80f0e8242f09aaba4`. An offline locked release build passed, as did release tests for benchmark (3), core (19) and scenario (8). The previous binary was preserved under `/tmp/pong-macroquad-benchmark-prior-20261113/`. Only the runner test fixture file was modified in the working tree during the run; no Macroquad or physics source was changed.

Three independent bounded processes completed with no resize and passed the current structural and timing-consistency validation. Actual reported dimensions were 1474×1864. Each used a five-second warmup and nominal ten-second interval sample.

| Run | Samples | Mean ms | p50 ms | p95 ms | p99 ms | Max ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1200 | 8.334 | 8.331 | 10.397 | 11.904 | 15.602 |
| 2 | 1200 | 8.333 | 8.333 | 9.382 | 11.500 | 14.699 |
| 3 | 1199 | 8.333 | 8.331 | 9.791 | 12.995 | 14.634 |

[Allowlisted raw intervals](macroquad-c77-omarchy-20261113.json) preserve timing and binary/source provenance; full private runner output remains at `/tmp/pong-macroquad-c77-benchmark-20261113.json`. The local display was used with permission. Backend driver, power/thermal state and background activity were not controlled or verified; process visibility was limited. These are callback intervals under existing pacing, not displayed FPS, GPU timing, a cross-engine ranking or proof of a full gameplay/installation pass. Prior pilot results remain historical.
