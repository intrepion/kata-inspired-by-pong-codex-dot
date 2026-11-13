# Ratatui small-terminal restart check on macOS — 2026-11-13

Cross-check of the small-terminal match-preservation fix in a running child-process PTY. This does not establish physical keyboard or terminal-emulator behavior.

An isolated, mixed local source tree used the exact Ratatui adapter blob `83ce9a5c6d689bab9f47188724e8603f23bbec55` and PTY harness blob `52424b51678f350402832e917207429e5e64ab9b` from checkpoint `6ace0a790fd551509798757be77c5b098dd8a110`. The local core source blob `8a9a1f2ecdcbfefde4d67e3f4df0d5f1d2930dd3` also matched that checkpoint. Other build inputs were not all compared, so this is not claimed as a complete build of that revision.

`CARGO_TARGET_DIR=/tmp/pong-ratatui-held-mac-target cargo test --offline --locked --release --manifest-path experiments/ratatui/Cargo.toml` passed 11 tests. The release executable built from the same tree had SHA-256 `10d3ac5d53fefe7fec6839727c6c77ad5a97446edc478466d91756739254766c`.

The repository harness's `--small-restart` scenario passed: after a nonzero score, synthetic PTY resize to 40×15 and Space/R inputs preserved that score; after resize to 100×30, R restarted at 0–0. Q while small exited successfully and the PTY terminal attributes matched their initial state. The harness SHA-256 was `3b78361e07b06aaa6e062a78e3214d8b96ae0e67028394603a807ca702bd7a07`; the preserved raw PTY stream at `/tmp/pong-ratatui-small-mac-20261113-v1/small.raw` has SHA-256 `f0cb89ae0c0a96fbe7b5ff9c911f4425c0f0e3a26614438cb937c8b581f95130`. Previous local source and binary were retained in the same evidence directory.

The harness injects terminal protocol bytes and SIGWINCH and uses a limited output parser. Real terminal-emulator resize, physical input, other terminal configurations, clean-machine installation and release qualification remain open.
