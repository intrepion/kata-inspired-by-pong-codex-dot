# Blue Engine zero-score marks — macOS, 2026-11-13

Mac native cross-check of the published Blue Engine zero-score change at `9e6dc14e`. All 13 offline locked release-profile tests passed, including the synthetic outline geometry check. The build emitted the existing future Rust compatibility warning for `block 0.1.6`.

The native window visibly showed two outlined zeros at Ready 0–0. After supported desktop automation sent Return, a solid player pip and CPU zero were visible at 1–0. Automated R then Space showed Paused at 0–0 with both zeros restored. The first app-selection call timed out and reset the tool session; inventory showed one candidate running, which was selected without a duplicate launch. The app then closed with Command-Q. These are visual observations with automated commands, not physical-input evidence.

The executable SHA-256 is `b79a57b67c8f53a030a51b78774ecb3cfd6ab2923ec813cdfd84aa5084fcd9c6`; adapter SHA-256 is `fca29607fcc6dacdc5e4755f6598181bfb0559a3b311aa9482cc6eeb9d0b1a3d`. The isolated mixed tree used adapter blob `d24364765c6ddd5a7699395a6211dcc9229e59b1` matching checkpoint `73d680442e71071d8632266549edab44f6709e89`; other build inputs were not all compared. Previous source and executable and the candidate app remain under `/tmp/pong-blue-zero-score-mac-20261113-v1`.

Other score combinations, sizes and platforms, physical input and release qualification remain unverified. The [Blue Mac license screen](BLUE-MAC-LICENSE-SCREEN.md) is not complete system-library or distribution clearance.
