# Omarchy Tauri contact-timing replay

2026-11-13: On Omarchy, source revision `6870864f485153e00d5fce2948d73f791ce62802` (including contact-timing physics `c77bf3a`) was built with locked, offline release builds of `pong-wasm` for `wasm32-unknown-unknown` and `pong-tauri`. The resulting WASM was copied into the web client before building the shell. `cargo test --offline --locked --release -p pong-tauri` passed two tests; `node --test tests/wasm.test.mjs` passed its WASM test file.

The native shell's `--scripted-test --scripted-captures` run exited successfully. Its 13 parsed telemetry records exactly matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. It exported six app-owned Canvas PNGs at ticks 120, 240, 480, 600, 1320 and 1440. The tick 480 image was inspected and shows both paddles, ball, center line and an unobstructed pause label. Local output is retained at `/tmp/pong-tauri-c77-captures-20261113/` and `/tmp/pong-tauri-c77-output-20261113.txt`.

SHA-256 of the tested `target/release/pong-tauri`: `c69fefd716dd384ae2b2677ed0c0874fcfba39f5b2fe94638d904ce2b9c11bfd`. SHA-256 of embedded `clients/web-wasm/pong_wasm.wasm`: `1f45dc2d99f117c1bede2d243ad34dac289e20d6d2d14e98216c6e4223adcda3`.

This is an automated native-shell replay and app-owned canvas export, not a desktop screenshot or physical input test. The export excludes DOM scores and the rest of the webview. Real input, focus, resizing, a full match, portable installation and release qualification remain unverified by this run. Earlier evidence remains historical.
