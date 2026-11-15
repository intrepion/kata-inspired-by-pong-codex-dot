# Nannou focus-action gate: Mac cross-check (2026-11-13)

Nannou source blob `668ff59e2355b7e3e22a24689aef153bdaa82ead` from `92e8fc1d3a`, SHA-256 `8ccca95f94d2a5b677b66a7bad5583de04bb5292f8a4a41fa8dca88a4e0f3285`, was built offline and locked using the cached Mac release target. The executable SHA-256 was `7e16fbd60780c53e5c64d8558f37f4d7587a00702544f8a5f4895a2b2473930f`. All 14 adapter tests passed, including the synthetic unfocused action gate.

In the native Mac window, 500 ms desktop-automation presses of Return, Space, Space, and Escape produced Playing, Paused, Playing, and app exit respectively. This checks focused actions in this run; it does not exercise a confirmed native focus loss, an action queued during focus loss, or a key reported as newly pressed at focus gain. Physical input, full gameplay and release qualification remain open. The app and prior binary are retained under `/tmp/pong-nannou-focus-native-mac-20261113-v1/`.
