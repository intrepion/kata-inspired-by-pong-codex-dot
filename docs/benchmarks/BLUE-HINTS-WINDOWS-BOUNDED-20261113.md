# Blue Engine score-label and hint benchmark: Windows, 2026-11-13

This is a small same-host check of the recent visual changes, not a controlled performance comparison. Windows MSVC stable Rust 1.99.1 built the windowed release candidate at checkpoint `478a16e22258002828799f00bb21c90bd67b17e3`, Blue adapter Git blob `5cc550a49b29d8c3c21cefc814480eb73660a773`, executable SHA-256 `92bb5f4d13f3472a7a184dea26acd7a6a2e2821aaa470779df05c73f2779aded`. The preserved baseline executable SHA-256 `582cda7462ec785dde99c286718d2a72a92f9b9afa6635ddc2ba8b5426e19c1a` was built from adapter blob `d24364765c6ddd5a7699395a6211dcc9229e59b1`; it has zero-score outlines but predates both score-owner labels and court hints. Differences therefore cannot be attributed to hints alone.

Each native `--benchmark` run used the existing Blue Engine AutoVsync setting, a 5-second warmup and nominal 10-second sample of wall-clock update-callback intervals. The callback is before rendering submission; these are neither GPU times nor displayed frame rates. Runs alternated baseline, candidate, candidate, baseline. The application reported 1000×680 and no measured resize in all four runs. Other applications, machine load, power and thermal conditions were not controlled; the graphics driver/backend was not independently recorded.

| Run | Samples | Mean ms | p50 ms | p95 ms | p99 ms | Max ms | Exit |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| Baseline 1 | 1429 | 7.005 | 6.921 | 8.108 | 9.402 | 36.039 | 0 |
| Candidate 1 | 1415 | 7.067 | 6.969 | 8.440 | 9.619 | 12.453 | 0 |
| Candidate 2 | 1351 | 7.405 | 7.096 | 9.631 | 12.581 | 89.867 | 0 |
| Baseline 2 | 1430 | 6.989 | 6.939 | 8.012 | 9.032 | 30.556 | `0xC0000005` |

The second baseline run emitted its complete benchmark record and then exited with a Windows access violation; stderr was empty. Its cause and whether it is related to shutdown are unknown. All other stderr files were empty. The differences and candidate outlier warrant further controlled investigation, but two samples per build, the uncontrolled host and the additional label change do not establish a regression or its cause. Raw logs and preserved baseline/headless binaries remain in sibling workspace directory `blue-hints-benchmark-windows-20261113`. No physical input, license clearance or release qualification is implied.
