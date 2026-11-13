# ggez winner indication

2026-11-13. ggez previously showed only `Over - Enter to play` at match end. The status now says `YOU WIN - Enter to restart` or `CPU WINS - Enter to restart` based on the final score. Ready and paused messages retain their earlier wording; play has no status overlay. A tie-state fallback says `Match over - Enter to restart`.

On macOS ARM64, offline locked release-profile ggez tests passed 8/8. The new model test drives a winning point for each side, verifies the respective status, then restarts and verifies the overlay clears. A cached release build succeeded. Tested main.rs SHA-256 `69627cd5dfae5ab3ee6a376fc75f5c5307b9adc9dc5040b4564cbca3ab4850e7`; executable SHA-256 `f23c801fc5f5d4ea3f17a5704f364a8ea0f8f17204ecc2f6eb627580b73b9bac`. The source was based on checkpoint `9dfc79043046aea02702f14a906c2bbf6dd38c71`.

This is a model and build check. The rendered end screen, a full interactive match, physical input, clean-machine package and release qualification remain unverified. The pre-existing embedded font and all prior artifacts remain unchanged; the isolated source and test target remain under `/tmp/pong-mac-input-recheck-20261113-v1` and `/tmp/pong-mac-input-target-20261113-v1`.
