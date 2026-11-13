# Piston winner title

2026-11-13. Piston renders score bars without bundled text and previously showed only `Over` in its window title. The title now identifies `YOU WIN - Enter to restart` or `CPU WINS - Enter to restart` before the score and controls; a tied Over state uses a neutral fallback. This uses the existing title surface and introduces no font dependency.

On macOS ARM64, offline locked release-profile tests passed 13/13. The new model regression drives a winning point for each side, checks the title, restarts and checks that the winner message clears. The cached release build succeeded. Tested main.rs SHA-256 `14ca4c43ce651b8b13db1d8415db3c697f2ee7fbbb7821d7b5d28183fbd23fae`; executable SHA-256 `e558902fa131443cbf4e7ee080ea6868d42243273da15bde351fa864c0526fe0`. The source was based on checkpoint `8981a8df8881ad8de367938862efab07117746f9`.

This verifies title selection and compilation, not actual title visibility or a full interactive match. The title may not be visible in fullscreen; no in-court winner text, physical-input or release qualification is claimed. Prior artifacts remain intact; isolated source and target are under `/tmp/pong-mac-input-recheck-20261113-v1` and `/tmp/pong-mac-input-target-20261113-v1`.

## In-court winner follow-up

Piston now also draws `YOU WIN` or `CPU WINS` inside the court using built-in rectangle glyphs; no font or dependency was added. This addresses the title-only limitation in fullscreen. The existing model test drives both winning scores, checks the label and nonempty glyph geometry, then verifies restart clears it. On macOS ARM64 the offline locked release-profile suite passed 13/13 and the cached release build succeeded. Tested main.rs SHA-256 `4412f47f6bbf5b077a9e3f935e31068cc660482859f4ad6348e59af0d84b5b13`; executable SHA-256 `27c8f11533838ebd3b68839719fba86dca0d66e12e1823f0d738eabef6f6463e`. The source was based on `605e55b6a82a95fe84834e1651e73055c68bb0b1` and checked unchanged against `84731994b80c217b538d2a2e6849fc5d9c93dc9e` before publication.

This validates model selection, geometry generation and compilation, not an actual GPU-rendered window or full match. The rendered label, physical input and release qualification remain unverified. Prior artifacts were preserved.
