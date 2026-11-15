# raylib-rs winner status cross-check — Omarchy, 2026-11-13

The independent change `613be89cfa99820dda3b084705f6cb604934cb5f` adds a central winner message. At source Git blob `8e9a7da9b17dcf9373e2b49ce648926bc2f5c1a4`, `cargo test --offline --locked --manifest-path experiments/raylib-rs/Cargo.toml` passed all 11 tests, including winning points for both sides and clearing the message on restart. A fresh locked debug build passed; executable SHA-256 `ca5b4b75bf9ea4d38a79dbe4a9b4f238322b77f91c402f2b4974473054016f14`.

That executable ran on the local display with `--scripted-test --scripted-captures`: exit 0, empty stderr, 13 JSON checkpoints exactly matching the retained `/tmp/pong-raylib-c77-host-telemetry-20261113.jsonl`, and six app-owned PNGs. The inspected tick-480 capture shows the court and pause status; the status line overlaps the left paddle at its bottom position, a pre-existing layout issue. The scenario never reaches game over, so neither winner message was visually verified. Outputs remain under `../pong-raylib-winner-mac-crosscheck-20261113/`. An earlier independently developed overlapping approach and its replay remain preserved at `../pong-raylib-winner-check-20261113/`; they are not the published implementation.

These are status-function tests and app-owned scripted rendering, not physical input, full-match play, installed-package or release qualification.
