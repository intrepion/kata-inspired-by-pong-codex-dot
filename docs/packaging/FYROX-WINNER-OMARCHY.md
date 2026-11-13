# Fyrox winner status — Omarchy, 2026-11-13

Fyrox's in-window header previously showed only `Over` at match end. It now shows `YOU WIN` or `CPU WINS`, with `Match over` as a defensive tied-state fallback. Ready, Playing and Paused retain their previous labels. A test driving winning points for both sides failed before the fix and passes afterward, including restart. All 9 locked offline adapter tests passed.

A fresh locked debug binary (SHA-256 `7e0f9ea72966503a176ed526e795b0d749bc1c1af79c68ee9757c91a26a6c6bd`; source Git blob `82d2c4c317a96c76e6e396aad624876c543f833c`) completed a local-display scripted replay: exit 0, empty stderr, six app-owned PNGs and 13 checkpoints exactly matching `/tmp/pong-fyrox-c77-host-telemetry-20261113.jsonl`. The inspected tick-480 capture shows the in-window header, score, pause status, court, paddles and ball. It does not reach game over, so the winner text has not been visually verified. Outputs remain under `../pong-fyrox-winner-check-20261113/`; historical artifacts are preserved.

This is a status-function test and app-owned scripted capture, not a physical-input, full-match, installed-package or release qualification.
