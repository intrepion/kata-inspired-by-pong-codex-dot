# Rallylight

An original real-time paddle game inspired by Pong. Return the ball, steer shots with your paddle, and beat the computer to seven points. The computer has a short reaction delay and a limited paddle speed, so well-placed shots can get past it. This independent kata is not affiliated with or endorsed by Atari.

## Play

[Play Rallylight online](https://intrepion.github.io/kata-inspired-by-pong-codex-dot/).

Run `python3 -m http.server 8000 --bind 127.0.0.1` and visit `http://127.0.0.1:8000`. Do not open the HTML file directly. No dependencies or build step are needed for the original game.

Move with the up/down arrows or W/S. Press Space or use the Pause button to pause. On a touch screen, hold the arrow buttons or drag on the court. The game pauses when the tab is hidden or the window loses focus. Hit the ball near the edge of your paddle to change its angle.

## Test

Run `node --test tests/game.test.js`.

## License

MIT; see [LICENSE](LICENSE). The root `.gitignore` combines GitHub's global Linux, macOS, and Windows templates.

## Arcade metadata

See [docs/arcade-metadata.md](docs/arcade-metadata.md) for the game-owned manifest and screenshot provenance.

## Rust / desktop comparison

Four frontends share a Rust game simulation. See [the comparison guide](docs/desktop-comparison.md) for architecture, commands, and verification. The original web game remains unchanged.

## Implementation testing dashboard

Updated **2026-11-11** on the experimental `codex/pong-comparison-checkpoint-20261111` branch. This is a testing checkpoint, **not a release certification**. The original hosted game is unchanged; compiled downloads/installers are not published yet.

Each desktop cell is **build / runtime evidence**:

- **B**: full development build passed; **C**: check only; **U**: not yet verified.
- **I**: bounded visible interaction passed, not the entire acceptance checklist.
- **S**: process startup only; **R**: rendered, gameplay not verified.
- **F**: observed failure; **blocked**: a concrete tool or prerequisite prevents the next check.
- **n/a**: not a target for that implementation. Unit tests do not count as gameplay.

### Core frontends and game engines

| Implementation | Mac build / runtime | Windows build / runtime | Omarchy ARM64 build / runtime |
| --- | --- | --- | --- |
| Original JavaScript browser game | No build required; existing 6 logic tests pass; no new all-platform UI certification | Same baseline | Same baseline |
| Rust WASM + Canvas browser | B / I | B / I | B / localhost serves; UI blocked |
| Macroquad | B / I | B / R; keyboard play unverified | B / S; UI blocked |
| Bevy | B / I | B / **F**: Vulkan exits; DX12 renders incorrectly | B / S; UI blocked |
| Tauri + Rust WASM | B / I | B / I | B / S; UI blocked |
| Miniquad | B / I | B / R; start input unsuccessful | B / S; UI blocked |
| pixels + winit | B / I | B / I | B / S; UI blocked |
| raw wgpu + winit | B / I | B / I | B / S; UI blocked |
| Piston | B / I | B / I | B / S; UI blocked |
| ggez | B / I; Mac surface fix verified | B / I on earlier snapshot | B / S; UI blocked |
| nannou | B / I before pacing follow-up | B / I | B / S; UI blocked |
| Fyrox | B / I | B / I | B / S; UI blocked |
| Blue Engine | B / I | B / I | B / S; UI blocked |
| raylib-rs | B / I | B / R; start input unsuccessful; LLVM build blocker resolved | B / S; UI blocked |
| Godot + Rust GDExtension | B / I for game; **F** in headless editor shutdown | U / U | U / U |
| bracket-lib (RLTK) | B / I; restart/exit incomplete | B / I; separate header improves readability, bitmap quality still rough | B / S on X11 and patched Wayland; UI blocked |

Details: [baseline frontends](docs/desktop-comparison.md), [extra engines](experiments/ENGINE-STATUS.md), [ggez fix](experiments/ggez/README.md), [Godot](experiments/godot/VERIFICATION.md), [bracket-lib](experiments/bracket-lib/README.md).

### Low-level rendering and native GUI/terminal clients

| Implementation | Mac build / runtime | Windows build / runtime | Omarchy ARM64 build / runtime |
| --- | --- | --- | --- |
| softbuffer + winit | B / I, including focus and resize checks | B / I | B / S; UI blocked |
| SDL3 | B / I, including focus and resize checks | B / I; LNK4098 linker warning remains | B / S; UI blocked |
| Vulkan via ash | B / blocked: missing Vulkan loader/MoltenVK | B / I (simple clear-attachment renderer only) | B / S; UI blocked |
| Slint | B / I | U / U | U / U |
| GPUI | B / I: Enter start, held-W movement, scoring, Space pause | U / U | U / U |
| GTK4 | Blocked: native dependencies missing / U | U / U | B / S; UI blocked |
| Ratatui | B / pseudo-terminal checks; enhanced-terminal interaction unverified | B / blocked: direct launch has no targetable window; Windows Terminal access blocked by product policy | B / pseudo-terminal startup, movement and quit; 4 tests pass |

Details: [low-level clients](experiments/LOW-LEVEL.md), [GUI toolkits](experiments/GUI-TOOLKITS.md), [Ratatui](experiments/ratatui/README.md). GPUI's initial default Mac check required the separate Metal Toolchain; the runtime-shader path and later native worker results are reflected above.

### Rust web UI frameworks

| Implementation / target | Mac | Windows | Omarchy |
| --- | --- | --- | --- |
| Yew browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Leptos browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Dioxus browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Yew native Tauri wrapper | B / U | B / I | U / U |
| Leptos native Tauri wrapper | B / U | B / I | U / U |
| Dioxus native desktop | U / U | B / I; maximized layout checked | U / U |

[Commands and detailed web-framework results](docs/web-framework-comparison.md). Browser checks use a localhost HTTP server, never file URLs. Narrow browser layout is not a physical touchscreen or mobile-package test.

### Mobile targets

| Implementation | Test host / target | Build and tests | Actual runtime evidence | Still outstanding |
| --- | --- | --- | --- | --- |
| Native Kotlin | Windows / Android 15 API35 emulator | Debug APK + 9 unit tests pass | Installed; touch paddle movement, ball motion, pause/resume/restart checked | Full match, physical device, store packaging |
| Native Swift | Mac / iPhone16 iOS18.4 simulator | Build + 9 core tests + start/pause/resume/restart XCUITest pass | Installed/launched; ready-screen pixels checked | Full touch match, physical device, store packaging |
| Tauri + Rust WASM iOS | Mac / iPhone16 iOS18.4 simulator | Development build passes via CLI coordinator + xcodebuild | Device Hub visual start, held Up/Down touch, scoring, pause/restart pass | White bottom safe-area strip; full match/physical device/standalone release unverified |
| Tauri + Rust WASM Android | Windows / Android emulator | Build/install pass | Start, scoring and pause verified | Sustained touch, full match, physical device and release packaging |
| Yew/Leptos Tauri mobile; Dioxus mobile | iOS / Android | U | U | Mobile setup, packaging and interaction qualification |
| Godot Rust mobile/web exports | iOS / Android / web | U | U | Export pipeline/templates; mobile touch controls not implemented |

Details: [native Kotlin/Swift](experiments/NATIVE-MOBILE.md), [Tauri mobile](docs/mobile-comparison.md), [Godot scope](experiments/godot/README.md). Other engine mobile exports are not implied by their desktop results.

### Acceptance coverage and remaining work

| Check or issue | Current evidence / remaining work |
| --- | --- |
| Shared simulation | 9 Rust core/WASM tests; 6 original JS tests + 2 compiled-WASM ABI tests passed. This is not frontend input QA. |
| Adapter regressions | Windows web controller 3/3, low-level controller 3/3, Ratatui 4/4, bracket-lib 6/6; other adapter results linked above. |
| Windows engine interaction | Most I cells mean start, scoring/ball motion, pause/resume and reset. Sustained paddle movement/release, full matches, focus/minimize and resize/DPI are mostly still unverified. |
| Windows web interaction | Browser variants exercised keyboard/pointer and CPU win; real focus-loss still unverified. Native wrappers exercised pointer movement and basic controls, not held-key/full-match/physical-touch coverage. |
| Omarchy visual/input testing | Many builds and smoke launches pass, but the automation connection exposes no native/browser surfaces. This is a tool-access blocker, not proof of a gameplay failure. |
| Known defects | Windows Bevy graphics; Windows bracket-lib bitmap text remains rough despite improved layout; Miniquad/raylib supported start input unsuccessful; Godot headless editor shutdown crash. |
| Snapshot freshness | Windows extra-engine interaction used existing isolated GNU binaries, not a fresh latest-source rebuild. Mac nannou UI predates the pacing patch. bracket-lib header layout has been rechecked on Windows only. Requalification remains necessary before release. |
| Packaging and compatibility | No complete signed-installer/download-install matrix, physical-mobile coverage, exhaustive OS/DPI/input matrix or accessibility audit. |
| Performance | No controlled cross-engine benchmark; debug build sizes/timings are not fair comparisons. |

Keep this dashboard updated with every meaningful testing checkpoint. This dashboard also incorporates coordinator-confirmed later GPUI, Tauri mobile and Omarchy Ratatui worker results that are not yet reflected in every linked document. Detailed logs contain earlier states followed by newer corrections: use the latest dated follow-up, not an obsolete blocker paragraph. **U means unrun/unverified, not failed; I does not mean everything passed.** Comfy and rend3 were intentionally excluded, not left waiting for tests.
