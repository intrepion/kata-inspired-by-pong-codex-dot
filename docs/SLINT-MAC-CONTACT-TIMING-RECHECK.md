# Mac Slint contact-timing recheck

macOS ARM64, source checkpoint `9195b633bf860417c53868f2322f437796921309`, including shared physics fix `c77bf3ac147e41f67d88962f5c615fcc113d6465`. An offline locked debug build reused a copy-on-write dependency cache and compiled only local Pong crates. Executable SHA-256: `02687007762a873e03ea6013361b11f85f453ca51508bf79851f524f9edf935a`.

A direct-shell `--scripted-test --scripted-captures` attempt timed out after 35 seconds and its capture directory was empty. The process was stopped by the timeout. This is not a scripted replay or app-owned capture pass.

The same binary was placed in a locally assembled test app bundle and opened through the supported desktop interface. Automated Enter started play; held W moved the paddle upward; Space showed Paused; Enter resumed; scoring reached 0–2; R followed by Space showed 0–0 Paused with the paddle centered. The test app exited. This is bounded automated native interaction, not a complete match, physical input, a distribution package, signing, or clean-machine qualification. Earlier Slint evidence remains historical.
