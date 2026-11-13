# Ratatui winner header — Omarchy, 2026-11-13

The Ratatui header previously displayed only `Over` at match end. It now displays `YOU WIN` or `CPU WINS`, with `Match over` as a defensive tie fallback. Ready, Playing and Paused retain their labels. A TestBackend regression driving a winning point for each side failed before the change and passes afterward; it verifies the winner label at the minimum supported 50×18 terminal size and that it disappears after restart. All 11 locked offline adapter tests passed.

A fresh locked debug binary (SHA-256 `08eef90b2bfe487c7f6197b971718743b173c4e66c66f3e41d7b0fc453ba52a6`; source Git blob `320f577883caf7fc03357d952df68db4c322850c`) completed `--scripted-test --scripted-cells` with exit 0, empty stderr, six TestBackend cell exports and 13 telemetry checkpoints exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. The inspected tick-480 text shows the 0–1 score and Paused header. The scenario does not reach game over; the winner check is the separate controlled TestBackend regression. Outputs remain at `../pong-ratatui-winner-check-20261113/`; prior artifacts are preserved.

TestBackend cell buffers are not terminal screenshots or physical keyboard interaction. Actual complete-match rendering, terminal compatibility, installation and release qualification remain open.
