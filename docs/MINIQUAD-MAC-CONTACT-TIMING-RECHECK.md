# Mac Miniquad contact timing recheck

2026-11-12, macOS ARM64. Source checkpoint `4f4da1dc2fa82025786383dc1ea0ca38ba23ff7a` includes physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. Offline locked release build reused a timestamp-preserved copy of the existing Miniquad dependency cache and recompiled only the four local Pong crates. Binary SHA-256: `48daa212b14011ff2c01c291bb7e08d01f3dc8d5cb8163c184cebd3b18fc4b02`.

The exact binary was packaged locally and opened through the supported desktop tool. Enter started ball motion; held W and S moved the left paddle in opposite directions. Space displayed the pause symbol, and the ball and paddle stayed fixed while W was held for two seconds. R reset the paddle to center and resumed ball motion. The app was closed with Command-Q, and desktop inventory no longer listed the pilot as running. Screens were visually inspected, not saved. This is bounded native automation, not physical input or specific collision-trajectory verification.

Direct shell launch with `--scripted-test --scripted-captures` failed before replay (exit 101) because the Miniquad macOS OpenGL pixel-format assertion failed. No scripted replay or app-owned capture pass is claimed for this build. Earlier evidence and binaries remain historical; no release signing or clean-machine qualification is inferred.

## Full-match desktop check

In a separate run of the same packaged binary (SHA-256 above), supported desktop automation pressed Enter and left the player paddle unattended. The visible score pips reached three for the player and seven for the CPU; the court remained static on a subsequent observation. Enter cleared the pips and ball motion resumed. The app was closed. The font-free court did not show an explicit game-over or winner label in the observed end state, a usability gap. This is automated native interaction, not physical input or a scripted replay.

## Winner-label fix

The Miniquad renderer now draws font-free `YOU WIN` or `CPU WINS` and `ENTER TO RESTART` when the shared game state is Over. Its streamed vertex and index buffers were enlarged for the extra glyph geometry. On Mac ARM64, an offline locked cached release build and all eight Miniquad release tests passed. The tested source was based on `61948a24c2ba99f12999bc7344c494b0d7c93d9c`, with Miniquad source blob `86bba1c9f9dd337fe68352dfe6d2c105642fcc6d`; executable SHA-256 `ae446059f349fa64ec2f5d70751a75f26192bc4aac78450b2fd6892126e54052`.

In a supported desktop-automation match with the player paddle unattended, the CPU reached seven and the court visibly displayed `CPU WINS` and `ENTER TO RESTART`. Enter cleared the text and resumed play. The app exited. The unit test covers both winner branches and removal on restart; only the CPU-win branch was visually inspected. This is not physical-input, scripted-replay, Windows input, or release qualification. The earlier failed direct-shell replay remains historical.
