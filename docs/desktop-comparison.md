# Pong: four frontend experiment

The original root Canvas game remains available unchanged. Rust versions live alongside it, not in four diverging repositories.

| Frontend | Rendering and packaging | Shared gameplay |
| --- | --- | --- |
| Macroquad | lightweight native game window | pong-core |
| Bevy | native ECS/rendering engine | pong-core |
| WebAssembly | browser Canvas, Rust WASM simulation | pong-core via pong-wasm |
| Tauri | OS webview desktop wrapper for WASM frontend | same WASM as browser |

Tauri is a desktop shell, not a game engine. WASM is a compilation target, not a fourth engine. Keeping these distinctions makes this a useful comparison of native rendering versus browser/webview rendering.

## Prerequisites

Stable Rust (Bevy currently requires Rust 1.97.1 or later), Node for tests, Python 3 for a local server. Add `wasm32-unknown-unknown` with rustup. macOS needs Xcode Command Line Tools. Windows needs MSVC C++ Build Tools, Windows SDK and WebView2; use `cargo +stable-x86_64-pc-windows-msvc` if GNU is your default. Linux needs compiler/linker and graphics development packages; Tauri additionally needs GTK3/WebKitGTK 4.1 development packages.

## Build and run

From the repository root:

```sh
cargo test -p pong-core -p pong-wasm
node --test tests/game.test.js
cargo run -p pong-macroquad
cargo run -p pong-bevy
sh scripts/build-wasm.sh
node --test tests/wasm.test.mjs
cargo run -p pong-tauri
```

On Windows use `powershell -File scripts/build-wasm.ps1` (and select MSVC with `RUSTUP_TOOLCHAIN=stable-x86_64-pc-windows-msvc` for the process if needed). Tauri must be built after the WASM build; it embeds that frontend at compile time. No Node dependency installer, wasm-bindgen, Trunk, or Tauri CLI is necessary.

For the browser version:

```sh
python3 -m http.server 5188 --bind 127.0.0.1 --directory clients/web-wasm
```

Visit http://127.0.0.1:5188. Always test through a server, never file URLs.

## Controls and parity

W/S or arrows move; Enter starts/resumes, Space pauses, R restarts. Browser/Tauri also have buttons. Court is 900×500, first to seven, angle-sensitive paddle hits, limited and delayed CPU. All four Rust frontends use 120 Hz simulation steps with bounded catch-up. Shared seeded simulation tests cover movement, collisions, scoring, pause, CPU and repeatability. The original JavaScript game is retained as a baseline; its randomness and variable timing are not byte-for-byte replay compatible with Rust.

## Scope

This is a local development experiment, not signed installers or a published release. Builds and actual per-platform runtime checks are recorded separately; successful compilation alone does not prove a working graphics/window/audio environment. Do not replace the published web deployment until approved.

## Verification log (2026-11-11)

- Shared core: 9 Rust tests pass.
- Existing JavaScript: 6 tests pass without changing the original simulation.
- Compiled WASM ABI: 2 Node tests pass (state/input/pause/restart and deterministic independent-instance replay).
- Mac: all three native clients compile and all Rust packages pass `clippy -- -D warnings`. Bevy initialized Apple M1 Metal; interactive start, move, pause, restart verified. Tauri embedded WASM start, movement, scoring and pause verified.
- Mac browser WASM via localhost: start, paddle movement, pause/restart and focus-away pause pass. Macroquad start, movement, pause and restart pass; focus-loss behavior needs further verification. Browser console inspection was unavailable through the native browser surface.
- Omarchy ARM64: all three native clients build, launch and appear as mapped Hyprland windows. Bevy initializes Vulkan on Apple M1 Pro/Honeykrisp; Macroquad/Tauri logs clean. The same 17 tests pass and localhost serves the rebuilt WASM. Interactive visual/input QA remains pending because the automation connection exposes no apps/browser surfaces.
- Windows: all three native clients compile and 17 tests pass; WASM rebuilt locally. Chrome localhost and Tauri rendering/start/scoring/pause verified. Macroquad renders but automated keyboard input remains unverified. Bevy default Vulkan exits on Intel UHD 630; process-only DX12 accepts input but renders sprites/court incorrectly, so Windows Bevy gameplay is not a pass. Tauri requires `clients/tauri/icons/icon.ico` (added on Windows by losslessly wrapping the existing PNG).

These are development builds, not benchmark results. Different compiler settings, renderers and webviews make raw debug executable sizes unsuitable for performance comparisons.
