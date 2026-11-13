# raylib-rs winner indication

2026-11-13. raylib-rs showed the score and an `Over` state but did not name the winner. It now displays `YOU WIN - ENTER OR R TO RESTART` or `CPU WINS - ENTER OR R TO RESTART` in the court; a tied Over state has a neutral fallback.

On macOS ARM64, offline locked release-profile tests passed 11/11. The new model regression drives a winning point for each side, checks the message, restarts and checks that it clears. The cached release build succeeded. Tested main.rs SHA-256 `725d2f2f958b74064c3eb1ba0078c8ca1fdf87fa447c91702dcb1f36e66c354f`; executable SHA-256 `aa3f471ebd60ff4fb1c3fdc4ac510e867aae2ef329b79843659143d2def717a3`. The source was based on checkpoint `9c0fd73ef711fb004c071d784dde08a2672bb2ef`.

This verifies message selection and compilation, not the rendered end screen, a full interactive match, physical input or release qualification. Prior artifacts remain intact; isolated source and target are under `/tmp/pong-mac-input-recheck-20261113-v1` and `/tmp/pong-mac-input-target-20261113-v1`.
