# Rallylight

An original real-time paddle game inspired by Pong. Return the ball, steer shots with your paddle, and beat the computer to seven points. The computer has a short reaction delay and a limited paddle speed, so well-placed shots can get past it. This independent kata is not affiliated with or endorsed by Atari.

## Excluded implementations

Godot's Rust GDExtension, the Tauri desktop/mobile client and Yew/Leptos native wrappers, Fyrox, and GPUI are excluded from the current build tree because the recorded dependency checks did not establish a path meeting this project's license policy. Browser-only Yew and Leptos remain. This does not declare the remaining implementations distribution-ready.

Their [source and historical results](https://github.com/intrepion/kata-inspired-by-pong-codex-dot/tree/265a13eaff0ba638c49f10946b0633ed79d3f405) remain in Git history; detailed reports under `docs/` are preserved as evidence, not current build instructions.

## Play

[Play Rallylight online](https://intrepion.github.io/kata-inspired-by-pong-codex-dot/).

Run `python3 -m http.server 8000 --bind 127.0.0.1` and visit `http://127.0.0.1:8000`. Do not open the HTML file directly. The checked-in generated game.js can be served directly. To develop the TypeScript source, run `npm ci`, `npm test`, then `npm run serve` and visit `http://127.0.0.1:4189/`. Edit `game.ts`, not generated `game.js`.

Move with the up/down arrows or W/S. Press Space or use the Pause button to pause. On a touch screen, hold the arrow buttons or drag on the court. The game pauses when the tab is hidden or the window loses focus. Hit the ball near the edge of your paddle to change its angle.

## Test

Run `npm ci && npm test` to type-check/build with pinned TypeScript and run the regression tests. `node --test tests/game.test.js` tests the checked-in generated JavaScript only.

The original browser game is now maintained in strict TypeScript. Rust comparison work exposed and fixed combined-input double speed, low-frame-rate slowdown, and nonfinite-value poisoning. The browser uses 120 Hz steps with at most 100 ms of catch-up per frame; pauses and restarts reset timing. See [original browser verification](docs/original-browser-verification.md) for evidence and limits.

## License

Project source is MIT; see [LICENSE](LICENSE). Third-party dependencies and assets have their own terms. License and notice review is incomplete; successful builds, tests and local package pilots do not establish distribution clearance.

## Arcade metadata

See [the game-owned manifest and screenshot provenance](docs/arcade-metadata.md).

## Implementation comparison

The Rust implementations share a game simulation, with adapter-specific rendering and input. The original browser game is maintained in strict TypeScript; independent Kotlin and Swift clients are also included. See [desktop architecture and commands](docs/desktop-comparison.md), [additional engines](experiments/ENGINE-STATUS.md), [web frameworks](docs/web-framework-comparison.md) and [mobile targets](experiments/NATIVE-MOBILE.md).

## Implementation testing dashboard

These tables retain recorded checkpoint evidence, principally from **2026-11-12**, with selected later follow-ups. They are **historical summaries, not a current-source qualification, live activity monitor or release certification**. Results apply to the revisions and configurations in the linked reports. Later fixes may supersede a listed failure or blocker; unlisted checks must not be assumed to have passed.

Interim progress posts have been removed from this README. Detailed reports and artifacts remain under [docs](docs/), [packaging](docs/packaging/) and [benchmarks](docs/benchmarks/). The [pre-cleanup README](https://github.com/intrepion/kata-inspired-by-pong-codex-dot/blob/024592e1b30d7a671a4ab14596a89a6558661498/README.md) retains the historical report links.

Each desktop cell is **build / runtime evidence**:

- **B**: full development build passed; **C**: check only; **U**: not verified.
- **I**: bounded visible interaction passed, not the entire acceptance checklist.
- **S**: process startup only; **R**: rendered, gameplay not verified.
- **F**: observed failure; **blocked**: a reported tool or prerequisite prevented a check.
- **n/a**: not a target for that implementation. Unit tests do not count as gameplay.

### Core frontends and game engines

| Implementation | Mac build / runtime | Windows build / runtime | Omarchy ARM64 build / runtime |
| --- | --- | --- | --- |
| Original TypeScript + Canvas browser | Updated UI not rechecked on Mac | Strict TS build + 15 tests pass; localhost normal start/pause/resume and 390×844 layout inspected at `31478c6` | Updated UI not rechecked on Omarchy |
| Rust WASM + Canvas browser | B / I | B / I; normal start/pause/resume rechecked at `31478c6`, 7 scenario/WASM tests pass | B / localhost serves; UI blocked |
| Macroquad | Fresh `7495366` B / bounded native automated full match; shell replay blocked ([details](docs/MACROQUAD-MAC-CONTACT-TIMING-RECHECK.md)) | B / R; keyboard play unverified | Fresh `617160c` B; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/MACROQUAD-C77-OMARCHY-REPLAY.md)) |
| Bevy | `173c0ef` cached B; automated desktop full match, play-again and relocated archive smoke; shell replay failed before gameplay ([details](docs/BEVY-MAC-CONTACT-TIMING-RECHECK.md)) | B / I + scripted captures **with DX12**; default Vulkan **F** ([details](docs/WINDOWS-BEVY-DIAGNOSIS.md)) | Fresh `5168fdd` B, 2 release tests; relocated package replay and six app-requested frame PNGs, exact model match; physical input unverified ([details](docs/packaging/BEVY-C77-OMARCHY-REPLAY.md)) |
| Miniquad | cached B + 8 tests; winner-label fix visually checked in automated CPU-win match and restart; earlier shell replay failed ([details](docs/MINIQUAD-MAC-CONTACT-TIMING-RECHECK.md)) | Fresh MSVC B + 1 test; scripted/six PNG reported passed; native Return/Space still no start | Fresh `ee445a7` B, 8 tests including winner geometry; relocated replay and six app-owned PNGs match model; winner visual and physical input unverified ([details](docs/packaging/MINIQUAD-WINNER-OMARCHY-PILOT.md)) |
| pixels + winit | Fresh `663aa40` B / native automated full match and restart; shell replay incomplete ([details](docs/PIXELS-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Fresh `7495366` B, 5 frontend tests; relocated package replay and six app-owned source PNGs, exact model match; physical input unverified ([details](docs/packaging/PIXELS-C77-OMARCHY-REPLAY.md)) |
| raw wgpu + winit | Fresh `617160c` B / native automated full match and restart; shell replay incomplete ([details](docs/WGPU-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Fresh `663aa40` B, 4 frontend tests; relocated package replay and six app-owned surface-readback PNGs, exact model match; physical input unverified ([details](docs/packaging/WGPU-C77-OMARCHY-REPLAY.md)) |
| Piston | Fresh `c2a4884` B / native automated full match and restart; shell replay incomplete ([details](docs/PISTON-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Fresh `38584cd` B, 7 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/PISTON-C77-OMARCHY-REPLAY.md)) |
| ggez | B / I; Mac surface fix verified | B / I on earlier snapshot | Fresh `24fbcd9` B, 2 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/GGEZ-C77-OMARCHY-REPLAY.md)) |
| nannou | `5215d8a` native automated full match; `ac99028` pause-label/resume recheck passed; earlier shell replay failed ([details](docs/NANNOU-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Fresh pause-label fix: 2 release tests, scripted frame inspection and exact model replay; physical input unverified ([details](docs/NANNOU-PAUSE-INDICATOR.md)) |
| Blue Engine | `4bab2ae` cached B; automated desktop full match/restart and relocated archive smoke; direct-shell replay failed before gameplay ([details](docs/BLUE-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Held-pause fix: 4 release tests; relocated windowed package replay matches model, native input unverified ([details](docs/packaging/BLUE-HELD-OMARCHY-PACKAGE.md); [earlier headless captures](docs/packaging/BLUE-C77-OMARCHY-REPLAY.md)) |
| raylib-rs | Fresh `99034e6` B / native automated full match and restart; shell replay incomplete ([details](docs/RAYLIB-MAC-CONTACT-TIMING-RECHECK.md)) | B / R; start input unsuccessful; LLVM build blocker resolved | Fresh `971319e` B, 3 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/RAYLIB-C77-OMARCHY-REPLAY.md)) |
| bracket-lib (RLTK) | B / bounded native I on existing contrast package; start, movement, pause, full match, restart and held-Q exit checked | B / I; separate header improves readability, bitmap quality still rough | Fresh `c691bdf` B, 9 tests; relocated X11 fallback replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/BRACKET-C77-OMARCHY-REPLAY.md)) |

### Low-level rendering and native GUI/terminal clients

| Implementation | Mac build / runtime | Windows build / runtime | Omarchy ARM64 build / runtime |
| --- | --- | --- | --- |
| softbuffer + winit | Fresh `f40c9a5` B / native automated full match and restart; shell replay incomplete ([details](docs/SOFTBUFFER-MAC-CONTACT-TIMING-RECHECK.md)); prior focus/resize checks are historical | B / I | Fresh `cf3bff1` B, 4 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/SOFTBUFFER-C77-OMARCHY-REPLAY.md)) |
| SDL3 | Fresh `082327b` B / native automated full match and restart; shell replay blocked ([details](docs/SDL3-MAC-CONTACT-TIMING-RECHECK.md)); prior focus/resize checks are historical | B / I; LNK4098 linker warning remains | Fresh `49f2c2f` B, 2 frontend + 3 shared tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/SDL3-C77-OMARCHY-REPLAY.md)) |
| Vulkan via ash | B / blocked: missing Vulkan loader/MoltenVK | B / I (simple clear-attachment renderer only) | Fresh `0cd6a1c` B, 6 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/ASH-C77-OMARCHY-REPLAY.md)) |
| Slint | `9195b63` cached debug B / automated full match and reset; shell replay timed out ([details](docs/SLINT-MAC-CONTACT-TIMING-RECHECK.md)) | B / I: start, scoring, pause/resume, reset | `ccd3783` B, 10 shared tests; relocated replay and six component PNGs match model; physical input unverified ([details](docs/packaging/SLINT-GPUI-PAUSE-OMARCHY-PILOT.md)) |
| GTK4 | Blocked: native dependencies missing / U | U / U | `59957bb` B, 1 frontend + 10 shared model tests; relocated replay and six app-owned Cairo PNGs match model; physical input unverified ([details](docs/packaging/GTK-GUI-PAUSE-OMARCHY-PILOT.md)) |
| Ratatui | B / pseudo-terminal checks; enhanced-terminal interaction unverified | B / blocked: direct launch has no targetable window; Windows Terminal access blocked by product policy | Fresh repeat-fix release B, 10 tests; relocated package replay and synthetic enhanced-key PTY pause/repeat check; real-terminal and physical input unverified ([details](docs/packaging/RATATUI-REPEAT-OMARCHY-PACKAGE.md); [earlier run](docs/packaging/RATATUI-C77-OMARCHY-REPLAY.md)) |

### Rust web UI frameworks

| Implementation / target | Mac | Windows | Omarchy |
| --- | --- | --- | --- |
| Yew browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | B / bounded localhost focus and pause check ([details](docs/YEW-LEPTOS-FOCUS-CHROME-OMARCHY.md)) |
| Leptos browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | B / bounded localhost focus and pause check ([details](docs/YEW-LEPTOS-FOCUS-CHROME-OMARCHY.md)) |
| Dioxus browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Dioxus native desktop | Fresh `4088089` B / I + native replay; held controls/release, full match and restart passed | B / I; maximized layout checked | Focus-fix debug B, 13 shared tests; relocated scripted SVG replay matches model; native input/visual unverified ([evidence](docs/packaging/DIOXUS-FOCUS-OMARCHY-PACKAGE.md); [earlier pilot](docs/packaging/DIOXUS-PAUSE-OMARCHY-PILOT.md)) |

### Mobile targets

| Implementation | Test host / target | Build and tests | Actual runtime evidence | Still outstanding |
| --- | --- | --- | --- | --- |
| Native Kotlin | Windows / Android 15 API35 emulator | Debug APK + 11 unit tests pass (latest replay candidate) | Installed; touch paddle movement, ball motion, pause/resume/restart checked | Full match, physical device, store packaging |
| Dioxus mobile | iOS / Android | U | U | Mobile setup, packaging and interaction qualification |
| Native Swift | Mac / iPhone16 Pro iOS18.4 simulator (latest) | `038f85e` Debug/Release builds + 11 core tests + 2 UI tests pass | DEBUG app replay 1440 ticks / 13 states / 6 app-owned PNGs; normal touch/release, Home pause/resume, full match and restart passed (`18bc91c`) | Physical device, store packaging |

### Acceptance coverage and remaining work

| Check or issue | Current evidence / remaining work |
| --- | --- |
| Shared simulation | Omarchy: 13 Rust core model tests pass, including scoring, restart and paddle face-crossing cases. Mac also passed core, scenario and WASM model tests and WASM compilation. Windows passed the same model and scenario tests with zero portable replay delta. The timing correction passes 19 Omarchy core model tests; prior Ratatui replay predates it. Independent Mac and Windows model review, client rebuilds, replay and physical interaction remain pending; [collision review](docs/RUST-PADDLE-FACE-COLLISION.md). |
| Adapter regressions | Windows web controller 3/3, low-level controller 3/3, Ratatui 5/5 (Omarchy current), bracket-lib 6/6, GUI support 4/4; other adapter results linked above. |
| Windows engine interaction | Most I cells mean start, scoring/ball motion, pause/resume and reset. Sustained paddle movement/release, full matches, focus/minimize and resize/DPI are mostly still unverified. |
| Windows web interaction | Browser variants exercised keyboard/pointer and CPU win; real focus-loss still unverified. Native wrappers exercised pointer movement and basic controls, not held-key/full-match/physical-touch coverage. |
| Omarchy visual/input testing | Many builds and smoke launches pass, but the automation connection exposes no native/browser surfaces. This is a tool-access blocker, not proof of a gameplay failure. |
| Known defects | Windows Bevy default Vulkan crash (DX12 rendering corrected); Windows bracket-lib bitmap text remains rough despite improved layout; Miniquad supported keys translate to Unknown ([diagnosis](docs/MINIQUAD-WINDOWS-INPUT-DIAGNOSIS.md)); raylib supported start input unsuccessful. |
| Snapshot freshness | Windows extra-engine interaction used existing isolated GNU binaries, not a fresh latest-source rebuild. The fresh Mac nannou UI check includes the pacing patch. bracket-lib contrast package header layout has also been inspected on Mac; see [bounded native check](docs/BRACKET-LIB-MAC-NATIVE-CHECK.md). Requalification remains necessary before release. |
| Packaging and compatibility | No complete signed-installer/download-install matrix, physical-mobile coverage, exhaustive OS/DPI/input matrix or accessibility audit. |
| Performance | Fourteen Mac and fifteen Omarchy native adapter pilots recorded; Windows results are listed individually in the timing matrix below. Omarchy has revised-physics release pilots for [Macroquad](docs/benchmarks/MACROQUAD-C77-OMARCHY-PILOT.md), [softbuffer](docs/benchmarks/SOFTBUFFER-C77-OMARCHY-PILOT.md), [SDL3](docs/benchmarks/SDL3-C77-OMARCHY-PILOT.md) and [Miniquad](docs/benchmarks/MINIQUAD-WINNER-OMARCHY-PILOT.md), with differing reported dimensions. Callback intervals only, not controlled cross-engine rankings or full acceptance. |

### Scripted input testing

The shared fixed-seed replay exercises start, held movement, release, pause/resume and restart. App-owned capture exports and telemetry are distinct from OS input, compositor screenshots, physical controls and smooth-motion validation. See [commands, artifact hashes and limits](docs/SCRIPTED-TESTING.md).

The following rollout table is retained as historical evidence, not a claim that every current adapter is fully qualified.

#### Shared scripted-demo rollout (Omarchy)

| Adapter | Replay | App-buffer PNG export | Evidence boundary |
|---|---|---|---|
| Macroquad | Passed | Six saved; paused sample rechecked | Shared clock refactor verified |
| softbuffer | Passed | Six saved; paused sample inspected | Exact CPU presentation buffer |
| raylib-rs | Passed | Six saved; exact tick-480 inspected | Current app backbuffer before swap |
| Pixels | Passed | Six saved; paused source frame inspected | 900x500 CPU source buffer, not scaled GPU output |
| SDL3 | Passed | Six saved; paused sample inspected | Renderer target before present |
| Bevy | Passed | Six saved; pause/reset inspected | Framework screenshot completion awaited |
| nannou | Passed | Six saved; pause/reset inspected | Framework window-frame export; pause label added in later follow-up |
| ggez | Passed | Six saved; pause/reset inspected | Same app render texture presented and captured |
| Miniquad | Passed | Six saved; paused sample inspected | Own OpenGL framebuffer; Metal capture unsupported |
| raw wgpu | Passed | Six saved; paused sample inspected | Surface COPY_SRC readback; requires capability |
| Piston | Passed | Six saved; pause/reset inspected | Same scene rendered to app-owned target, not swapchain |
| bracket-lib | Passed | Six saved; pause/reset inspected | Native framework framebuffer export |
| Blue Engine | Passed (headless) | Six saved; pause/reset inspected | Supported headless renderer only; window capture unsupported |
| Vulkan via ash | Passed | Six saved; all inspected | Acquired swapchain image copied before present |
| Ratatui | Passed (TestBackend) | No PNG: six text + six cell/style JSON exports | Cell-buffer evidence only, not terminal screenshots |
| Slint | Passed | Six saved; pause/reset inspected | Supported software-renderer component snapshot |
| GTK4 | Passed | Six saved; pause/reset inspected | Same Cairo scene on separate app-owned surface |
| Raw web-WASM | Implemented; ABI tests passed | Canvas export implemented; Windows completion verified, bundle files unverified | Not counted among nineteen verified render adapters |
| Yew / Leptos / Dioxus web | Shared replay tests/builds pass; Windows Chrome demos reached tick 1440 with 13 identical states | Six SVG links per app and final courts inspected on Windows; data-URL download blocked | SVG source is not screenshot evidence; downloaded files unverified; no Omarchy browser pass |
| Yew / Leptos native wrappers | Passed | Six SVG sources each; geometry checked | Source export, not screenshot or visual QA |
| Dioxus native | Passed | Six SVG sources; geometry checked | Source export, not final DOM presentation or screenshot |
| Original TypeScript | Portable-action tests pass; Windows bounded scripted completion verified | Canvas export link created; bundle contents unverified; Windows normal UI/390×844 inspected separately | TS physics/RNG trajectory differs from Rust; no scripted visual pass |
| Independent Swift / Kotlin | Kotlin compile + 11 JUnit tests + emulator replay pass; Swift DEBUG app replay + 11 core / 2 simulator UI tests and Debug/Release builds pass | Six app-owned PNGs each; Swift pause/reset inspected, not simulator screenshots | [Android evidence](docs/SCRIPTED-ANDROID.md), [Swift evidence](docs/evidence/swift-ios-replay-20261112); no physical-device or cross-language parity claim |
| Remaining Rust / Swift / Kotlin adapters | Not wired yet | Not wired yet | No implied pass |

## Benchmarks

Use one shared benchmark harness, with platform-specific helpers where needed. Callback intervals are not displayed FPS, GPU time or a controlled cross-engine ranking. CPU/memory/startup/power/presentation metrics are not established by these pilots. Run conditions and dimensions differ; follow each report. See [protocol and evidence](docs/benchmarks/README.md) and [architecture plan](docs/BENCHMARK-PLAN.md).

### Macroquad cross-platform pilot coverage

| Computer | Timing pilot | Package lifecycle |
| --- | --- | --- |
| Omarchy ARM64 | Three callback-timing runs; [protocol/evidence](docs/benchmarks/README.md) | Local extraction/replay/uninstall passed; [limits](docs/packaging/README.md) |
| Windows | Three valid release repeats; [evidence](docs/benchmarks/WINDOWS-MACROQUAD-PILOT.md) | Portable ZIP/extraction + scripted run passed; not installer/clean-machine/signing qualified. [Pilot](docs/packaging/MACROQUAD-WINDOWS-PILOT.md) |
| Mac ARM64 | Earlier three valid release repeats; [evidence](docs/benchmarks/MACROQUAD-MAC-PILOT.md) | Creation/extraction and bounded extracted-app GUI gameplay passed; shell launch failed; [details](docs/packaging/MACROQUAD-MAC-PILOT.md) |

### Shared callback-timing coverage by adapter

`3/3` means three valid bounded release callback-timing runs, not full gameplay acceptance. `Not recorded` means no published timing pilot in this summary, not a failed implementation.

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
| ash / Vulkan | Not recorded; Vulkan loader previously blocked | [3/3](docs/benchmarks/ASH-WINDOWS-PILOT.md) | [3/3](docs/benchmarks/ASH-ADAPTER.md) |

## Packaging

Packaging stays alongside each application until repeated common steps justify shared tooling. Local archive creation, extraction, relocated execution and scoped removal are separate from clean-machine compatibility, signed installers, physical-mobile coverage, upgrades and license clearance. No public distribution readiness is implied. See [packaging evidence and limitations](docs/packaging/README.md).

### Local portable-package lifecycle coverage

“Not recorded” is not a failure. Native input remains a separate gate.

| Adapter | Mac | Windows | Omarchy ARM64 |
| --- | --- | --- | --- |
| Macroquad | Local pilot | Local pilot | Local pilot |
| SDL3 | [Held-fix relocated replay and desktop smoke](docs/packaging/SDL3-HELD-MAC-RELOCATED.md) | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | Local pilot |
| raylib | [Held-fix relocated replay and desktop smoke](docs/packaging/RAYLIB-HELD-MAC-RELOCATED.md); [earlier pilot](docs/packaging/RAYLIB-MAC-PILOT.md) | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | Local pilot |
| ash | Loader blocked | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | Local pilot |
| Miniquad | [Local replay/capture pilot](docs/packaging/MINIQUAD-MAC-PILOT.md) | [Local ZIP/replay pilot](docs/packaging/MINIQUAD-WINDOWS-PILOT.md) | [Four-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH.md) |
| pixels, raw wgpu | [Local replay/capture pilots](docs/packaging/MAC-PIXELS-WGPU-PILOT.md) | [Local ZIP/replay pilots](docs/packaging/WINDOWS-PORTABLE-BATCH.md) | [Four-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH.md) |
| Piston, ggez | [Piston](docs/packaging/PISTON-HELD-MAC-RELOCATED.md) and [ggez](docs/packaging/GGEZ-HELD-MAC-RELOCATED.md) held-fix relocated checks; [earlier pilots](docs/packaging/MAC-PISTON-GGEZ-PILOT.md) | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | [Six-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH2.md) |
| Nannou, bracket-lib, Bevy | [Nannou](docs/packaging/NANNOU-HELD-MAC-RELOCATED.md) and [bracket-lib](docs/packaging/BRACKET-HELD-MAC-RELOCATED.md) held-fix relocated checks; [earlier pilots](docs/packaging/MAC-FOUR-ENGINE-PILOT.md) | [Local ZIP/replay pilots; Bevy DX12](docs/packaging/WINDOWS-PORTABLE-BATCH3.md) | [Six-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH2.md) |
| Blue Engine, softbuffer | [Blue held-fix relocated replay and desktop smoke](docs/packaging/BLUE-HELD-MAC-RELOCATED.md); [softbuffer held-fix relocated check](docs/packaging/SOFTBUFFER-HELD-MAC-RELOCATED.md); [earlier pilot](docs/packaging/MAC-SOFTBUFFER-BLUE-PILOT.md) | [Blue state-only](docs/packaging/WINDOWS-PORTABLE-BATCH3.md); [softbuffer replay/capture](docs/packaging/WINDOWS-PORTABLE-BATCH.md) | [Final two local pilots](docs/packaging/LINUX-PORTABLE-BATCH3.md); Blue native captures unsupported |
| Ratatui | [Release archive, replay and synthetic PTY checks](docs/packaging/RATATUI-MAC-RELEASE-PILOT.md); [earlier debug pilot](docs/packaging/RATATUI-MAC-DEBUG-PILOT.md) | [Local ZIP/cell replay only](docs/packaging/RATATUI-WINDOWS-PILOT.md) | [Terminal local pilot](docs/packaging/RATATUI-LINUX-PILOT.md); cells + bounded PTY, not screenshots |
| GTK4 | Not recorded | Not recorded | [Local package pilot](docs/packaging/GTK-LINUX-PILOT.md); existing GTK4 libraries required |
| Slint | [Local debug archive and desktop smoke](docs/packaging/SLINT-MAC-DEBUG-PILOT.md) | [Local ZIP/own-window snapshots](docs/packaging/SLINT-WINDOWS-PILOT.md) | [Local package pilot](docs/packaging/SLINT-LINUX-PILOT.md); own-window snapshots, license text included |
| Dioxus Desktop | Not recorded | Not recorded | [Local debug archive/relocated replay](docs/packaging/DIOXUS-PAUSE-OMARCHY-PILOT.md); WebKitGTK and libxdo required, notices pending |
| Other adapters | Not recorded | Not recorded | Not recorded |
