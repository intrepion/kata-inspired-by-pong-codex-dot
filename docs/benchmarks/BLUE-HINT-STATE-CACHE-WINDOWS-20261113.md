# Blue Engine hint state updates: Windows, 2026-11-13

Blue Engine previously rewrote all hint rectangle visibility and positions on every update, even when its state was unchanged. It now updates those existing rectangles on the first frame and when Ready, Playing, Paused or Over changes. Hint geometry, input and physics are unchanged. On Windows MSVC stable Rust 1.99.1, all 16 locked offline windowed-feature release tests passed, including a first-frame and state-transition regression. The tested adapter Git blob is `9596606e2fc1d2f809aa3c18ad02418df5d78b58`, based on checkpoint `a5ee23663bdfcc0c2f8d25bfe7ec70869a1a1b90`.

A locked offline headless release build (SHA-256 `4beab8decc3eca66b760a650b656b5e62419b41c3f64c79bafc526b896d7478c`) exited 0 with empty stderr in scripted capture mode; all 13 parsed telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Five of six capture PNGs were byte-identical to the earlier [headless hint run](../packaging/BLUE-COURT-HINTS-WINDOWS-20261113.md), including paused tick 480 and playing tick 1320. Tick 240 differed by 14 pixels within image coordinates x=511–512, y=322–328, where the ball crosses the net; the cause was not established. The inspected captures showed no hint in play and the resume hint when paused. Ready and game-over presentations were not captured.

A bounded native windowed callback comparison used the prior release executable SHA-256 `92bb5f4d13f3472a7a184dea26acd7a6a2e2821aaa470779df05c73f2779aded` and the newly built release executable SHA-256 `6c6296a747bb5547aa8286cbe56f1fdbd5b27bf27a1bc7f03ccfa367b7075f19`. In order old, new, new, old, all four `--benchmark` runs exited 0 with empty stderr, reported 1000×680 with no measured resize, and used the existing AutoVsync setting. Each sampled callback intervals after five seconds warmup for a nominal ten seconds; these are not GPU times or displayed FPS. Other applications, load, thermals and power were not controlled.

| Run | Samples | Mean ms | p50 ms | p95 ms | p99 ms | Max ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Old 1 | 1413 | 7.099 | 6.981 | 8.074 | 9.122 | 93.566 |
| New 1 | 1404 | 7.134 | 6.974 | 8.470 | 10.793 | 85.536 |
| New 2 | 1410 | 7.110 | 7.000 | 8.126 | 9.231 | 82.815 |
| Old 2 | 1431 | 7.034 | 6.943 | 7.921 | 8.906 | 88.525 |

These short noisy samples do not demonstrate a speedup or establish a regression. Logs, captures and prior/current binaries remain in sibling workspace directory `blue-hint-state-cache-windows-20261113`. Native and physical input, accessibility, other states and sizes, complete dependency and system-library review, clean-machine installation and release qualification remain open. No license clearance is claimed.

The concurrent [Mac transition-frame investigation](../packaging/BLUE-TRANSITION-ORDER-MAC-20261113.md) describes a possible visibility/buffer ordering issue. State-change caching was not tested with synchronized consecutive presented frames and is not claimed to fix or diagnose it.
