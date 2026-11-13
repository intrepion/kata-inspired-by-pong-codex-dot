# Pixels court states: Mac cross-check (2026-11-13)

Source revision tested: `ce015a506bbaab4851fbaec995803f61ff01e612`; `experiments/pixels/src/main.rs` SHA-256 `26f3d4ae8bd18ee7fbfd431e15f83247904f7292a236905f1f54b586e208f788`.

On macOS, `cargo test --offline --locked --release --manifest-path experiments/pixels/Cargo.toml` passed 14 tests. These include the court score, state/winner, and backdrop checks.

A temporary local test fixture called the actual software `draw` function with forced Ready, Paused, player-win (7–3), and CPU-win (2–7) states. Inspection of the resulting 900×500 frames showed legible messages, the expected scores, and no ball overlap with the messages. The temporary test was removed and the source restored to the hash above. Local PNG fixture SHA-256 values:

- Ready: `aa0ddf83b05b518b4813fd1db594658d5e8614aa5b9f5e7503bf3627154ebf3b`
- Paused: `84c54a0a7fc1948fa8ba386f2fe16a9f195dcf21f16c2047a49a078aedebe6c1`
- Player wins: `d64d0b6b6597c51242e7899e3e0a2d598e30c332cca5f17bc44ce699d6adce53`
- CPU wins: `f489d532abfdaa38a9f2d0bc0f74571371ca4a0beadc12d84b3d1f45b4678535`

These are synthetic software-frame fixtures, not screenshots of a running native window or a scripted gameplay replay. They do not verify native input, real match progression, resizing, or release readiness. The fixture files remain locally under `/tmp/pong-pixels-state-mac-fixture-20261113-v1/`.
