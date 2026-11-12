# Rallylight

An original real-time paddle game inspired by Pong. Return the ball, steer shots with your paddle, and beat the computer to seven points. The computer has a short reaction delay and a limited paddle speed, so well-placed shots can get past it. This independent kata is not affiliated with or endorsed by Atari.

## Current activity (last reported)

**Snapshot updated: 2026-11-12 03:59 UTC.** This is a committed progress snapshot, **not live process telemetry**. Refreshing this checkpoint branch shows the latest published report, not an automatic view of the computers. Activity is updated at meaningful starts, completions, pauses, and blockers; a row can become stale between updates.

| Computer | Last reported activity | Phase / next step |
| --- | --- | --- |
| Windows | Miniquad fresh MSVC build, one test, and scripted replay (1440 ticks / six captures) passed per worker report; native Return/Space start attempts still failed. Bevy scoped fix/evidence was published in `55d0975`. | **Disconnected; testing paused.** No active test window. Miniquad results/evidence remain uncommitted and await reconnect/publication; no native-input pass claimed. |
| Mac | Yew and Leptos native-wrapper checks assigned, followed by Dioxus native desktop. Existing test binaries use an older checkpoint. | Preparing visual checks with existing `6ceb22a` binaries; Pong Yew app-access call pending, possibly approval. Gameplay not yet confirmed; Dioxus build queued. |
| Omarchy ARM64 | Native Yew/Leptos shared replay completed with SVG-source and telemetry checks. | Publishing verified source-evidence subset; Dioxus and Tauri pause polish await shared-source coordination. SVG exports are not screenshots. |

The testing matrices below record **verified results**, separately from work in progress. An activity row does not imply that an application is currently open, that every check passed, or that a release is ready.

## Play

[Play Rallylight online](https://intrepion.github.io/kata-inspired-by-pong-codex-dot/).

Run `python3 -m http.server 8000 --bind 127.0.0.1` and visit `http://127.0.0.1:8000`. Do not open the HTML file directly. The checked-in generated game.js can be served directly. To develop the TypeScript source, run `npm ci`, `npm test`, then `npm run serve` and visit `http://127.0.0.1:4189/`. Edit `game.ts`, not generated `game.js`.

Move with the up/down arrows or W/S. Press Space or use the Pause button to pause. On a touch screen, hold the arrow buttons or drag on the court. The game pauses when the tab is hidden or the window loses focus. Hit the ball near the edge of your paddle to change its angle.

## Test

Run `npm ci && npm test` to type-check/build with pinned TypeScript and run all 12 regression tests. `node --test tests/game.test.js` tests the checked-in generated JavaScript only.

The original browser game is now maintained in strict TypeScript. Rust comparison work exposed and fixed combined-input double speed, low-frame-rate slowdown, and nonfinite-value poisoning. The browser uses 120 Hz steps with at most 100 ms of catch-up per frame; pauses and restarts reset timing. See [original browser verification](docs/original-browser-verification.md) for evidence and limits.

## License

MIT; see [LICENSE](LICENSE). The root `.gitignore` combines GitHub's global Linux, macOS, and Windows templates.

## Arcade metadata

See [docs/arcade-metadata.md](docs/arcade-metadata.md) for the game-owned manifest and screenshot provenance.

## Rust / desktop comparison

Four frontends share a Rust game simulation. See [the comparison guide](docs/desktop-comparison.md) for architecture, commands, and verification. The original web game now has a strict TypeScript source and regression fixes; its UI and intended gameplay remain the same.

## Implementation testing dashboard

Updated **2026-11-12** on the experimental `codex/pong-comparison-checkpoint-20261111` branch. This is a testing checkpoint, **not a release certification**. The original hosted game is unchanged; compiled downloads/installers are not published yet.

Each desktop cell is **build / runtime evidence**:

- **B**: full development build passed; **C**: check only; **U**: not yet verified.
- **I**: bounded visible interaction passed, not the entire acceptance checklist.
- **S**: process startup only; **R**: rendered, gameplay not verified.
- **F**: observed failure; **blocked**: a concrete tool or prerequisite prevents the next check.
- **n/a**: not a target for that implementation. Unit tests do not count as gameplay.

### Core frontends and game engines

| Implementation | Mac build / runtime | Windows build / runtime | Omarchy ARM64 build / runtime |
| --- | --- | --- | --- |
| Original TypeScript + Canvas browser | Updated UI not rechecked on Mac | Strict TS build + 12 tests pass; localhost start/rally/pause/resume smoke | Updated UI not rechecked on Omarchy |
| Rust WASM + Canvas browser | B / I | B / I | B / localhost serves; UI blocked |
| Macroquad | B / I | B / R; keyboard play unverified | B / S + scripted replay and app-frame PNGs inspected; real input blocked |
| Bevy | B / I | B / I + scripted captures **with DX12**; default Vulkan **F** ([details](docs/WINDOWS-BEVY-DIAGNOSIS.md)) | B / S + scripted/PNG sample passed; real input blocked |
| Tauri + Rust WASM | B / I | B / I | B / native scripted Canvas PNG samples passed; real input unverified |
| Miniquad | B / I | Fresh MSVC B + 1 test; scripted/six PNG reported passed; native Return/Space still no start | B / S + scripted/PNG sample passed; real input blocked |
| pixels + winit | B / I | B / I | B / S + scripted/source PNG passed; real input blocked |
| raw wgpu + winit | B / I | B / I | B / S + scripted/PNG sample passed; real input blocked |
| Piston | B / I | B / I | B / S + scripted/PNG sample passed; real input blocked |
| ggez | B / I; Mac surface fix verified | B / I on earlier snapshot | B / S + scripted/PNG sample passed; real input blocked |
| nannou | B / I before pacing follow-up | B / I | B / S + scripted/PNG sample passed; real input blocked |
| Fyrox | B / I | B / I | B / S + scripted/PNG samples passed; real input blocked |
| Blue Engine | B / I | B / I | B / S + headless replay/PNG samples passed; window/input unverified |
| raylib-rs | B / I | B / R; start input unsuccessful; LLVM build blocker resolved | B / S + scripted/PNG sample passed; real input blocked |
| Godot + Rust GDExtension | B / I for game; **F** in headless editor shutdown | U / U | B / scripted native viewport PNG samples passed; real input unverified |
| bracket-lib (RLTK) | B / I; restart/exit incomplete | B / I; separate header improves readability, bitmap quality still rough | B / S on X11 and patched Wayland + scripted/PNG samples passed; real input blocked |

Details: [baseline frontends](docs/desktop-comparison.md), [extra engines](experiments/ENGINE-STATUS.md), [ggez fix](experiments/ggez/README.md), [Godot](experiments/godot/VERIFICATION.md), [bracket-lib](experiments/bracket-lib/README.md).

### Low-level rendering and native GUI/terminal clients

| Implementation | Mac build / runtime | Windows build / runtime | Omarchy ARM64 build / runtime |
| --- | --- | --- | --- |
| softbuffer + winit | B / I, including focus and resize checks | B / I | B / S + scripted/PNG sample passed; real input blocked |
| SDL3 | B / I, including focus and resize checks | B / I; LNK4098 linker warning remains | B / S + scripted/PNG sample passed; real input blocked |
| Vulkan via ash | B / blocked: missing Vulkan loader/MoltenVK | B / I (simple clear-attachment renderer only) | B / S + scripted/PNG samples passed; real input blocked |
| Slint | B / I | B / I: start, scoring, pause/resume, reset | B / scripted component PNG samples passed; real input unverified |
| GPUI | B / I: Enter start, held-W movement, scoring, Space pause | B / I; maximized rendering checked | U / U |
| GTK4 | Blocked: native dependencies missing / U | U / U | B / S + scripted Cairo PNG samples passed; real input unverified |
| Ratatui | B / pseudo-terminal checks; enhanced-terminal interaction unverified | B / blocked: direct launch has no targetable window; Windows Terminal access blocked by product policy | B / pseudo-terminal startup, movement and quit; scripted cell-buffer evidence; 5 tests pass |

Details: [low-level clients](experiments/LOW-LEVEL.md), [GUI toolkits](experiments/GUI-TOOLKITS.md), [Ratatui](experiments/ratatui/README.md). GPUI's initial default Mac check required the separate Metal Toolchain; the runtime-shader path and later native worker results are reflected above.

### Rust web UI frameworks

| Implementation / target | Mac | Windows | Omarchy |
| --- | --- | --- | --- |
| Yew browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Leptos browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Dioxus browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Yew native Tauri wrapper | B / U | B / I | B / scripted SVG-source checks passed; visual/input unverified |
| Leptos native Tauri wrapper | B / U | B / I | B / scripted SVG-source checks passed; visual/input unverified |
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
| Adapter regressions | Windows web controller 3/3, low-level controller 3/3, Ratatui 5/5 (Omarchy current), bracket-lib 6/6, GUI support 4/4; other adapter results linked above. |
| Windows engine interaction | Most I cells mean start, scoring/ball motion, pause/resume and reset. Sustained paddle movement/release, full matches, focus/minimize and resize/DPI are mostly still unverified. |
| Windows web interaction | Browser variants exercised keyboard/pointer and CPU win; real focus-loss still unverified. Native wrappers exercised pointer movement and basic controls, not held-key/full-match/physical-touch coverage. |
| Omarchy visual/input testing | Many builds and smoke launches pass, but the automation connection exposes no native/browser surfaces. This is a tool-access blocker, not proof of a gameplay failure. |
| Known defects | Windows Bevy default Vulkan crash (DX12 rendering corrected); Windows bracket-lib bitmap text remains rough despite improved layout; Miniquad/raylib supported start input unsuccessful; Godot headless editor shutdown crash. |
| Snapshot freshness | Windows extra-engine interaction used existing isolated GNU binaries, not a fresh latest-source rebuild. Mac nannou UI predates the pacing patch. bracket-lib header layout has been rechecked on Windows only. Requalification remains necessary before release. |
| Packaging and compatibility | No complete signed-installer/download-install matrix, physical-mobile coverage, exhaustive OS/DPI/input matrix or accessibility audit. |
| Performance | No controlled cross-engine benchmark; debug build sizes/timings are not fair comparisons. |

Keep this dashboard updated with every meaningful testing checkpoint. This dashboard also incorporates coordinator-confirmed later GPUI, Tauri mobile and Omarchy Ratatui worker results that are not yet reflected in every linked document. Detailed logs contain earlier states followed by newer corrections: use the latest dated follow-up, not an obsolete blocker paragraph. **U means unrun/unverified, not failed; I does not mean everything passed.** Comfy and rend3 were intentionally excluded, not left waiting for tests.

### Scripted input testing (separate from interactive QA)

Macroquad now supports an opt-in, fixed-seed, 12-second application-input replay.
It exercises start, held movement, release, pause/resume and restart while the normal
window renders. See [scripted test commands and limits](docs/SCRIPTED-TESTING.md).
The shared driver has two regression tests (plus nine existing core tests).
No OS input injection or desktop capture is involved; telemetry does not establish
visual correctness or real keyboard behavior. Other adapters are not wired yet.

Omarchy ARM64: two native replay runs exited 0 at tick 1440 with identical 13-line
[telemetry](docs/evidence/omarchy-macroquad-scripted-20261112.jsonl). Build, 11 tests
and strict Clippy passed. This is **not** a visual/interactive PASS.

Optional `--scripted-captures ./new-smoke-images` now exports six app-owned PNGs.
Omarchy capture replay exited 0 with unchanged telemetry; all six frames were
inspected (movement, pause/resume, reset). This establishes bounded framebuffer
rendering evidence, **not** real keyboard/focus or smooth-motion validation.

#### Shared scripted-demo rollout (Omarchy)

| Adapter | Replay | App-buffer PNG export | Evidence boundary |
|---|---|---|---|
| Macroquad | Passed | Six saved; paused sample rechecked | Shared clock refactor verified |
| softbuffer | Passed | Six saved; paused sample inspected | Exact CPU presentation buffer |
| raylib-rs | Passed | Six saved; exact tick-480 inspected | Current app backbuffer before swap |
| Pixels | Passed | Six saved; paused source frame inspected | 900x500 CPU source buffer, not scaled GPU output |
| SDL3 | Passed | Six saved; paused sample inspected | Renderer target before present |
| Bevy | Passed | Six saved; pause/reset inspected | Framework screenshot completion awaited |
| nannou | Passed | Six saved; pause/reset inspected | Framework window-frame export; existing UI has no pause label |
| ggez | Passed | Six saved; pause/reset inspected | Same app render texture presented and captured |
| Miniquad | Passed | Six saved; paused sample inspected | Own OpenGL framebuffer; Metal capture unsupported |
| raw wgpu | Passed | Six saved; paused sample inspected | Surface COPY_SRC readback; requires capability |
| Piston | Passed | Six saved; pause/reset inspected | Same scene rendered to app-owned target, not swapchain |
| bracket-lib | Passed | Six saved; pause/reset inspected | Native framework framebuffer export |
| Fyrox | Passed | Six saved; pause/reset inspected | App-owned UI render target, not compositor |
| Blue Engine | Passed (headless) | Six saved; pause/reset inspected | Supported headless renderer only; window capture unsupported |
| Vulkan via ash | Passed | Six saved; all inspected | Acquired swapchain image copied before present |
| Godot + Rust | Passed | Six saved; pause/reset inspected | Actual scene viewport after frame draw |
| Ratatui | Passed (TestBackend) | No PNG: six text + six cell/style JSON exports | Cell-buffer evidence only, not terminal screenshots |
| Slint | Passed | Six saved; pause/reset inspected | Supported software-renderer component snapshot |
| GTK4 | Passed | Six saved; pause/reset inspected | Same Cairo scene on separate app-owned surface |
| Raw web-WASM | Implemented; ABI tests passed | Canvas export implemented, browser run unverified | Not counted among nineteen verified render adapters |
| GPUI | Passed (native state-only) | Unsupported; request explicitly rejected | Telemetry only, not visual QA |
| Yew / Leptos / Dioxus web | Shared replay tests and WASM builds pass | SVG scene-source export implemented, browser run unverified | SVG source is not screenshot evidence |
| Tauri + Rust WASM | Passed (native WebKitGTK) | Six canvas PNGs; pause/reset inspected | Court only, not DOM scores or full webview |
| Yew / Leptos native wrappers | Passed | Six SVG sources each; geometry checked | Source export, not screenshot or visual QA |
| Remaining Rust / Swift / Kotlin adapters | Not wired yet | Not wired yet | No implied pass |

All twenty render-evidence adapters finish at tick 1440 with the same baseline telemetry. Shared core has
14 passing tests; raylib has one additional viewport test. Details and artifact
hashes: [scripted testing](docs/SCRIPTED-TESTING.md).

### Planned benchmark and packaging architecture

Approved direction: use **one shared benchmark harness**, with platform-specific helpers only where useful. Keep packaging alongside each application until repeated common steps justify extracting shared tooling. This is a plan, not measured performance or completed release packaging. See [benchmark and packaging plan](docs/BENCHMARK-PLAN.md).

Latest Bevy Linux source requalification: [Mesh2d native Vulkan replay and inspected captures](docs/OMARCHY-BEVY-MESH-RECHECK.md). Real-input/compositor checks remain separate.

Windows Miniquad follow-up (worker report): fresh MSVC build and one adapter test passed; bounded replay reached tick 1440, exported six PNGs, and paused sample looked correct. Native Return/Space still did not visibly start play. The test window was closed before Windows disconnected; evidence files remain on that machine pending retrieval. This is not a native-input pass.

GPUI Linux state-only replay now builds and completes after a narrow, licensed xattr compatibility patch; see [provenance and evidence](docs/SCRIPTED-GPUI.md). No libc downgrade or image-capture claim. [Browser-framework replay](experiments/web-ui-common/SCRIPTED-DEMO.md) is tested/build-verified, but browser runtime and download links remain unverified.
