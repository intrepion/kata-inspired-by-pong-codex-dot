# Shared GUI non-playing movement input on Omarchy

2026-11-13, Omarchy ARM64. GTK, Slint and GPUI share `pong-gui-support::Model`. A focused model regression reproduced the same defect: W pressed in Ready remained active after Enter started play. The regression failed before the fix. The model now suppresses directional presses made outside Playing until release; repeats cannot rearm them, while a fresh press during play works. The new test covers Ready, Paused and Over and all four W/Up/S/Down bindings. An existing down-alias test now starts the game before asserting active movement, preserving its original alias purpose under the new behavior.

All 11 shared-model tests passed with locked offline dependencies, including pause, repeats, case normalization, focus and aliases; formatting passed. The tested support source Git blob is `68032c026c5fcdf2c23a96540f2f78d72f1fb965` (SHA-256 `9c02b55aeba64094b837ea3d6303daad36ca4ad5548d1c624c8722dfd45814dc`). These are model tests, not native or physical key delivery.

Separate copied targets produced locked offline release builds for GTK, Slint and GPUI. Three bounded scripted runs exited 0, each matching all 13 current model telemetry records. GTK and Slint produced six app-owned captures each; their inspected tick 480 frames showed paused status, court, paddles and ball. The GTK Cairo capture SHA-256 is `bd48da3286920165f3779e767b2e738982fdd8abd26467c29afd0f692ed8fe89`; the Slint own-window snapshot SHA-256 is `1f33c9dfa8944cbb6f8b2e62860f175c376727391cf3ba58427afd0dec7146c6`. GPUI replay is state-only. These outputs are not desktop screenshots, and replay does not exercise actual keys.

| Adapter | Release executable SHA-256 |
| --- | --- |
| GTK | `4d02b8009b1444f5b50252b040fc9e2a7d2ac2e53ba4ff629544e17dd66e5c23` |
| Slint | `84d8d84a1b107ae3e4ec5c68898aa977fa60c1eedec97ce94a1c3874dc46b6d0` |
| GPUI | `5a36ba15dc0204cc8079cb44cd3c916a33b23a0730398e198097375bc648c791` |

The GPUI build retained its `proc-macro-error2 2.0.1` future-incompatibility warning. Builds, isolated working directories, logs and captures remain under `/home/intrepion/Documents/Codex/2026-11-12/task/pong-gui-nonplaying-20261113-P7PXxp`. Native/physical keyboard behavior, Mac/Windows rechecks, clean-machine operation, dependency/notice review under the owner's constraints, and release qualification remain open; prior artifacts were preserved.
