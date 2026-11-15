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

## Fyrox, Nannou and raylib follow-up

The source from checkpoint `39d3037e0a662ee59a74dedd25d84548a9fdcbd6` was fetched into the same isolated Mac overlay. Offline locked release-profile tests passed for Fyrox (7), Nannou (10) and raylib-rs (10), including each `active_restart_blocks_held_direction_until_release` regression. Tested main.rs SHA-256 values: Fyrox `025f40d4cd6331a9a2db73705e7752ba6bd7ad5a7ebd8bfbddefa08fb268db05`, Nannou `f65cdf7482ac1118e3dc5e2d37cbc5937c755c09b0288fa7ce8265c9ee129268`, raylib-rs `6c951aee26cf134dc09ad3479524be3abe7173976404682ef66b7aa9dc00f16f`. These are adapter/model tests, not graphical replay, operating-system key delivery or physical input.

## SDL3, ash, softbuffer and Blue Engine follow-up

Fetched the landed active-restart fix `c880abb491e4c113c8db5cd1186a62dffd44c618` into the same isolated Mac overlay. Offline locked release-profile tests passed for SDL3 (9), ash (11), softbuffer (9) and Blue Engine (8), each including `active_restart_suppresses_movement_until_fresh_press`. Tested main.rs SHA-256 values: SDL3 `6de67a8fea8e238039f4fba7c59a7c4bda8d86c34a084f88b762004ce85a5bea`; ash `d2dc8bd55c0eb618db69fedc411482d86d9bb3884f58682bb4e1765333af0f55`; softbuffer `f11818fce657d5d1780f764bbf9a7792b2181f2b58e2e13b4f57cb258f5f2b09`; Blue Engine `37ddaddea395e3604798d2782ebb401b4b7048b1a60dda5211d7e8ccd6e1ef1e`. Existing isolated cached targets were reused. These are adapter/model tests, not graphical replay, Vulkan runtime, operating-system key delivery or physical input.
