# Pixels in-court score — Omarchy, 2026-11-13

The Pixels renderer kept scores only in the window title, which may be hidden by a window manager. It now draws each score as a seven-segment digit inside the court. A framebuffer regression for a 0–1 score failed before the change and passed afterward; all 12 locked offline adapter tests passed. Scores are bounded to a single digit for defensive rendering (normal matches end at seven).

A fresh locked debug build (SHA-256 `40fa9f29bd83e7b47856125fba7e906d5cae40c5ae65daaf7dbf5a825c595279`; source Git blob `33b2110c613510cf986d45b3083fbe4deaa193fe`) completed a local-display scripted replay with exit 0 and empty stderr. Its 13 JSON checkpoints matched the retained `/tmp/pong-pixels-c77-host-telemetry-20261113.jsonl` exactly. Six app-owned PNGs were produced; tick 480 was inspected and visibly shows 0–1, both paddles and the ball. Outputs remain in `../pong-pixels-score-check-20261113/`; earlier data was preserved.

This verifies a framebuffer and app-owned scripted capture, not physical input, full-match interaction or release qualification. In-court ready/paused and winner messaging remains a separate gap.

Update: the later [state-message follow-up](PIXELS-COURT-STATES-OMARCHY.md) addresses the in-court Ready, Paused and winner gap noted above; this score evidence remains historical.
