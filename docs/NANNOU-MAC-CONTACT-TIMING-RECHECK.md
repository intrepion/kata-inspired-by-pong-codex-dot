# Mac nannou contact timing recheck

macOS ARM64. Source checkpoint `5215d8adca4545b5edc9cbea201ad1b0a7ea808e` includes physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. Offline locked release build reused a copy-on-write, timestamp-preserved cache and rebuilt only the local nannou frontend. Binary SHA-256: `10071bfcff383a53be8bf92f277c3c8c0376e965d03f917a5b89fa01d43ca1da`.

The exact binary was packaged locally and opened through the supported desktop tool. The initial app-selection call timed out; desktop inventory showed the pilot running and a later selection exposed its Ready window. Enter started play; held W/S moved the player paddle in opposite directions; Space showed Paused in the title; R restarted at 0–0 Playing. A subsequent match reached 2–7 Over, and R restarted to 0–0 Playing. The window close control exited the pilot app according to desktop inventory. Screens were visually inspected, not saved. This is native desktop automation, not physical keyboard input or verification of a specific collision trajectory.

Direct shell launch with `--scripted-test --scripted-captures` exited 101 before replay because the renderer reported no GPU. No fresh scripted replay or app-owned capture pass is claimed. Normal app-bundle launch eventually worked; historical evidence and binaries remain unchanged. No signing or clean-machine qualification is inferred.
