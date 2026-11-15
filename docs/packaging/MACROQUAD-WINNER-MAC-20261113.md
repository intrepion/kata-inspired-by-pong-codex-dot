# Macroquad winner indication

2026-11-13. Macroquad previously displayed only `Over - Enter to play` at match end. It now displays `YOU WIN - Enter to restart` or `CPU WINS - Enter to restart` according to the final score. The Ready and Paused messages retain their wording, Playing has no overlay, and an impossible tied Over state has a neutral fallback.

On macOS ARM64, the offline locked release-profile Macroquad suite passed 6/6. The new model regression drives a winning point for each side, checks the corresponding message, restarts and checks that the overlay clears. The cached release build succeeded. Tested main.rs SHA-256 `588fe3e3c7cc067616e07524bed0249503e68903c33cbd63bb88baf73d950bf5`; executable SHA-256 `6c09ca62b792b7cafd13e970015d8a74cd736bf1b0f5d288fa0ce0cd1d4bf363`. The source was based on checkpoint `5095cb6d2475bbc4c522784a1514084eeb1a149b`.

This verifies message selection and compilation, not the rendered end screen, a full interactive match, physical input or release qualification. Prior artifacts were retained; isolated source and target remain under `/tmp/pong-mac-input-recheck-20261113-v1` and `/tmp/pong-mac-input-target-20261113-v1`.
