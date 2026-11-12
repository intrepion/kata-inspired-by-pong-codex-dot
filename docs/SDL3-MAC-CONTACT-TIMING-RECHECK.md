# Mac SDL3 contact timing recheck

2026-11-12, macOS ARM64. Source checkpoint `082327be3c710e27263b321dfa8db4f47ef24bfb` includes physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. Offline locked release build reused a copy-on-write, timestamp-preserved copy of the SDL3 dependency cache and rebuilt only five local Pong crates. Binary SHA-256: `56dac410858f86a239790df2e3d8dd770b0a22d299fa86b05183eeeac445d987`.

The exact binary was packaged locally and opened through the supported desktop tool. Space started play; held W/S moved the paddle in opposite directions; Space showed Paused; R reset to 0–0 Playing. A subsequent match progressed to 4–7 Over, and R restarted to 0–0 Playing. Escape exited the app according to desktop inventory. Screens were visually inspected, not saved. This is native desktop automation, not physical keyboard input or proof of a specific collision trajectory.

Direct shell launch with `--scripted-test --scripted-captures` exited 1 before replay: SDL reported that the video driver did not add any displays. No fresh scripted replay or app-owned captures are claimed. Normal app-bundle launch worked; earlier evidence and binaries remain historical. No signing or clean-machine qualification is inferred.
