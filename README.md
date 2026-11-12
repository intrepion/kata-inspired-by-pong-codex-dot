# Rallylight

An original real-time paddle game inspired by Pong. Return the ball, steer shots with your paddle, and beat the computer to seven points. The computer has a short reaction delay and a limited paddle speed, so well-placed shots can get past it. This independent kata is not affiliated with or endorsed by Atari.

## Current activity (last reported)

**Snapshot updated: 2026-11-12 13:09 UTC.** This is a committed progress snapshot, **not live process telemetry**. Refreshing this checkpoint branch shows the latest published report, not an automatic view of the computers. Activity is updated at meaningful starts, completions, pauses, and blockers; a row can become stale between updates.

| Computer | Last reported activity | Phase / next step |
| --- | --- | --- |
| Windows | Three notice-bearing ZIP pilots pass for softbuffer, SDL3 and ash: all staged/extracted hashes match, 39 states and 18 captures. Fixed a Windows license-fallback path containment bug with a failing-then-passing regression. | [Windows notice packages](docs/packaging/WINDOWS-NOTICE-PACKAGES.md). Thirteen packaging tests pass/seven platform skips. Legal/linked-library completeness, installer and physical-input gates remain open. |
| Mac | Corrected Pixels package passed supported native start, held W/S movement, pause/resume, 0–7 Over and restart/reset. Actual window inspected; app-menu Quit confirmed process exit. | [Pixels native interaction evidence](docs/NATIVE-HELD-BINDINGS-FIX.md). Overlapping physical keys, OS-focus and resize remain separate. No new builds. |
| Omarchy ARM64 | Shared notice-path containment fix cross-checked: eighteen packaging/notice/state-safety tests pass; two Windows-only junction cases skip. GTK/Slint corrected-package replay evidence retained. | [Linux cross-platform notice validation](docs/packaging/WINDOWS-NOTICE-PACKAGES.md#linux-cross-platform-validation-2026-11-12). No license-clearance, new OS-input or public-release claim. |

The testing matrices below record **verified results**, separately from work in progress. An activity row does not imply that an application is currently open, that every check passed, or that a release is ready.

## Play

[Play Rallylight online](https://intrepion.github.io/kata-inspired-by-pong-codex-dot/).

Run `python3 -m http.server 8000 --bind 127.0.0.1` and visit `http://127.0.0.1:8000`. Do not open the HTML file directly. The checked-in generated game.js can be served directly. To develop the TypeScript source, run `npm ci`, `npm test`, then `npm run serve` and visit `http://127.0.0.1:4189/`. Edit `game.ts`, not generated `game.js`.

Move with the up/down arrows or W/S. Press Space or use the Pause button to pause. On a touch screen, hold the arrow buttons or drag on the court. The game pauses when the tab is hidden or the window loses focus. Hit the ball near the edge of your paddle to change its angle.

## Test

Run `npm ci && npm test` to type-check/build with pinned TypeScript and run all 15 regression tests. `node --test tests/game.test.js` tests the checked-in generated JavaScript only.

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
| Original TypeScript + Canvas browser | Updated UI not rechecked on Mac | Strict TS build + 15 tests pass; localhost normal start/pause/resume and 390×844 layout inspected at `31478c6` | Updated UI not rechecked on Omarchy |
| Rust WASM + Canvas browser | B / I | B / I; normal start/pause/resume rechecked at `31478c6`, 7 scenario/WASM tests pass | B / localhost serves; UI blocked |
| Macroquad | B / I | B / R; keyboard play unverified | B / S + scripted replay and app-frame PNGs inspected; real input blocked |
| Bevy | B / I; Mesh2d rechecked at `55d0975` | B / I + scripted captures **with DX12**; default Vulkan **F** ([details](docs/WINDOWS-BEVY-DIAGNOSIS.md)) | B / S + scripted/PNG sample passed; real input blocked |
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
| GPUI | B / I: Enter start, held-W movement, scoring, Space pause | B / I; maximized rendering checked | B / native state-only replay passed; visual/input unverified |
| GTK4 | Blocked: native dependencies missing / U | U / U | B / S + scripted Cairo PNG samples passed; real input unverified |
| Ratatui | B / pseudo-terminal checks; enhanced-terminal interaction unverified | B / blocked: direct launch has no targetable window; Windows Terminal access blocked by product policy | B / pseudo-terminal startup, movement and quit; scripted cell-buffer evidence; 5 tests pass |

Details: [low-level clients](experiments/LOW-LEVEL.md), [GUI toolkits](experiments/GUI-TOOLKITS.md), [Ratatui](experiments/ratatui/README.md). GPUI's initial default Mac check required the separate Metal Toolchain; the runtime-shader path and later native worker results are reflected above.

### Rust web UI frameworks

| Implementation / target | Mac | Windows | Omarchy |
| --- | --- | --- | --- |
| Yew browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Leptos browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Dioxus browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Yew native Tauri wrapper | Fresh `b92b7df` B + state/SVG replay + bounded I; provenance/evidence published `18bc91c` | B / I | B / scripted SVG-source checks passed; visual/input unverified |
| Leptos native Tauri wrapper | Fresh `b92b7df` B + state/SVG replay + bounded I; provenance/evidence published `18bc91c` | B / I | B / scripted SVG-source checks passed; visual/input unverified |
| Dioxus native desktop | Fresh `4088089` B / I + native replay; held controls/release, full match and restart passed | B / I; maximized layout checked | B / native scripted SVG-source checks passed; visual/input unverified |

[Commands and detailed web-framework results](docs/web-framework-comparison.md). Browser checks use a localhost HTTP server, never file URLs. Narrow browser layout is not a physical touchscreen or mobile-package test.

### Mobile targets

| Implementation | Test host / target | Build and tests | Actual runtime evidence | Still outstanding |
| --- | --- | --- | --- | --- |
| Native Kotlin | Windows / Android 15 API35 emulator | Debug APK + 11 unit tests pass (latest replay candidate) | Installed; touch paddle movement, ball motion, pause/resume/restart checked | Full match, physical device, store packaging |
| Native Swift | Mac / iPhone16 Pro iOS18.4 simulator (latest) | `038f85e` Debug/Release builds + 11 core tests + 2 UI tests pass | DEBUG app replay 1440 ticks / 13 states / 6 app-owned PNGs; normal touch/release, Home pause/resume, full match and restart passed (`18bc91c`) | Physical device, store packaging |
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
| Performance | Fourteen Mac and fifteen Omarchy native adapter pilots recorded; Windows results are listed individually in the timing matrix below. Callback intervals only, not controlled cross-engine rankings or full acceptance. |

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
| Raw web-WASM | Implemented; ABI tests passed | Canvas export implemented; Windows completion verified, bundle files unverified | Not counted among nineteen verified render adapters |
| GPUI | Passed (native state-only) | Unsupported; request explicitly rejected | Telemetry only, not visual QA |
| Yew / Leptos / Dioxus web | Shared replay tests/builds pass; Windows Chrome demos reached tick 1440 with 13 identical states | Six SVG links per app and final courts inspected on Windows; data-URL download blocked | SVG source is not screenshot evidence; downloaded files unverified; no Omarchy browser pass |
| Tauri + Rust WASM | Passed (native WebKitGTK) | Six canvas PNGs; pause/reset inspected | Court only, not DOM scores or full webview |
| Yew / Leptos native wrappers | Passed | Six SVG sources each; geometry checked | Source export, not screenshot or visual QA |
| Dioxus native | Passed | Six SVG sources; geometry checked | Source export, not final DOM presentation or screenshot |
| Original TypeScript | Portable-action tests pass; Windows bounded scripted completion verified | Canvas export link created; bundle contents unverified; Windows normal UI/390×844 inspected separately | TS physics/RNG trajectory differs from Rust; no scripted visual pass |
| Independent Swift / Kotlin | Kotlin compile + 11 JUnit tests + emulator replay pass; Swift DEBUG app replay + 11 core / 2 simulator UI tests and Debug/Release builds pass | Six app-owned PNGs each; Swift pause/reset inspected, not simulator screenshots | [Android evidence](docs/SCRIPTED-ANDROID.md), [Swift evidence](docs/evidence/swift-ios-replay-20261112); no physical-device or cross-language parity claim |
| Remaining Rust / Swift / Kotlin adapters | Not wired yet | Not wired yet | No implied pass |

All twenty render-evidence adapters finish at tick 1440 with the same baseline telemetry. Shared core has
14 passing tests; raylib has one additional viewport test. Details and artifact
hashes: [scripted testing](docs/SCRIPTED-TESTING.md).

### Planned benchmark and packaging architecture

Approved direction: use **one shared benchmark harness**, with platform-specific helpers only where useful. Keep packaging alongside each application until repeated common steps justify extracting shared tooling. This is a plan, not measured performance or completed release packaging. See [benchmark and packaging plan](docs/BENCHMARK-PLAN.md).

Latest Bevy Linux source requalification: [Mesh2d native Vulkan replay and inspected captures](docs/OMARCHY-BEVY-MESH-RECHECK.md). Real-input/compositor checks remain separate.

Windows Miniquad follow-up (worker report): fresh MSVC build and one adapter test passed; bounded replay reached tick 1440, exported six PNGs, and paused sample looked correct. Native Return/Space still did not visibly start play. The test window was closed before Windows disconnected; evidence files remain on that machine pending retrieval. This is not a native-input pass.

GPUI Linux state-only replay now builds and completes after a narrow, licensed xattr compatibility patch; see [provenance and evidence](docs/SCRIPTED-GPUI.md). No libc downgrade or image-capture claim. [Browser-framework replay](experiments/web-ui-common/SCRIPTED-DEMO.md) is tested/build-verified. Windows localhost Chrome at `31478c6` subsequently completed Yew/Leptos/Dioxus demos at tick 1440 with 13 identical telemetry records; six SVG links per app and final rendered courts were inspected. Data-URL downloads were blocked, so exported-file integrity remains unverified. Raw WASM normal start/pause/resume and rendered court were verified on Windows at `31478c642cf07d536cd979d82e81ea900a3c1acd`; seven scenario/WASM tests passed. That earlier run did not verify scripted completion; the fresh Windows follow-up below subsequently verified completion. The browser accepts only HTTP/HTTPS downloads and rejected the existing data-URL link; no indirect workaround was attempted. These are not Omarchy browser results.

[Dioxus native shared replay](docs/SCRIPTED-DIOXUS-NATIVE.md) passes source/telemetry validation. Tauri paused label now stays above the playfield and no longer covers the ball, confirmed in a fresh native canvas export.

[Portable scenario and original TypeScript replay](docs/SCRIPTED-ORIGINAL-TYPESCRIPT.md): one manifest drives both timelines; Rust preserves its existing baseline exactly. TypeScript uses its own physics/RNG, so only action/checkpoint semantics are shared. The later Windows follow-up below verifies bounded execution and export-link creation; downloaded canvas bundle contents remain unverified.

[Portable Swift/Kotlin replay source candidate](docs/SCRIPTED-NATIVE-MOBILE.md) consumes the shared manifest. Kotlin compile/JUnit and APK resource checks pass; Swift DEBUG app replay is now wired in `038f85e`, with 11 core tests, two simulator UI tests and Debug/Release builds passing; Kotlin emulator replay/captures also pass. Swift exports are app-owned ImageRenderer frames, not simulator screenshots.

Windows original TypeScript follow-up at `31478c642cf07d536cd979d82e81ea900a3c1acd`: localhost Chrome normal Start → Rally in progress, Pause → Paused, Resume → Rally in progress verified. A 390×844 screenshot showed court/scoreboard fitting, touch controls visible and no horizontal clipping; viewport override reset. `npm test` passed all 15 tests. In that earlier run, raw WASM and TypeScript scripted scores advanced without observed completion links. The fresh Windows follow-up below supersedes that completion limitation; bundle integrity remains unverified. Focus-loss and raw WASM resize remain unverified.

Mac follow-up published in `18bc91c`: [Swift normal UI regression](docs/evidence/swift-ios-normal-ui-20261112.txt) passed in 47.349 seconds using genuine touch, release, background/resume, game-over and restart assertions; no physics injection. [Fresh Yew/Leptos native interaction evidence](docs/evidence/mac-fresh-web-wrapper-ui-20261112.json) records rendering, Enter start, W/S movement, Space pause/resume, CPU scoring and restart. Wrapper focus/resize/full-match remain unverified. Both apps exited and the simulator was shut down after checks.

Windows follow-up: [fresh original TypeScript and raw-WASM browser replay completion](docs/WINDOWS-BROWSER-REPLAY-FOLLOWUP.md). Both completion links appeared; raw final court inspected. Bundle contents, true focus transitions and responsive override remain unverified.

[Omarchy portable-manifest regression](docs/MANIFEST-REGRESSION-OMARCHY.md): source 0cbe38f preserves sixteen shared tests and exact scripted baselines; Macroquad/Piston app-owned capture samples rechecked. This is not benchmark or manual-input evidence.

## Benchmark pilot

The [shared Macroquad benchmark pilot](docs/benchmarks/README.md) completed three release runs on Omarchy. Median **frame-callback intervals** were about 8.33 ms, not measured display FPS. Raw samples, rejected attempts and metadata limits are retained. This is a harness pilot, **not a cross-engine ranking**; CPU/memory/startup/power/presentation metrics remain unmeasured; separate packaging pilot follows.

## Packaging pilot

[Macroquad Linux ARM64 portable archive](docs/packaging/README.md): local creation, checksum validation, isolated staged extraction, installed scripted run and exact-file uninstall passed. Archive remains local and is not a distribution-ready release. System installation, clean-machine/cross-distro compatibility, upgrades, signing and full dependency notices remain pending.

Second shared benchmark adapter: [softbuffer/winit pilot](docs/benchmarks/SOFTBUFFER-ADAPTER.md), three valid bounded release runs, shared scenario/sampler and explicit runner metadata. Existing 8ms pacing remains; no displayed-FPS or engine ranking claim.

Third shared benchmark adapter: [SDL3 pilot](docs/benchmarks/SDL3-ADAPTER.md), three valid bounded release runs with actual SDL driver/renderer metadata. Existing 8ms sleep remains; no engine ranking.

### Macroquad cross-platform pilot coverage

| Computer | Timing pilot | Package lifecycle |
| --- | --- | --- |
| Omarchy ARM64 | Three callback-timing runs; [protocol/evidence](docs/benchmarks/README.md) | Local extraction/replay/uninstall passed; [limits](docs/packaging/README.md) |
| Windows | Three valid release repeats; [evidence](docs/benchmarks/WINDOWS-MACROQUAD-PILOT.md) | Portable ZIP/extraction + scripted run passed; not installer/clean-machine/signing qualified. [Pilot](docs/packaging/MACROQUAD-WINDOWS-PILOT.md) |
| Mac ARM64 | Earlier three valid release repeats; [evidence](docs/benchmarks/MACROQUAD-MAC-PILOT.md) | Creation/extraction and bounded extracted-app GUI gameplay passed; shell launch failed; [details](docs/packaging/MACROQUAD-MAC-PILOT.md) |

These are differently configured machines and callback boundaries, not a controlled ranking. No public installer release is implied.

Fourth shared benchmark adapter: [raylib-rs](docs/benchmarks/RAYLIB-ADAPTER.md), three valid Linux ARM64 runs using shared scenario/sampler and unchanged120fps pacing; no new manual gameplay or package claim.

Fifth shared benchmark adapter: [pixels/winit](docs/benchmarks/PIXELS-ADAPTER.md), three valid Linux ARM64 callbacks pilots with fixed CPU source-buffer size and recorded output dimensions.

### Shared callback-timing coverage by adapter

`3/3` means three valid bounded release callback-timing runs, **not displayed FPS, GPU time, full gameplay acceptance or a comparable performance ranking**. `Not recorded` means this checkpoint has no published timing pilot for that combination; it does not mean the implementation failed. Run conditions and dimensions differ: follow each report.

| Adapter | Mac ARM64 | Windows | Omarchy ARM64 |
| --- | --- | --- | --- |
| Macroquad | [3/3](docs/benchmarks/MACROQUAD-MAC-PILOT.md) | [3/3](docs/benchmarks/WINDOWS-MACROQUAD-PILOT.md) | [3/3](docs/benchmarks/README.md) |
| softbuffer/winit | [3/3](docs/benchmarks/SOFTBUFFER-MAC-PILOT.md) | [3/3](docs/benchmarks/SOFTBUFFER-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/SOFTBUFFER-ADAPTER.md) |
| SDL3 | [3/3](docs/benchmarks/SDL3-MAC-PILOT.md) | [3/3](docs/benchmarks/SDL3-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/SDL3-ADAPTER.md) |
| raylib-rs | [3/3](docs/benchmarks/RAYLIB-MAC-PILOT.md) | [3/3](docs/benchmarks/RAYLIB-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/RAYLIB-ADAPTER.md) |
| pixels/winit | [3/3](docs/benchmarks/PIXELS-MAC-PILOT.md) | [3/3](docs/benchmarks/PIXELS-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/PIXELS-ADAPTER.md) |
| Miniquad | [3/3](docs/benchmarks/MINIQUAD-MAC-PILOT.md) | [3/3](docs/benchmarks/MINIQUAD-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/MINIQUAD-ADAPTER.md) |
| Piston | [3/3](docs/benchmarks/PISTON-MAC-PILOT.md) | [3/3](docs/benchmarks/PISTON-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/PISTON-ADAPTER.md) |
| ggez | [3/3](docs/benchmarks/GGEZ-MAC-PILOT.md) | [3/3](docs/benchmarks/GGEZ-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/GGEZ-ADAPTER.md) |
| Nannou | [3/3 repeat; initial rejection retained](docs/benchmarks/NANNOU-MAC-PILOT.md) | [3/3](docs/benchmarks/NANNOU-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/NANNOU-ADAPTER.md) |
| raw wgpu | [3/3](docs/benchmarks/WGPU-MAC-PILOT.md) | [3/3](docs/benchmarks/WGPU-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/WGPU-ADAPTER.md) |
| bracket-lib | [3/3](docs/benchmarks/BRACKET-LIB-MAC-PILOT.md) | [3/3](docs/benchmarks/BRACKET-LIB-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/BRACKET-LIB-ADAPTER.md) |
| Bevy (main-world callback) | [3/3](docs/benchmarks/BEVY-MAC-PILOT.md) | [DX12 3/3; Vulkan rejected](docs/benchmarks/BEVY-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/BEVY-ADAPTER.md) |
| Blue Engine (native callback) | [3/3](docs/benchmarks/BLUE-ENGINE-MAC-PILOT.md) | [3/3](docs/benchmarks/BLUE-ENGINE-WINDOWS-PILOT.md) | [3/3; initial failure retained](docs/benchmarks/BLUE-ENGINE-ADAPTER.md) |
| Fyrox (pre-render callback) | [3/3](docs/benchmarks/FYROX-MAC-PILOT.md) | [3/3](docs/benchmarks/FYROX-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/FYROX-ADAPTER.md) |
| ash / Vulkan | Not recorded; Vulkan loader previously blocked | [3/3](docs/benchmarks/ASH-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/ASH-ADAPTER.md) |

Other implementations have not yet been qualified through this shared timing protocol. Package lifecycle status is a separate dimension: Mac [Macroquad](docs/packaging/MACROQUAD-MAC-PILOT.md) and [SDL3](docs/packaging/SDL3-MAC-PILOT.md) have local extracted-app gameplay/removal evidence, not signed or clean-machine release qualification.

Additional local package lifecycle: [ash/Vulkan Linux ARM64](docs/packaging/ASH-LINUX-PILOT.md) passed relocation/replay/capture/removal; runtime dependencies and release gates remain explicit.

Mac [raylib-rs local package pilot](docs/packaging/RAYLIB-MAC-PILOT.md) also passed extracted native scripted replay/captures, bounded real-input controls and scoped removal; no signed distribution qualification inferred.

Windows notice preparation: [three local notice-bearing ZIPs](docs/packaging/WINDOWS-NOTICE-PACKAGES.md) with discovered texts and hash indexes, not legal or release clearance.

Windows corrected GUI/terminal package follow-up: [Ratatui, Slint and GPUI](docs/packaging/WINDOWS-GUI-HELD-PACKAGE.md), including the GPUI hidden-launch limitation.

Windows corrected held-input package follow-up: [five rebuilt portable archives](docs/packaging/WINDOWS-HELD-FIXES-PACKAGE.md) preserve historical results while recording current binary hashes.

Linux portable lifecycle expansion: [Miniquad, pixels, raw wgpu and raylib-rs](docs/packaging/LINUX-PORTABLE-BATCH.md) have verified local extraction/replay/capture/removal evidence.

Corrected-input package follow-up: [five Linux adapters](docs/packaging/LINUX-HELD-FIXES-PACKAGE.md) requalified with fixed binaries; older linked pilots retain historical hashes.

### Local portable-package lifecycle coverage

These are local archive extraction, relocated runtime and removal checks, **not public installer or clean-machine release qualification**. “Not recorded” is not a failure. Native input remains a separate gate.

| Adapter | Mac | Windows | Omarchy ARM64 |
| --- | --- | --- | --- |
| Macroquad | Local pilot | Local pilot | Local pilot |
| SDL3 | Local pilot | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | Local pilot |
| raylib | [Local replay + bounded native input](docs/packaging/RAYLIB-MAC-PILOT.md) | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | Local pilot |
| ash | Loader blocked | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | Local pilot |
| Miniquad | [Local replay/capture pilot](docs/packaging/MINIQUAD-MAC-PILOT.md) | [Local ZIP/replay pilot](docs/packaging/MINIQUAD-WINDOWS-PILOT.md) | [Four-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH.md) |
| pixels, raw wgpu | [Local replay/capture pilots](docs/packaging/MAC-PIXELS-WGPU-PILOT.md) | [Local ZIP/replay pilots](docs/packaging/WINDOWS-PORTABLE-BATCH.md) | [Four-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH.md) |
| Piston, ggez | [Local replay/capture pilots](docs/packaging/MAC-PISTON-GGEZ-PILOT.md) | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | [Six-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH2.md) |
| Fyrox | [Corrected HUD replay passed; full cleanup incomplete](docs/FYROX-HUD-SCALE-FIX.md) | [Local ZIP/replay; generated state preserved](docs/packaging/WINDOWS-PORTABLE-BATCH3.md) | [Corrected HUD replay passed; full cleanup incomplete](docs/FYROX-HUD-SCALE-FIX.md) |
| Nannou, bracket-lib, Bevy | [Local replay/capture pilots](docs/packaging/MAC-FOUR-ENGINE-PILOT.md) | [Local ZIP/replay pilots; Bevy DX12](docs/packaging/WINDOWS-PORTABLE-BATCH3.md) | [Six-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH2.md) |
| Blue Engine, softbuffer | [Local pilots; Blue state-only](docs/packaging/MAC-SOFTBUFFER-BLUE-PILOT.md) | [Blue state-only](docs/packaging/WINDOWS-PORTABLE-BATCH3.md); [softbuffer replay/capture](docs/packaging/WINDOWS-PORTABLE-BATCH.md) | [Final two local pilots](docs/packaging/LINUX-PORTABLE-BATCH3.md); Blue native captures unsupported |
| Ratatui | Not recorded | [Local ZIP/cell replay only](docs/packaging/RATATUI-WINDOWS-PILOT.md) | [Terminal local pilot](docs/packaging/RATATUI-LINUX-PILOT.md); cells + bounded PTY, not screenshots |
| GTK4 | Not recorded | Not recorded | [Local package pilot](docs/packaging/GTK-LINUX-PILOT.md); existing GTK4 libraries required |
| Slint | Not recorded | [Local ZIP/own-window snapshots](docs/packaging/SLINT-WINDOWS-PILOT.md) | [Local package pilot](docs/packaging/SLINT-LINUX-PILOT.md); own-window snapshots, license text included |
| GPUI | Not recorded | [Local ZIP/state-only replay](docs/packaging/GPUI-WINDOWS-PILOT.md) | [Local archive/state-only replay](docs/packaging/GPUI-LINUX-PILOT.md) |
| Other adapters | Not recorded | Not recorded | Not recorded |

Evidence and limitations: [packaging index](docs/packaging/README.md), [Mac Macroquad](docs/packaging/MACROQUAD-MAC-PILOT.md), [Windows Macroquad](docs/packaging/MACROQUAD-WINDOWS-PILOT.md), [Mac SDL3](docs/packaging/SDL3-MAC-PILOT.md), [Mac raylib](docs/packaging/RAYLIB-MAC-PILOT.md), [Linux ash](docs/packaging/ASH-LINUX-PILOT.md).
