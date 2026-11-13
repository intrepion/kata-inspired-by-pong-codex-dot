# Mac Softbuffer contact timing recheck

macOS ARM64. Source checkpoint `f40c9a503168991facfec091e58dcdddd4eb133f` includes physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. Offline locked release build reused a copy-on-write, timestamp-preserved cache and rebuilt the local low-level common and Softbuffer crates. Binary SHA-256: `9d01dcf934024951474cdc4874b9e2e49cfa991eb76d2042d54cefe1e19974ba`.

The exact binary was packaged locally and opened through the supported desktop tool. Space started play; held W/S moved the player paddle in opposite directions; Space showed Paused; R restarted at 0–0 Playing. A subsequent match reached 1–7 Over, and R restarted to 0–0 Playing. Escape exited the pilot app according to desktop inventory. Screens were visually inspected, not saved. This is native desktop automation, not physical keyboard input or verification of a specific collision trajectory.

Direct shell launch with `--scripted-test --scripted-captures` produced no telemetry or captures after more than 15 seconds and was stopped (exit 130). System logs reported a connection-invalid error; the root cause was not established. No fresh scripted replay or app-owned capture pass is claimed. Normal app-bundle launch worked; historical evidence and binaries remain unchanged. No signing or clean-machine qualification is inferred.
