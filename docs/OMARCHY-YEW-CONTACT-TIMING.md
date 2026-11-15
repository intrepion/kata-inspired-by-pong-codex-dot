# Omarchy Yew native wrapper contact-timing replay

On 2026-11-13, Omarchy ARM64 built the Yew WASM frontend and its Tauri native wrapper from revision `6b9cf3807849ba9d45a02e1fc2a4172f902c464f` with locked, offline debug builds. The frontend used the matching locally cached wasm-bindgen CLI 0.2.130 and was embedded before building the shell. The native bridge's locked tests passed (2 tests); the shared web UI model's 11 tests were separately passed at the immediately preceding source revision `48fffc8` ([Dioxus evidence](OMARCHY-DIOXUS-CONTACT-TIMING.md)).

The native wrapper's bounded `--scripted-test --scripted-scenes` run exited successfully. Its 13 parsed telemetry records exactly matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six generated SVG sources (ticks 120, 240, 480, 600, 1320 and 1440) parsed as XML; score and status text was inspected, including the pause label at tick 480. Local outputs remain at `/tmp/pong-yew-c77-scenes-20261113/` and `/tmp/pong-yew-c77-output-20261113.txt`.

SHA-256 of the tested `experiments/yew/desktop/target/debug/pong-yew-desktop`: `3b27fc3cdc784889d470cb7c6d055f3cea950de7f96413630dada89955aca66c`. Embedded `experiments/yew/dist/app_bg.wasm`: `a2c99c3e2646e775caa843cd16c49bec2f5e382e964da3d9db877dac19c610c2`.

These are scripted model and generated SVG-source checks, not screenshots or proof of webview presentation. Real input, focus, resizing, complete gameplay, portable installation and release qualification were not checked. Earlier evidence remains historical.
