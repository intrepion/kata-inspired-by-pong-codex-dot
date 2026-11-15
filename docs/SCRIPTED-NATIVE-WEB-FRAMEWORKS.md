# Native Yew/Leptos shared replay and SVG-source evidence

Both Tauri wrappers now reuse `pong-native-web-demo`. Their existing browser code and shared `pong-web-ui-common::Session` are unchanged. Build the matching frontend first (wasm-bindgen CLI must match the locked Rust version; this run used official 0.2.130):

```sh
bash scripts/build-web-framework.sh yew
cargo run --locked --manifest-path experiments/yew/desktop/Cargo.toml -- --scripted-test --scripted-scenes /absolute/new-yew-scenes
# Substitute leptos in both commands for that adapter.
```

An opt-in local Tauri page hook selects `?scripted-test` and waits for the existing seven app-generated download links. A single scoped command receives their decoded contents, validates all thirteen states against the shared Rust Scenario and validates exact allowed filenames/checkpoint order. It writes only fixed filenames in a new CLI-selected directory, using create-new semantics and an 8 MiB input cap. It then exits. Without a scene directory it prints verified telemetry and exits. Normal play disables the command/hook. No general filesystem API, remote-origin permission or plugin capability is added; existing CSP remains unchanged.

`--scripted-captures` is rejected: these files are **SVG scene source, not PNG screenshots**. No OS capture/input workaround or raster conversion is used. The outputs verify generated geometry, not actual webview pixel presentation, font rendering, compositor behavior, real keyboard/focus, resize or a full match. They exclude surrounding framework DOM.

## Verified on Omarchy ARM64 / WebKitGTK 2.54.2

Both frontends and native shells build. Locked native checks and strict Clippy pass; bridge two validation tests and four existing common tests pass. Each bounded native run exits 0 with six SVGs and thirteen byte-identical Macroquad baseline states through tick 1440. XML inspection confirms every scene's circle coordinates, player-paddle position and scores agree with its state record. No visual screenshot pass is claimed. Existing platform pins are preserved, including core-foundation 0.10.1.

[Hashes and geometry validation](evidence/native-yew-leptos-scripted-20261112.json) · [Yew telemetry](evidence/omarchy-native-yew-scripted-20261112.jsonl) · [Leptos telemetry](evidence/omarchy-native-leptos-scripted-20261112.jsonl)

Native Dioxus has a [separate verified integration](SCRIPTED-DIOXUS-NATIVE.md); it is not covered by the Yew/Leptos run results above.
