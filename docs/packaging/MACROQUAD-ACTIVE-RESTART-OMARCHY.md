# Macroquad active-restart cross-check on Omarchy

2026-11-13: checked the Macroquad restart change at `758261ed6b27d2356ad4dcbce194af2fbcf2aacd`, contained in source checkpoint `c880abb491e4c113c8db5cd1186a62dffd44c618`. (The requested `3586d99` is a later macOS evidence commit for Fyrox, Nannou and raylib, not the Macroquad code change.) Macroquad source Git blob: `c7f07f218656d7da29b7dc21cccb2c1983a3d40b`.

`cargo test --offline --locked -p pong-macroquad` passed all 5 tests, including the active-restart held-key gate regression. A fresh locked offline debug build passed; executable SHA-256 `0f4eb760fdddfa6c2fa6a177be239f19ea8e6f2e2b0aefe924adf704e65bbd00`.

The debug executable ran on the local display with `--scripted-test --scripted-captures`: exit 0, empty stderr, 13 telemetry lines and six app-owned PNGs. Telemetry exactly matched the retained `/tmp/pong-macroquad-c77-host-telemetry-20261113.jsonl` replay. The tick-600 image was inspected and displayed the court, paddles, ball and 0–1 score. New outputs are retained at `../pong-macroquad-restart-omarchy-20261113/`; older artifacts were not replaced.

The unit test exercises the movement gate and replay exercises application-owned scripted input and rendering. Neither verifies physical key input through restart, full-match play, an installed package, or release readiness.
