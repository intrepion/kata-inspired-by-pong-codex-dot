# Shared low-level Ready indicator

2026-11-13. The shared rectangle model used by SDL3, ash and softbuffer previously showed pause bars in both Ready and Paused states. Ready now shows a right-pointing play symbol; Paused keeps the pause bars, and the existing winner indication is unchanged. No new asset or dependency was added.

On macOS ARM64, the shared offline locked release-profile suite passed 5/5, including a new test distinguishing Ready and Paused geometry. Dependent SDL3, ash and softbuffer suites passed 10, 11 and 9 tests respectively. Shared lib.rs SHA-256 `b6f2e6ec1de9759e7af7759f45171da1c66ddd07315172d0e424c87f542e0947`; source was based on checkpoint `0aa0f67230e95670ef79751921199a445d0cff4b` and checked unchanged against `0a71fd25e6bd7e476070cd9d3c282f8b45cd0d39` before publication.

Two software-raster fixtures at 900×500 were visually inspected. The Ready fixture displayed the play symbol; the Paused fixture displayed two bars. PNG SHA-256: Ready `3a1780f7f8b9fa01940827a52290e04dd7ee6e3d97d529dc1052fe2f65290a30`; Paused `c51c1c0ac0c9b7f269bbe2c95578a82a2ec7a58c62460afeca7311fbd98b1b5e`. They remain under `/tmp/pong-lowlevel-ready-preview-20261113-v1`. These are software-raster fixtures, not native window captures; full gameplay, physical input and release qualification remain open. Prior artifacts were preserved.
