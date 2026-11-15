# bracket-lib winner header — Omarchy, 2026-11-13

The bracket-lib in-window header previously displayed only `Over` at match end. It now names `YOU WIN` or `CPU WINS`, with `Match over` as a defensive tie fallback; Ready, Playing and Paused retain their wording. A regression driving a winning point for each side failed before the change and passes afterward, including restart. All 15 locked offline adapter tests passed, including the existing fixed-width header test.

A fresh locked debug binary (SHA-256 `ec8a74c8ef0782c7f72d35ad7d6e5b3fc2cb480c134a6807ddf2c424d73e3eac`; source Git blob `dd0b975487a1cd81130abfe009b1ddd224a2704d`) completed a local-display scripted replay: exit 0, six app-owned framework captures and 13 checkpoints exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Its stderr contained only the documented process-local X11/XWayland fallback notice. The inspected tick-480 capture shows the paused header, 0–1 score, court, paddles and ball. The scenario does not reach game over, so winner text was not visually verified. Outputs remain at `../pong-bracket-winner-check-20261113/`; prior artifacts are unchanged.

These are header-function tests and scripted rendering, not physical input, full-match interaction, native Wayland qualification, clean-machine packaging or release qualification.
