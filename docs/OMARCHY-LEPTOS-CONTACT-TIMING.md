# Omarchy Leptos native wrapper contact-timing replay

On 2026-11-13, Omarchy ARM64 built the Leptos WASM frontend and its Tauri native wrapper from revision `ebb5238ab8e99d30e88a2c6f2eab507aea4d07f6` with locked, offline debug builds. The frontend used the matching locally cached wasm-bindgen CLI 0.2.130 and was embedded before building the shell. The common native bridge's two locked tests passed in the preceding Yew qualification at source `6b9cf38` ([evidence](OMARCHY-YEW-CONTACT-TIMING.md)); the shared web UI model's 11 tests passed at `48fffc8` ([evidence](OMARCHY-DIOXUS-CONTACT-TIMING.md)).

The native wrapper's bounded `--scripted-test --scripted-scenes` run exited successfully. Its 13 parsed telemetry records exactly matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six generated SVG sources (ticks 120, 240, 480, 600, 1320 and 1440) parsed as XML; score and status text was inspected, including the pause label at tick 480. Local outputs remain at `/tmp/pong-leptos-c77-scenes-20261113/` and `/tmp/pong-leptos-c77-output-20261113.txt`.

SHA-256 of the tested `experiments/yew/desktop/target/debug/pong-leptos-desktop` (built with a shared target cache): `7c9d4f0c32f5ddf218d36c16f194ea04ec6899b0c3ca189b8d146324c905f43c`. Embedded `experiments/leptos/dist/app_bg.wasm`: `4ab8abf10d5367bb2918442e3750c6c9d5a4208fc0d06cde328811e032722242`.

These are scripted model and generated SVG-source checks, not screenshots or proof of webview presentation. Real input, focus, resizing, complete gameplay, portable installation and release qualification were not checked. Earlier evidence remains historical.
