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
- Omarchy: these new framework variants not yet checked. Windows browser follow-up is below.
- The initial local prototype was subsequently committed and pushed in the non-main comparison checkpoint; no release artifacts or deployment are implied.

## Windows browser validation (2026-11-11)

Checkpoint `17dcc09` was fetched into a separate Windows Git worktree, leaving the pre-existing dirty checkout untouched. All three locked WASM builds passed using the stable MSVC toolchain and official wasm-bindgen 0.2.130. The published SHA-256 of the downloaded Windows wasm-bindgen archive was verified. No Library helper workaround was used.

PowerShell equivalent (install the WASM target and matching wasm-bindgen first):

```powershell
$env:RUSTUP_TOOLCHAIN = 'stable-x86_64-pc-windows-msvc'
# Optional: $env:WASM_BINDGEN = 'C:\path\to\wasm-bindgen.exe'
.\scripts\build-web-framework.ps1 yew # also leptos or dioxus
python -m http.server 5191 --bind 127.0.0.1 --directory experiments
```

Open `http://127.0.0.1:5191/yew/dist/` (or the matching framework path). The script uses two build jobs, locked dependencies, and an ignored target directory by default. It does not install tools or change system settings.

Verified in connected Windows Chrome through the supported browser interface:

| Check | Yew | Leptos | Dioxus |
|---|---|---|---|
| Visible court, start, scoring/ball movement | Passed | Passed | Passed |
| Pointer-held paddle movement | Passed | Passed | Passed |
| Keyboard paddle movement | Passed | Passed | Passed |
| Pause/resume | Passed | Passed (button and Space) | Passed (button and Space) |
| CPU reaches seven / win message | Passed | Passed | Passed |
| Restart clears scores | Passed | Passed | Passed |

Shared controller tests also passed 3/3 on Windows: fixed-step consistency, repeated-key restart protection, and focus-loss input clearing. These unit tests do not substitute for browser focus-loss validation. Browser resize, focus-loss, touch/mobile packaging, and native desktop wrappers remain unverified on Windows. The running court replaces its SVG repeatedly; one semantic court-click attempt returned a stale-node error, but keyboard testing via the existing focused controls worked. This was not a browser crash.

- Windows responsive follow-up: all three variants passed visible layout and pointer-control checks at an explicit 390x844 viewport, with document scroll width equal to viewport width (no horizontal overflow). Default viewport restored afterward. This is browser responsive testing, not a native mobile or physical-touch-device pass.
- Windows native wrapper follow-up: Yew initially failed because tauri-build requires `icons/icon.ico` for Windows resources. Added ICO containers of the existing 128px Pong PNG to both wrappers. Yew and Leptos now build successfully with the stable MSVC toolchain. The supported Computer Use launch request for Pong Yew timed out awaiting approval, so native visible launch/gameplay is still unverified, not passed.

- Windows Dioxus Desktop follow-up: locked native MSVC build passed (5m05s first build in this target cache). Visible launch/gameplay has not yet been attempted; this is not a native UI pass.
