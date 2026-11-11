# Rust web-framework Pong experiments

These are isolated prototypes in the existing Pong repository, not separate game implementations. All three use `pong-core` physics and `web-ui-common` fixed-step input/presentation (120 Hz). SVG is deliberately shared so the comparison measures framework/WebView integration, not different games. It is not a native GPU-engine benchmark.

| Variant | Browser | Desktop route | Current status |
|---|---|---|---|
| Dioxus | Rust/WASM + Dioxus Web | Dioxus Desktop, native Rust and OS WebView | macOS WASM build + localhost Chrome controls passed; native pending |
| Leptos | CSR Rust/WASM | Tauri wrapper in `experiments/leptos/desktop` | macOS WASM + native wrapper builds passed; localhost Chrome controls passed; native gameplay pending |
| Yew | Rust/WASM | Tauri wrapper in `experiments/yew/desktop` | macOS WASM + native wrapper builds passed; localhost Chrome controls passed; native gameplay pending |

Dioxus Desktop is **not** wrapped in Tauri and is **not** Dioxus Native/Blitz. Leptos deliberately avoids SSR, authentication, and server infrastructure. Tauri wrappers grant no IPC capabilities. Comfy/rend3 are excluded as requested.

## Build and run

Install a matching official `wasm-bindgen-cli` for the resolved `wasm-bindgen` version. Run from repository root:

```sh
scripts/build-web-framework.sh yew  # also leptos or dioxus
python3 -m http.server 5191 --bind 127.0.0.1 --directory experiments/yew/dist
```

Open `http://127.0.0.1:5191`. Never test with a file URL. The helper supports an explicit `WASM_BINDGEN=/path/to/wasm-bindgen`. `index.html` in each source directory also supports Trunk. Build output stays ignored in `dist/`.

```sh
cargo run --manifest-path experiments/dioxus/Cargo.toml
# Build corresponding WASM dist first:
cargo run --manifest-path experiments/yew/desktop/Cargo.toml
cargo run --manifest-path experiments/leptos/desktop/Cargo.toml
cargo test --manifest-path experiments/web-ui-common/Cargo.toml
```

All manifests declare their own workspace and own lockfile to avoid interfering with the main four-client workspace. Native WebView prerequisites still apply (WebView2 on Windows, WebKitGTK development packages on Linux, system WebKit on macOS). Use `-j 1` on memory-constrained systems and do not run all builds concurrently.

## Controls and checks

W/S or arrow keys move; Enter starts/restarts; Space/Escape toggles pause. Buttons provide start, pause, and hold-to-move controls for pointer/touch use. Simulation seed is fixed at 42 for comparison. Timers measure real elapsed time, cap catch-up to 100 ms, and step at 120 Hz. SVG updates are approximately 60 Hz.

Validation checklist per platform: build, visible launch, start, movement/release, score/win/restart, pause/resume, focus-loss behavior, resize, pointer controls, shutdown. Build success is not gameplay verification.

## Mobile

Touch controls are included but mobile packaging/testing is not yet complete. Dioxus mobile needs its mobile feature/CLI setup; the desktop default does not constitute an iOS/Android build. Tauri mobile similarly needs generated platform projects and signing/emulator setup. Mac inspection found Xcode 27.2 and available iOS 18.4 simulators (iPhone 16 family), all shutdown. No iOS or Android runtime success is claimed.

## Verification log

- macOS: common controller tests 3/3 passed (fixed-step consistency, repeat-key restart protection, focus-loss input clearing).
- macOS Yew: Rust/WASM build passed. Native Chrome UI at `http://127.0.0.1:5191/yew/dist/` verified visible court, start, keyboard down movement, Space pause/resume, and pointer-held up movement. Screenshots confirmed paddle displacement and paused state. Full score/win/resize/focus-loss checks remain pending.
- macOS Leptos: WASM build and the same localhost keyboard/pointer movement, pause/resume, start checks passed at `/leptos/dist/`.
- macOS Dioxus: WASM build and same localhost keyboard/pointer movement, pause/resume, start checks passed at `/dioxus/dist/`. Added explicit `web-sys/Location` feature because Dioxus 0.7.10 history assumes it transitively.
- Follow-up (2026-11-11): Yew native wrapper initially failed because Tauri requires `icons/icon.png`, even with bundling disabled. Reused the existing Pong icon in Yew and Leptos wrappers. Both native macOS builds now pass using the existing Tauri target cache and one build job. Native launch/gameplay remains unverified; a compiled wrapper is not a gameplay pass.
- Dioxus native desktop build/launch remains pending.
- Windows and Omarchy: these new framework variants not yet checked.
- No commits, pushes, deployment, or release artifacts created.
