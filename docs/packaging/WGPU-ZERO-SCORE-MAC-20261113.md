# Raw wgpu zero-score marks — macOS, 2026-11-13

Mac cross-check of the shader change at `b53ce5e5`. All 13 offline locked release-profile tests passed. In the native window, Ready at 0–0 showed two outlined zero marks; at 1–0 the player side showed one solid tally and the CPU side an outlined zero. After a 2–2 pause, supported desktop automation sent Return and Space: the paused 0–0 state showed both zeros again. Escape closed the app. These are native visual observations with automated commands, not physical input.

The candidate executable SHA-256 is `01209c79023278f73dfc9baf39145cabc151740b6b71b100221bba6285752b4d`; shader SHA-256 is `9a64b58b9f0ef880442e7104b39422e142915edfb867e923212163ed2fa65790`. The isolated mixed source tree used shader blob `706fc55207ba9042aad08c8e5765472f8a7c0b2a`, adapter blob `cc1c49ff555f8e0ef650f8614f9886eaf9673d8f`, and renderer blob `9ed876996cfc35b432b13a8ad367ebb8a171203c` matching checkpoint `30d6bb3fd2cf5baf7f0bb9edbbd0642bff1f11f4`; other build inputs were not all compared. Prior shader and executable and the candidate app remain under `/tmp/pong-wgpu-zero-score-mac-20261113-v1`.

Other score combinations, sizes and platforms, physical input and release qualification remain open. The [Mac Pixels and wgpu license screen](PIXELS-WGPU-MAC-LICENSE-SCREEN.md) is not complete native/system dependency or distribution clearance.
