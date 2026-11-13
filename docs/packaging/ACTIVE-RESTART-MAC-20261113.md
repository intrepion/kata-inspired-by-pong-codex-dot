# Active restart held-input cross-check on macOS

2026-11-13. The source files for checkpoint `18e83789c37ee016a4d0a7128b48027c6b797c6c` were fetched into the isolated overlay described in [the earlier Mac report](NONPLAYING-INPUT-MAC-20261113.md). Locked offline release-profile tests passed on macOS ARM64:

| Component | Passed | Tested source SHA-256 |
| --- | ---: | --- |
| shared GUI support | 13 | `94e72100690cb43106f52027e2b9e9a658284b4987dd850d310b18efe35320ff` |
| Pixels | 10 | `dad486eb852920592f535313b55176cf301f55b36bbc5aee073950d178e746e1` |
| raw wgpu | 9 | `aa01c0a857a425e06f34c3127489138405b0db7d22c836e05e11435f462bad64` |

Each suite includes its new active-restart held-movement regression. Pixels and wgpu used separately cloned cached targets; the GUI support target was the existing isolated test target. These are adapter/model tests, not native delivery or physical input. No graphical replay, desktop interaction, clean-machine package or release qualification is claimed. Source and test targets remain under `/tmp/pong-mac-input-recheck-20261113-v1`, `/tmp/pong-mac-input-target-20261113-v1`, `/tmp/pong-pixels-restart-mac-target-20261113-v1` and `/tmp/pong-wgpu-restart-mac-target-20261113-v1`; previous artifacts were retained.

## ggez, Miniquad and Piston follow-up

The source at checkpoint `99c6a657fa5e7536fc18f9fe68fadab52c3c26c8` was fetched into the same isolated overlay. Offline locked release-profile macOS tests passed for ggez (7), Miniquad (13) and Piston (12), each including `active_restart_clears_held_movement`. Tested main.rs SHA-256 values: ggez `f056be1a49c5c83961b3f8b469ca9d5d49bbd3f1a2b26b3d7f8bb2680eb84305`; Miniquad `24a4463440a27f82bc8e304cfe226adc8997014a812e6e0cba36fff8999c236c`; Piston `f64b6d5d9fb7c749439b9c00fa9f3fe33e4acb7f559eb5f2d020266f65966c5c`. These are adapter/model tests using the existing isolated cached target; no rebuilt frontend replay, actual held keys or physical input is claimed.
