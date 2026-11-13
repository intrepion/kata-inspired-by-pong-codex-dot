# Mac Blue Engine contact timing attempt

macOS ARM64. Source checkpoint `4bab2ae0541fb3d3e68026bf40bb1e03ca769890` includes physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. Offline locked windowed release build reused a copy-on-write, timestamp-preserved cache and rebuilt only the local Blue Engine frontend. Binary SHA-256: `95e53e1887f8b3b8c8bc960f5551fa51ce5e4b7c6334929df34242b772e68595`.

Direct shell launch with `--scripted-test` exited 1 before gameplay with `AdapterNotFound`: no Metal adapter was found in that launch context. No fresh scripted replay, app-owned capture, or native gameplay pass is claimed. The exact binary was packaged locally; the desktop app-selection call was interrupted before returning a window, so its launch and process outcome are unconfirmed. Historical evidence and binaries remain unchanged. No physical-input, signing, or clean-machine qualification is inferred.

## Desktop retry

The previously interrupted process was absent from the desktop app inventory and process list before retry. The same packaged binary opened on the supported macOS desktop interface. Automated key input started play, moved the left paddle upward, paused at 0–1, resumed, and restarted to 0–0; the window subsequently showed play continuing to 0–5. The test app was closed. This is an automated desktop interaction check, not physical input, a complete match, or a scripted replay.

## Full-match desktop check

On a separate run of the same packaged binary (SHA-256 above), supported desktop automation pressed Enter at 0–0, then left the player paddle unattended. The visible score progressed through 2–3 and 3–5 to **3–7, Over**. Pressing R then showed **0–0, Playing**. The test app was closed. This checks an automated native match and post-match restart; it is not physical input or a scripted model replay.
