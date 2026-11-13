# Piston winner title

2026-11-13. Piston renders score bars without bundled text and previously showed only `Over` in its window title. The title now identifies `YOU WIN - Enter to restart` or `CPU WINS - Enter to restart` before the score and controls; a tied Over state uses a neutral fallback. This uses the existing title surface and introduces no font dependency.

On macOS ARM64, offline locked release-profile tests passed 13/13. The new model regression drives a winning point for each side, checks the title, restarts and checks that the winner message clears. The cached release build succeeded. Tested main.rs SHA-256 `14ca4c43ce651b8b13db1d8415db3c697f2ee7fbbb7821d7b5d28183fbd23fae`; executable SHA-256 `e558902fa131443cbf4e7ee080ea6868d42243273da15bde351fa864c0526fe0`. The source was based on checkpoint `8981a8df8881ad8de367938862efab07117746f9`.

This verifies title selection and compilation, not actual title visibility or a full interactive match. The title may not be visible in fullscreen; no in-court winner text, physical-input or release qualification is claimed. Prior artifacts remain intact; isolated source and target are under `/tmp/pong-mac-input-recheck-20261113-v1` and `/tmp/pong-mac-input-target-20261113-v1`.
