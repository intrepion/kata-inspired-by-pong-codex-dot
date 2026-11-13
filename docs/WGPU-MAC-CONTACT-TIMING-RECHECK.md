# Mac raw wgpu contact timing recheck

macOS ARM64. Source checkpoint `617160ceb4573ff06f7339c7dbc98787d54f55df` includes physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. Offline locked release build reused a copy-on-write, timestamp-preserved cache and rebuilt only the local raw-wgpu frontend. Binary SHA-256: `8fc1dc9005f858e33c5f28e2324b0b857bc597512f41b95e0d36b84dec1daa8c`.

The exact binary was packaged locally and opened through the supported desktop tool. Enter started play; held W/S moved the player paddle in opposite directions; Space showed Paused; Enter restarted at 0–0 Playing. A subsequent match reached 1–7 Over, and Enter restarted to 0–0 Playing. The window close control exited the pilot app according to desktop inventory. Screens were visually inspected, not saved. This is native desktop automation, not physical keyboard input or verification of a specific collision trajectory.

Direct shell launch with `--scripted-test --scripted-captures` produced no telemetry or captures after more than 15 seconds and was stopped (exit 130). System logs reported a connection-invalid error; the root cause was not established. No fresh scripted replay or app-owned capture pass is claimed. Normal app-bundle launch worked; historical evidence and binaries remain unchanged. No signing or clean-machine qualification is inferred.
