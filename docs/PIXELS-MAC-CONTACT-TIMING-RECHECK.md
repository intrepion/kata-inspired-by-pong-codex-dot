# Mac Pixels contact timing recheck

macOS ARM64. Source checkpoint `663aa403e04a571cb1b43d9731fcb83090630436` includes physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. Offline locked release build reused a copy-on-write, timestamp-preserved copy of the Pixels dependency cache and recompiled four local Pong crates. Binary SHA-256: `80ad65c944d02fa1c3f5290e27f7b3615fe0ba757c5db6e33120ad1a2d671b86`.

The exact binary was packaged locally and opened through the supported desktop tool. Enter started play; held W/S moved the player paddle in opposite directions; Space showed Paused; Enter restarted at 0–0 Playing. A subsequent match reached 1–7 Over, and Enter restarted to 0–0 Playing. The window close control exited the pilot app according to desktop inventory. Screens were visually inspected, not saved. This is native desktop automation, not physical keyboard input or verification of a specific collision trajectory.

Direct shell launch with `--scripted-test --scripted-captures` emitted no telemetry or captures after more than 15 seconds and was stopped (exit 130). System logs reported a connection-invalid error; the root cause was not established. No fresh scripted replay or app-owned capture pass is claimed. Normal app-bundle launch worked; historical evidence and binaries remain unchanged. No signing or clean-machine qualification is inferred.
