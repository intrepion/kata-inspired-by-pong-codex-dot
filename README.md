# Rallylight

An original real-time paddle game inspired by Pong. Return the ball, steer shots with your paddle, and beat the computer to seven points. The computer has a short reaction delay and a limited paddle speed, so well-placed shots can get past it. This independent kata is not affiliated with or endorsed by Atari.

Android held-touch pause and restart fixes: 16 model/unit tests pass and a debug APK builds; simultaneous device input remains unverified. [Evidence](docs/ANDROID-HELD-TOUCH-PAUSE-FIX.md).

Windows Blue held-pause recheck: four tests and a 13-checkpoint model-matched scripted run pass; native input remains open. [Evidence](docs/WINDOWS-BLUE-CONTACT-TIMING.md).

Windows Chrome narrow layout: focused movement buttons moved the paddle with Enter and Space; physical and assistive input remain open. [Evidence](docs/ORIGINAL-MOVEMENT-BUTTON-KEYS.md).

Windows original browser recheck: 24 JavaScript tests pass, collision math reviewed, and localhost Chrome scripted mode reaches its export link; blob download and physical input remain open. [Evidence](docs/WINDOWS-ORIGINAL-COLLISION-INPUT-RECHECK.md).

Windows Blue Engine held-input release ZIP: local extraction, payload hashes and 13-checkpoint relocated replay pass; distribution and physical input remain open. [Evidence](docs/packaging/BLUE-HELD-WINDOWS-RELEASE-PILOT.md).

Windows Blue Engine notice discovery: 202 resolved packages indexed; exact upstream mappings addressed two flags, with 12 still open and legal review incomplete. [Evidence](docs/packaging/BLUE-WINDOWS-NOTICE-DISCOVERY.md).

Windows Ratatui enhanced-repeat release ZIP: local extraction and 13-checkpoint relocated TestBackend replay pass with a reused notice inventory; terminal and distribution checks remain open. [Evidence](docs/packaging/RATATUI-REPEAT-WINDOWS-RELEASE-PILOT.md).

Windows Yew release WASM: 13 controller tests and localhost Chrome first-click Pause and in-page focus-loss checks pass; physical input and native-wrapper focus remain open. [Evidence](docs/WINDOWS-YEW-FOCUS-PAUSE-RECHECK.md).

Windows Leptos and Dioxus release WASM: localhost Chrome first-click Pause and in-page focus-loss checks pass; native-wrapper focus and physical input remain open. [Evidence](docs/WINDOWS-LEPTOS-DIOXUS-FOCUS-RECHECK.md).

Windows-built Yew release static ZIP: extracted hashes, localhost Chrome Start/Pause/Resume and SVG icon fetch pass; dependency notices and distribution remain open. [Evidence](docs/packaging/YEW-FOCUS-WINDOWS-STATIC-PILOT.md).

Swift iOS versioned Release simulator smoke and unsigned generic device build pass; explicit Combine import clears its prior warning. Device run, signing and distribution remain open. [Evidence](docs/SWIFT-HELD-TOUCH-PAUSE-FIX.md).

Windows Miniquad held-input release ZIP: ten tests, local extraction and model-matched relocated replay pass; actual key delivery and distribution remain open. [Evidence](docs/packaging/MINIQUAD-HELD-WINDOWS-RELEASE-PILOT.md).

Windows SDL3 held-input debug ZIP: corrected notice scan, local extraction and model-matched scripted replay pass; release build, runtime warning and legal review remain open. [Evidence](docs/packaging/SDL3-WINDOWS-HELD-NOTICE-PILOT.md).

Windows Softbuffer held-input release ZIP: six tests, local extraction and model-matched relocated replay pass; native input and distribution remain open. [Evidence](docs/packaging/SOFTBUFFER-HELD-WINDOWS-RELEASE-PILOT.md).

Windows Piston held-input release ZIP: nine tests, local extraction and model-matched relocated replay pass; ten notice flags and native input remain open. [Evidence](docs/packaging/PISTON-HELD-WINDOWS-RELEASE-PILOT.md).

Windows ash held-input release ZIP: eight tests, local extraction and model-matched relocated replay pass; native input and distribution remain open. [Evidence](docs/packaging/ASH-HELD-WINDOWS-RELEASE-PILOT.md).

Windows bracket-lib release portable pilot: 11 tests passed and relocated scripted telemetry matched all 13 reference records; seven dependency notice flags and physical-input, accessibility, and clean-machine checks remain open. See [evidence](docs/packaging/BRACKET-HELD-WINDOWS-RELEASE-PILOT.md).

Windows ggez release portable pilot: four tests passed and relocated scripted telemetry matched all 13 reference records; 14 dependency notice flags and physical-input, accessibility, and clean-machine checks remain open. See [evidence](docs/packaging/GGEZ-HELD-WINDOWS-RELEASE-PILOT.md).

Windows Nannou release portable pilot: five tests passed and relocated scripted telemetry matched all 13 reference records; 31 dependency notice flags and physical-input, accessibility, and clean-machine checks remain open. See [evidence](docs/packaging/NANNOU-HELD-WINDOWS-RELEASE-PILOT.md).

Windows Fyrox debug replay from a dedicated writable directory matched all 13 reference records and created its registry there; generated state was preserved. See [evidence](docs/packaging/FYROX-DEDICATED-CWD-WINDOWS.md).

Windows ggez release runs from a separate working directory; a task-local USERPROFILE override prevented known-folder initialization on this host. See [bounded environment check](docs/packaging/GGEZ-ISOLATED-HOME-WINDOWS.md).

Windows ggez profile follow-up: a native known-folder probe recorded missing-path HRESULTs under the artificial USERPROFILE override; see [bounded diagnostics](docs/packaging/GGEZ-KNOWN-FOLDER-WINDOWS.md).

Windows raylib held-pause debug pilot: relocated replay matched 13 records and carries a Windows-filtered notice inventory; the debug runtime warning and release build remain open. See [evidence](docs/packaging/RAYLIB-HELD-WINDOWS-DEBUG-NOTICE-PILOT.md).

Notice builder now rejects malformed platform options instead of silently defaulting; prior mislabeled excluded bundles remain preserved. See [validation](docs/packaging/NOTICE-PLATFORM-OPTIONS.md).

Windows PE import inspection found a direct VCRUNTIME140.dll dependency in the ggez and Nannou release pilots and raylib debug pilot; [clean-machine runtime availability remains unverified](docs/packaging/WINDOWS-PE-IMPORTS-PILOTS.md).

Windows dependency discovery found Slint GPL-or-alternative declarations and an SDL_mixer bundled LGPL notice requiring review; [scope and feature limits](docs/packaging/WINDOWS-COPYLEFT-DISCOVERY.md).

## Current activity (last reported)

**Snapshot updated: 2026-11-12 13:24 UTC.** This is a committed progress snapshot, **not live process telemetry**. Refreshing this checkpoint branch shows the latest published report, not an automatic view of the computers. Activity is updated at meaningful starts, completions, pauses, and blockers; a row can become stale between updates.

| Computer | Last reported activity | Phase / next step |
| --- | --- | --- |
| Windows | Original TS passes 17 tests and Chrome start/pause smoke. Rebuilt Yew, Leptos and Dioxus pass single-mouse up/down and interior release checks; shared controller passes 11 tests. GPUI geometry/model and replay checks pass. | [Web pointer evidence](docs/WEB-INPUT-SOURCES-FIX.md); [original pointer follow-up](docs/ORIGINAL-TS-POINTER-BUTTONS-FIX.md); [GPUI evidence](docs/GPUI-COURT-ASPECT-FIX.md). Browser multi-touch and GPUI native resize/input remain open. |
| Mac | ggez existing package passed native movement, pause/resume, full match to 0–7 Over, Enter restart to 0–0 Paused, and menu Quit with confirmed process exit. Actual window inspected. | [ggez native gameplay](docs/GGEZ-MAC-NATIVE-GAMEPLAY.md). Focus/resize/physical key overlap remain separate. No rebuild or active process; GPUI corrected renderer qualification still pending. |
| Omarchy ARM64 | Original TypeScript pointer buttons now retain each finger independently: opposite directions cancel and releasing one finger preserves other held controls. Strict build and all seventeen tests pass. | [Original pointer-button regression](docs/ORIGINAL-TS-POINTER-BUTTONS-FIX.md). Browser/device multi-touch qualification remains pending; generated JavaScript updated from TypeScript. |

The testing matrices below record **verified results**, separately from work in progress. An activity row does not imply that an application is currently open, that every check passed, or that a release is ready.

Mac Ratatui repeat-fix follow-up: ten debug tests, TestBackend replay and synthetic enhanced/fallback PTY checks pass; real-terminal and physical input remain open. [Model evidence](docs/RATATUI-HELD-REPEAT-FIX.md); [PTY checks](docs/packaging/RATATUI-PTY-MAC-HELD.md).

Mac Fyrox and Blue Engine held-pause follow-up: four tests each and sequential automated movement/pause/resume pass; native held-across-pause remains open. [Fyrox](docs/FYROX-HELD-PAUSE-FIX.md); [Blue Engine](docs/BLUE-HELD-PAUSE-FIX.md).

Mac Godot held-pause follow-up: integration and synthetic held-input smoke pass; sequential automated native pause/resume and fresh movement pass. [Evidence](docs/GODOT-HELD-PAUSE-FIX.md).

Mac Godot resize/focus attempt: gameplay opened but the resize and focus transitions were not established; [limitations and log replacement](docs/GODOT-MAC-FOCUS-RESIZE-ATTEMPT.md).

Mac bracket-lib pause-fix follow-up: eleven release tests and sequential automated movement/pause/resume pass; native held-across-pause remains open. [Evidence](docs/BRACKET-PAUSE-HELD-INPUT-FIX.md).

Mac SDL3 pause-fix follow-up: four release tests and sequential automated movement/pause/resume pass; native held-across-pause remains open. [Evidence](docs/SDL3-PAUSE-HELD-INPUT-FIX.md).

Mac raylib pause-fix follow-up: five release tests and sequential automated movement/pause/resume pass after a sustained start press; shorter Return actions did not start in this run. [Evidence](docs/RAYLIB-PAUSE-HELD-INPUT-FIX.md).

Mac Nannou pause-fix follow-up: five release tests and sequential automated movement/pause/resume pass; native held-across-pause remains open. [Evidence](docs/NANNOU-PAUSE-HELD-INPUT-FIX.md).

Mac Godot pause-prompt fix: headless integration smoke passes and automated desktop pause/resume shows and clears the unobscured prompt. [Evidence](docs/GODOT-MAC-CONTACT-TIMING-ATTEMPT.md).

Mac Pixels, wgpu and softbuffer pause follow-up: 7/6/6 release tests and sequential automated movement/pause/resume checks pass; native held-across-pause remains open. [Pixels/wgpu](docs/PIXELS-WGPU-PAUSE-HELD-INPUT-FIX.md); [softbuffer/ash](docs/SOFTBUFFER-ASH-PAUSE-HELD-INPUT-FIX.md).

Dioxus local-libxdo launcher now rejects package paths containing loader separators; relocated release replay still passes. [Evidence](docs/packaging/DIOXUS-LIBRARY-PATH-SEPARATOR-FIX.md).

Omarchy Dioxus local-libxdo release pilot: locked release build, 13 controller tests and relocated model-matched replay pass; native input and distribution remain open. [Evidence](docs/packaging/DIOXUS-RELEASE-XDO-OMARCHY.md).

Omarchy Dioxus local-libxdo debug pilot: a package-local launcher runs model-matched replay without a pre-set library path; other dependencies, native input and distribution remain open. [Evidence](docs/packaging/DIOXUS-LOCAL-XDO-OMARCHY.md).

Omarchy Dioxus desktop non-writable launch: missing host libxdo blocks startup; with the documented local library, replay passes and WebKit state is observed outside the package. [Evidence](docs/packaging/DIOXUS-READONLY-INSTALL-OMARCHY.md).

Omarchy Blue Engine non-writable package launch: state-only replay matches the model; graphics warnings, physical input and clean-machine installation remain open. [Evidence](docs/packaging/BLUE-READONLY-INSTALL-OMARCHY.md).

Omarchy Fyrox dedicated working directory: the non-writable package ran its replay without error-level logs and wrote its registry in the dedicated folder; package launch instructions corrected. [Evidence](docs/packaging/FYROX-DEDICATED-CWD-OMARCHY.md).

Omarchy Fyrox non-writable launch: replay finishes but registry writes log permission errors; package guidance now recommends a dedicated writable working directory. [Evidence](docs/packaging/FYROX-READONLY-INSTALL-OMARCHY.md).

Omarchy SDL3 read-only package launch: scripted replay passes with a non-writable package directory and isolated home/XDG paths; clean-machine and physical-input checks remain open. [Evidence](docs/packaging/SDL3-READONLY-INSTALL-OMARCHY.md).

Omarchy ggez isolated-directory launch: the relocated release replay passes with separate home/XDG directories; Mesa cache files were observed and retained. [Evidence](docs/packaging/GGEZ-ISOLATED-HOME-OMARCHY.md).

Omarchy Fyrox held-pause release pilot: relocated replay matches the model; the app created working-directory state and future package guidance now flags it; physical input, removal and notices remain open. [Evidence](docs/packaging/FYROX-HELD-OMARCHY-RELEASE-PILOT.md).

Omarchy Ratatui fallback PTY: synthetic character input verifies latched movement, stop and pause clearing; real terminal and physical input remain open. [Evidence](docs/packaging/RATATUI-FALLBACK-PTY-OMARCHY.md).

Omarchy SDL3 held-pause notice pilot: corrected target scan and six relocated manifest checks pass; native input and full notice review remain open. [Evidence](docs/packaging/SDL3-HELD-NOTICE-OMARCHY.md).

Omarchy bracket-lib held-pause release pilot: relocated X11 fallback replay matches the model; physical input, native Wayland and dependency notices remain open. [Evidence](docs/packaging/BRACKET-HELD-OMARCHY-RELEASE-PILOT.md).

Omarchy ggez held-pause release pilot: relocated archive checks and model-matched replay pass; physical input and dependency notices remain open. [Evidence](docs/packaging/GGEZ-HELD-OMARCHY-RELEASE-PILOT.md).

Omarchy Piston held-pause release pilot: relocated archive checks and model-matched replay pass; physical input and dependency notices remain open. [Evidence](docs/packaging/PISTON-HELD-OMARCHY-RELEASE-PILOT.md).

Omarchy Nannou held-pause release pilot: relocated archive checks and model-matched replay pass; physical input and dependency notices remain open. [Evidence](docs/packaging/NANNOU-HELD-OMARCHY-RELEASE-PILOT.md).

Omarchy raylib held-pause release pilot: notice-bearing archive checks and relocated scripted replay pass; physical input and full notice review remain open. [Evidence](docs/packaging/RAYLIB-HELD-NOTICE-OMARCHY.md).

Omarchy SDL3 held-pause release pilot: relocated archive checks and model-matched replay pass; physical input and dependency notices remain open. [Evidence](docs/packaging/SDL3-HELD-OMARCHY-RELEASE-PILOT.md).

Omarchy Miniquad winner-fix release pilot: notice-bearing archive checks and relocated scripted replay pass; notice review, winner visuals and physical input remain open. [Evidence](docs/packaging/MINIQUAD-WINNER-NOTICE-OMARCHY.md).

Omarchy Ratatui focus and resize PTY: synthetic focus loss and a PTY resize paused the release executable and cleared movement; real terminal and physical input remain open. [Evidence](docs/packaging/RATATUI-FOCUS-RESIZE-PTY-OMARCHY.md).

Omarchy Ratatui enhanced-key PTY: the relocated release process passed a synthetic press/pause/repeat/release check; real-terminal and physical-input behavior remain open. [Evidence](docs/packaging/RATATUI-ENHANCED-PTY-OMARCHY.md).

Omarchy Ratatui release pilot now includes a resolved-graph notice discovery bundle; relocated replay and six manifest entries verify, with notice review and clean-machine installation pending. [Evidence](docs/packaging/RATATUI-RELEASE-NOTICE-OMARCHY.md).

Omarchy tab-loss check was inconclusive because the automation reported both pages visible; no unverified focus-listener changes were adopted. [Details](docs/WEB-TAB-FOCUS-ATTEMPT.md).

Dioxus focus/pause fix: a fresh localhost Chrome build reproduced and corrected the first-click failure; desktop scripted replay passes, with native interaction pending. [Evidence](docs/DIOXUS-INTERNAL-FOCUS-PAUSE-FIX.md).

Leptos focus/pause fix: rebuilt localhost Chrome check passes internal and outside-page focus transitions; native wrapper and physical input remain pending. [Evidence](docs/YEW-LEPTOS-FOCUS-CHROME-OMARCHY.md).

Yew focus/pause follow-up: localhost Chrome exposed and verified a further outside-focus correction; native wrapper and physical input remain pending. [Evidence](docs/YEW-LEPTOS-FOCUS-CHROME-OMARCHY.md).

Dependency notice preparation now rejects inconsistent resolved Cargo graphs before indexing; this is not legal clearance. [Evidence](docs/packaging/NOTICE-GRAPH-INTEGRITY.md).

Original browser canvas drag now retains first-pointer ownership; mocked multi-pointer tests pass, with physical multi-touch pending. [Evidence](docs/ORIGINAL-CANVAS-POINTER-OWNER.md).

Original browser live-status updates now occur only when text changes; mocked DOM regression passes, with real assistive-technology checks pending. [Evidence](docs/ORIGINAL-LIVE-STATUS-FIX.md).

Original TypeScript collision timing fix: 23 model/handler tests pass; a separate before/after deterministic replay records expected trajectory changes. Real browser collision interaction remains pending. [Evidence](docs/ORIGINAL-COLLISION-TIMING-FIX.md).

Benchmark runner no longer assumes supplied binaries are release builds; profile defaults to unverified and can be supplied explicitly. [Evidence](docs/benchmarks/PROFILE-PROVENANCE-FIX.md).

Benchmark publication now revalidates runs marked valid against execution and measurement rules; existing private data remains local. [Evidence](docs/benchmarks/PUBLIC-VALIDITY-GUARD.md).

Linux portable packager now rejects mismatched or invalid ELF architecture before output creation; local packaging regressions pass. [Evidence](docs/packaging/LINUX-ELF-ARCH-GUARD.md).

Original browser movement buttons now respond to zero-detail synthesized clicks in mocked tests; browser/assistive-technology validation remains pending. [Evidence](docs/ORIGINAL-MOVEMENT-BUTTON-CLICK.md).

Original browser movement-button keyboard fix: Space/Enter hold and release are covered by mocked handlers; browser and assistive-technology checks remain pending. [Evidence](docs/ORIGINAL-MOVEMENT-BUTTON-KEYS.md).

Original browser keyboard follow-up: focused Start/Pause buttons activate with Space in Windows localhost Chrome; mocked-handler tests also cover the in-game shortcut. Physical input remains open. [Evidence](docs/ORIGINAL-BUTTON-SPACE-FIX.md).

Original browser pause-input fix: 19 JavaScript model and mocked-handler tests pass; Windows localhost Chrome start, pause, resume and W movement smoke pass. Held-through-pause and physical touch remain open. [Evidence](docs/ORIGINAL-PAUSE-INPUT-FIX.md).

Held-input audit follow-up: Swift retained-touch movement was reproduced at model level and guarded; 14 Swift tests in debug and release, iOS Simulator debug/release builds, Release start/pause/resume UI smoke and debug paused-drag/fresh-gesture checks pass; overlapping simulator touches remain unverified. Android remains a separate source-review concern. [Coverage](docs/HELD-INPUT-COVERAGE-20261113.md); [Swift fix and limits](docs/SWIFT-HELD-TOUCH-PAUSE-FIX.md).

Omarchy Ratatui enhanced-key repeat fix: ten release tests and model-matched terminal-cell replay pass; actual terminal input remains open. [Evidence](docs/RATATUI-HELD-REPEAT-FIX.md).

Omarchy Blue Engine held-pause fix: four release tests and model-matched windowed replay pass; native input remains open. [Evidence](docs/BLUE-HELD-PAUSE-FIX.md).

Omarchy Fyrox held-pause fix: four release tests and a model-matched scripted replay pass; physical input remains open. [Evidence](docs/FYROX-HELD-PAUSE-FIX.md).

Omarchy Godot held-pause fix: synthetic adapter events cover four movement keys, three pause keys and fresh alias presses; headless smoke and scripted replay pass. Native and physical input remain open. [Evidence](docs/GODOT-HELD-PAUSE-FIX.md).

Mac Slint shared pause follow-up: ten controller tests and sequential automated movement/pause/resume pass; held-across-pause native input remains open. [Evidence](docs/GUI-EXPLICIT-PAUSE-INPUT-FIX.md).

Mac Miniquad pause follow-up: ten release tests pass; sequential desktop start/move/pause/resume/fresh movement checked. Held-across-pause native delivery remains open. [Evidence](docs/MINIQUAD-PAUSE-HELD-INPUT-FIX.md).

Windows bracket-lib held-input pause follow-up: 11 action/model tests and fresh scripted replay pass; native held-key input remains open. [Evidence](docs/WINDOWS-BRACKET-HELD-PAUSE-RECHECK.md).

Windows Godot headless integration and synthetic held-input checks pass with a fresh isolated build; graphical, native input and packaging checks remain open. [Details](docs/WINDOWS-GODOT-PAUSE-ATTEMPT.md).

Windows raylib/SDL3 held-input pause follow-up: five and four model tests plus fresh scripted replays pass; native input and ordinary debug runtime warnings remain open. [Evidence](docs/WINDOWS-RAYLIB-SDL3-HELD-PAUSE-RECHECK.md).

Windows Nannou held-input pause follow-up: five polled-input tests and a fresh scripted replay pass; native held-key input remains open. [Evidence](docs/WINDOWS-NANNOU-HELD-PAUSE-RECHECK.md).

Windows Piston pause-fix follow-up: nine action/clock tests and a fresh scripted replay pass; native held-key input remains open. [Evidence](docs/WINDOWS-PISTON-PAUSE-RECHECK.md).

Windows ggez pause-fix follow-up: four action/model tests and fresh scripted replay pass; native held-key input remains open. [Evidence](docs/WINDOWS-GGEZ-PAUSE-RECHECK.md).

Windows Softbuffer/ash pause-fix follow-up: six and eight action/model tests plus fresh scripted replays pass; native held-key input remains open. [Evidence](docs/WINDOWS-SOFTBUFFER-ASH-PAUSE-RECHECK.md).

Windows Pixels/raw wgpu pause-fix follow-up: seven and six action/model tests plus fresh scripted replays pass; native held-key input remains open. [Evidence](docs/WINDOWS-PIXELS-WGPU-PAUSE-RECHECK.md).

Windows GUI pause-fix follow-up: 10 shared model tests and fresh Slint/GPUI scripted runs pass; native held-key delivery remains open. [Evidence](docs/WINDOWS-GUI-EXPLICIT-PAUSE-RECHECK.md).

Windows Miniquad pause-fix follow-up: 10 action/model tests and fresh scripted replay pass; native key delivery remains open. [Evidence](docs/WINDOWS-MINIQUAD-PAUSE-RECHECK.md). Windows Dioxus Desktop scripted export completed under approved host execution after restricted attempts stalled; actual window input remains open. [Details](docs/WINDOWS-DIOXUS-NATIVE-PAUSE-ATTEMPT.md).

Miniquad keypad Enter now shares the existing start/restart action; Windows and Mac action-boundary tests pass; Mac automated `KP_Enter` starts and resets, but its delivered key code, physical delivery and the separate Windows Unknown-key issue remain open. [Evidence](docs/MINIQUAD-KEYPAD-ENTER.md).

bracket-lib explicit-pause fix: eleven adapter tests pass, including held-key and repeat suppression; scripted replay matches the model. Native input, packages and other platforms remain pending. [Evidence](docs/BRACKET-PAUSE-HELD-INPUT-FIX.md).

SDL3 explicit-pause fix: four adapter tests pass, including scancode held-key suppression; scripted replay matches the model. Native input, snapshot/focus behavior, packages and other platforms remain pending. [Evidence](docs/SDL3-PAUSE-HELD-INPUT-FIX.md).

raylib-rs explicit-pause fix: five adapter tests pass, including polled held-key suppression; scripted replay matches the model. Native input, focus changes, packages and other platforms remain pending. [Evidence](docs/RAYLIB-PAUSE-HELD-INPUT-FIX.md).

Nannou explicit-pause fix: five adapter tests pass, including polled held-key suppression; scripted replay matches the model. Native input, focus changes, packages and other platforms remain pending. [Evidence](docs/NANNOU-PAUSE-HELD-INPUT-FIX.md).

Piston explicit-pause fix: nine adapter tests pass, including Space/Escape held-input and repeat regressions; scripted replay matches the model. Mac cached build and sequential automated pause/resume check pass; held-across-pause native input remains pending. [Evidence](docs/PISTON-PAUSE-HELD-INPUT-FIX.md).

ggez explicit-pause fix: four adapter tests pass, including Space/Escape held-input and repeat regressions; scripted replay matches the model. Mac cached release build, four tests and sequential automated pause/resume check pass; held-across-pause native input remains pending. [Evidence](docs/GGEZ-PAUSE-HELD-INPUT-FIX.md).

Softbuffer/ash explicit-pause fix: six and eight adapter tests pass, including held-input and repeat regressions; scripted replays match the model. Native input, packages and other platforms remain pending. [Evidence](docs/SOFTBUFFER-ASH-PAUSE-HELD-INPUT-FIX.md).

Pixels/raw wgpu explicit-pause fix: seven and six adapter tests pass, including held-input and repeat regressions; scripted replays match the model. Native input, packages and other platforms remain pending. [Evidence](docs/PIXELS-WGPU-PAUSE-HELD-INPUT-FIX.md).

Shared GTK/Slint/GPUI explicit-pause fix: ten model tests and Omarchy compilation checks pass; refreshed package replays passed for [GTK](docs/packaging/GTK-GUI-PAUSE-OMARCHY-PILOT.md) and [Slint/GPUI](docs/packaging/SLINT-GPUI-PAUSE-OMARCHY-PILOT.md). Native event delivery remains pending. [Fix evidence](docs/GUI-EXPLICIT-PAUSE-INPUT-FIX.md).

Omarchy Miniquad explicit-pause fix: ten adapter tests pass, including stale held input and repeat suppression; native input and other-platform/package rechecks remain pending. [Evidence](docs/MINIQUAD-PAUSE-HELD-INPUT-FIX.md).

Windows Miniquad winner-label follow-up: eight tests and a fresh replay pass; a simulated match reaches an unobstructed CPU winner frame; native input and interactive matches remain unverified. [Evidence](docs/WINDOWS-MINIQUAD-WINNER-RECHECK.md).

Windows WebView2 scripted shutdown probe: two original exits logged the unregister error, while two isolated window-close probe exits did not; normal and cross-platform behavior remain open. [Evidence](docs/WINDOWS-WEBVIEW2-SHUTDOWN-PROBE.md).

Windows Leptos native pause-fix follow-up: fresh native scripted state/SVG export matches the model; native input and the recurring WebView2 exit diagnostic remain open. [Evidence](docs/WINDOWS-LEPTOS-NATIVE-PAUSE-RECHECK.md).

Windows Yew native pause-fix follow-up: two bridge tests and scripted state/SVG export pass; native input and WebView2 exit diagnostic remain open. [Evidence](docs/WINDOWS-YEW-NATIVE-PAUSE-RECHECK.md).

Windows web pause follow-up: 12 controller tests pass; rebuilt Yew, Leptos and Dioxus show pause/resume and fresh pointer movement in localhost Chrome. Held-across-pause remains model-only. [Evidence](docs/WINDOWS-WEB-EXPLICIT-PAUSE-RECHECK.md).

Windows Tauri contact-timing follow-up: the missing Windows icon is supplied; fresh native tests and scripted Canvas replay pass, with WebView2 exit diagnostic noted. [Evidence](docs/WINDOWS-TAURI-CONTACT-TIMING.md).

Windows Fyrox held-pause follow-up: four adapter/benchmark tests and a 13-checkpoint scripted replay pass; native input remains unverified. [Evidence](docs/WINDOWS-FYROX-CONTACT-TIMING.md).

Windows Blue Engine contact-timing follow-up: two frontend tests and windowed/headless scripted replays pass; a headless frame was inspected, while native input remains open. [Evidence](docs/WINDOWS-BLUE-CONTACT-TIMING.md).

Windows Ratatui MSVC notice-bearing debug ZIP: all five extracted files match their staged hashes; notice review and release qualification remain open. [Evidence](docs/packaging/RATATUI-WINDOWS-MSVC-NOTICE-PILOT.md).

Windows Ratatui MSVC notice discovery: the manifest now declares the existing project MIT license; all resolved packages have discovered notice files, subject to further review. [Scope](docs/packaging/RATATUI-WINDOWS-MSVC-NOTICE-DISCOVERY.md).

Windows Ratatui local debug ZIP: extracted files match their staged hashes and the relocated scripted TestBackend replay matches the current model; terminal and release qualification remain open. [Evidence](docs/packaging/RATATUI-C77-WINDOWS-DEBUG-PILOT.md).

Windows Ratatui enhanced-repeat follow-up: ten tests and scripted TestBackend replay pass, with all 13 model checkpoints matched; real terminal interaction remains open. [Evidence](docs/WINDOWS-RATATUI-CONTACT-TIMING.md).

Windows Slint contact-timing follow-up: a fresh scripted replay matches all 13 model checkpoints and a paused component capture was inspected; normal native and physical input remain open. [Evidence](docs/WINDOWS-SLINT-CONTACT-TIMING.md).

Windows GPUI contact-timing follow-up: the geometry and shared model tests pass, and a fresh scripted state replay matches all 13 reference checkpoints; native input and visual checks remain open. [Evidence](docs/WINDOWS-GPUI-CONTACT-TIMING.md).

Windows Bevy contact-timing follow-up: a fresh DX12 scripted replay matches all 13 model checkpoints; native and physical input remain open. [Evidence](docs/WINDOWS-BEVY-CONTACT-TIMING.md).

Windows raw WASM contact-timing follow-up: a fresh release WASM build and two ABI tests pass; localhost Chrome normal start/pause/resume was checked. [Scope](docs/WINDOWS-RAW-WASM-CONTACT-TIMING.md).

Windows nannou pause-indicator recheck: fresh model replay and separate automated native pause/resume display checks pass; sustained and physical input remain open. [Evidence](docs/WINDOWS-NANNOU-CONTACT-TIMING.md).

Windows nannou contact-timing follow-up: two adapter tests and a fresh scripted replay pass; the tick 480 app-owned frame was inspected. A missing Vulkan overlay-layer manifest was logged, and native input remains unverified. [Evidence](docs/WINDOWS-NANNOU-CONTACT-TIMING.md).

Windows Vulkan/ash contact-timing follow-up: six adapter/model tests and a fresh scripted replay pass; a separate native start/pause/restart/exit smoke check passed. Sustained and physical input remain unverified. [Evidence](docs/WINDOWS-ASH-CONTACT-TIMING.md).

Windows bracket-lib contact-timing follow-up: nine adapter/model tests and a fresh scripted replay pass; the paused app-owned frame was inspected. Normal input remains unverified. [Evidence](docs/WINDOWS-BRACKET-CONTACT-TIMING.md). Raylib's debug runtime mismatch was independently confirmed; an explicit matching-runtime probe passed, but ordinary Cargo remains unresolved. [Diagnosis](docs/WINDOWS-RAYLIB-CONTACT-TIMING.md).

Windows raylib contact-timing follow-up: three adapter tests and a fresh scripted replay pass; the paused app-owned frame was inspected. The debug link emitted `LNK4098`; native input remains unverified. [Evidence](docs/WINDOWS-RAYLIB-CONTACT-TIMING.md).

Windows ggez contact-timing follow-up: two adapter tests, a fresh scripted replay and bounded native start/score/pause/restart/close checks pass. Sustained and physical input remain unverified. [Evidence](docs/WINDOWS-GGEZ-CONTACT-TIMING.md).

Windows Piston contact-timing follow-up: seven adapter/clock tests and a fresh scripted replay pass; the paused app-owned frame was inspected. Normal input remains unverified. [Evidence](docs/WINDOWS-PISTON-CONTACT-TIMING.md).

Windows Macroquad contact-timing follow-up: fresh scripted telemetry matches the model reference and the paused app-owned frame was inspected; normal input remains unverified. [Evidence](docs/WINDOWS-MACROQUAD-CONTACT-TIMING.md). SDL3's ordinary debug runtime warning remains open; a cross-platform profile workaround was evaluated but not adopted. [Diagnosis](docs/WINDOWS-SDL3-CONTACT-TIMING.md).

Windows SDL3 contact-timing follow-up: two adapter tests and a fresh scripted replay pass; the paused app-owned frame was inspected. An opt-in MSVC build path selects the matching runtime and links without the original debug `LNK4098` warning; native input remains unverified. [Evidence](docs/WINDOWS-SDL3-CONTACT-TIMING.md).

Windows Miniquad contact-timing follow-up: seven adapter/model tests and a fresh scripted replay pass; the paused app-owned frame was inspected. The prior supported-key delivery limitation remains unresolved. [Evidence](docs/WINDOWS-MINIQUAD-CONTACT-TIMING.md).

Windows Pixels contact-timing follow-up: fresh scripted telemetry matches the model reference and six app-owned source frames were inspected. Native interaction remains unverified after an approval timeout. [Evidence](docs/WINDOWS-PIXELS-CONTACT-TIMING.md).

Windows raw wgpu contact-timing follow-up: fresh scripted telemetry matches the model reference and the paused app-owned frame was inspected; normal input remains unverified. [Evidence](docs/WINDOWS-WGPU-CONTACT-TIMING.md).

Windows Softbuffer contact-timing follow-up: fresh native scripted telemetry matches the revised model reference; normal start, score, pause, restart and exit were observed. [Evidence and limits](docs/WINDOWS-SOFTBUFFER-CONTACT-TIMING.md).

Windows contact-timing follow-up at `c77bf3a`: 19 core, 8 scenario and 1 WASM model tests pass; rebuilt Yew passed a bounded localhost mouse smoke check. [Scope](docs/RUST-PADDLE-FACE-COLLISION.md).

Windows collision-revision follow-up: Yew, Leptos and Dioxus rebuilt at `9cee3c3` and completed localhost scripted replays; a separate normal Yew mouse smoke check also passed. [Scope and limits](docs/WINDOWS-WEB-COLLISION-RECHECK.md).

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
| Macroquad | Fresh `7495366` B / bounded native automated full match; shell replay blocked ([details](docs/MACROQUAD-MAC-CONTACT-TIMING-RECHECK.md)) | B / R; keyboard play unverified | Fresh `617160c` B; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/MACROQUAD-C77-OMARCHY-REPLAY.md)) |
| Bevy | `173c0ef` cached B; automated desktop full match, play-again and relocated archive smoke; shell replay failed before gameplay ([details](docs/BEVY-MAC-CONTACT-TIMING-RECHECK.md)) | B / I + scripted captures **with DX12**; default Vulkan **F** ([details](docs/WINDOWS-BEVY-DIAGNOSIS.md)) | Fresh `5168fdd` B, 2 release tests; relocated package replay and six app-requested frame PNGs, exact model match; physical input unverified ([details](docs/packaging/BEVY-C77-OMARCHY-REPLAY.md)) |
| Tauri + Rust WASM | B / I | B / I | B / revised-physics native scripted replay and Canvas export passed; real input unverified ([evidence](docs/OMARCHY-TAURI-CONTACT-TIMING.md)) |
| Miniquad | cached B + 8 tests; winner-label fix visually checked in automated CPU-win match and restart; earlier shell replay failed ([details](docs/MINIQUAD-MAC-CONTACT-TIMING-RECHECK.md)) | Fresh MSVC B + 1 test; scripted/six PNG reported passed; native Return/Space still no start | Fresh `ee445a7` B, 8 tests including winner geometry; relocated replay and six app-owned PNGs match model; winner visual and physical input unverified ([details](docs/packaging/MINIQUAD-WINNER-OMARCHY-PILOT.md)) |
| pixels + winit | Fresh `663aa40` B / native automated full match and restart; shell replay incomplete ([details](docs/PIXELS-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Fresh `7495366` B, 5 frontend tests; relocated package replay and six app-owned source PNGs, exact model match; physical input unverified ([details](docs/packaging/PIXELS-C77-OMARCHY-REPLAY.md)) |
| raw wgpu + winit | Fresh `617160c` B / native automated full match and restart; shell replay incomplete ([details](docs/WGPU-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Fresh `663aa40` B, 4 frontend tests; relocated package replay and six app-owned surface-readback PNGs, exact model match; physical input unverified ([details](docs/packaging/WGPU-C77-OMARCHY-REPLAY.md)) |
| Piston | Fresh `c2a4884` B / native automated full match and restart; shell replay incomplete ([details](docs/PISTON-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Fresh `38584cd` B, 7 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/PISTON-C77-OMARCHY-REPLAY.md)) |
| ggez | B / I; Mac surface fix verified | B / I on earlier snapshot | Fresh `24fbcd9` B, 2 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/GGEZ-C77-OMARCHY-REPLAY.md)) |
| nannou | `5215d8a` native automated full match; `ac99028` pause-label/resume recheck passed; earlier shell replay failed ([details](docs/NANNOU-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Fresh pause-label fix: 2 release tests, scripted frame inspection and exact model replay; physical input unverified ([details](docs/NANNOU-PAUSE-INDICATOR.md)) |
| Fyrox | Fresh `5005498` B / native automated full match and restart; shell replay incomplete ([details](docs/FYROX-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Fresh `8b5ff6a` B, 2 release tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/FYROX-C77-OMARCHY-REPLAY.md)) |
| Blue Engine | `4bab2ae` cached B; automated desktop full match/restart and relocated archive smoke; direct-shell replay failed before gameplay ([details](docs/BLUE-MAC-CONTACT-TIMING-RECHECK.md)) | B / I | Held-pause fix: 4 release tests; relocated windowed package replay matches model, native input unverified ([details](docs/packaging/BLUE-HELD-OMARCHY-PACKAGE.md); [earlier headless captures](docs/packaging/BLUE-C77-OMARCHY-REPLAY.md)) |
| raylib-rs | Fresh `99034e6` B / native automated full match and restart; shell replay incomplete ([details](docs/RAYLIB-MAC-CONTACT-TIMING-RECHECK.md)) | B / R; start input unsuccessful; LLVM build blocker resolved | Fresh `971319e` B, 3 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/RAYLIB-C77-OMARCHY-REPLAY.md)) |
| Godot + Rust GDExtension | Historical B / I; updated extension B; headless smoke + exact replay and automated native full match/restart pass with writable log ([details](docs/GODOT-MAC-CONTACT-TIMING-ATTEMPT.md)); earlier editor shutdown **F** | U / U | Updated pause prompt: locked extension B, isolated smoke and exact scene replay; pause viewport inspected, physical input unverified ([details](docs/GODOT-PAUSE-PROMPT-OMARCHY-RECHECK.md); [earlier run](docs/GODOT-C77-OMARCHY-REPLAY.md)) |
| bracket-lib (RLTK) | B / bounded native I on existing contrast package; start, movement, pause, full match, restart and held-Q exit checked | B / I; separate header improves readability, bitmap quality still rough | Fresh `c691bdf` B, 9 tests; relocated X11 fallback replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/BRACKET-C77-OMARCHY-REPLAY.md)) |

Details: [baseline frontends](docs/desktop-comparison.md), [extra engines](experiments/ENGINE-STATUS.md), [ggez fix](experiments/ggez/README.md), [Godot](experiments/godot/VERIFICATION.md), [bracket-lib](experiments/bracket-lib/README.md).

### Low-level rendering and native GUI/terminal clients

| Implementation | Mac build / runtime | Windows build / runtime | Omarchy ARM64 build / runtime |
| --- | --- | --- | --- |
| softbuffer + winit | Fresh `f40c9a5` B / native automated full match and restart; shell replay incomplete ([details](docs/SOFTBUFFER-MAC-CONTACT-TIMING-RECHECK.md)); prior focus/resize checks are historical | B / I | Fresh `cf3bff1` B, 4 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/SOFTBUFFER-C77-OMARCHY-REPLAY.md)) |
| SDL3 | Fresh `082327b` B / native automated full match and restart; shell replay blocked ([details](docs/SDL3-MAC-CONTACT-TIMING-RECHECK.md)); prior focus/resize checks are historical | B / I; LNK4098 linker warning remains | Fresh `49f2c2f` B, 2 frontend + 3 shared tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/SDL3-C77-OMARCHY-REPLAY.md)) |
| Vulkan via ash | B / blocked: missing Vulkan loader/MoltenVK | B / I (simple clear-attachment renderer only) | Fresh `0cd6a1c` B, 6 frontend tests; relocated package replay and six app-owned PNGs, exact model match; physical input unverified ([details](docs/packaging/ASH-C77-OMARCHY-REPLAY.md)) |
| Slint | `9195b63` cached debug B / automated full match and reset; shell replay timed out ([details](docs/SLINT-MAC-CONTACT-TIMING-RECHECK.md)) | B / I: start, scoring, pause/resume, reset | `ccd3783` B, 10 shared tests; relocated replay and six component PNGs match model; physical input unverified ([details](docs/packaging/SLINT-GPUI-PAUSE-OMARCHY-PILOT.md)) |
| GPUI | B / I: Enter start, held-W movement, scoring, Space pause | B / I; maximized rendering checked | `ccd3783` B, 1 geometry + 10 shared tests; relocated state-only replay matches model; visual/input unverified ([details](docs/packaging/SLINT-GPUI-PAUSE-OMARCHY-PILOT.md)) |
| GTK4 | Blocked: native dependencies missing / U | U / U | `59957bb` B, 1 frontend + 10 shared model tests; relocated replay and six app-owned Cairo PNGs match model; physical input unverified ([details](docs/packaging/GTK-GUI-PAUSE-OMARCHY-PILOT.md)) |
| Ratatui | B / pseudo-terminal checks; enhanced-terminal interaction unverified | B / blocked: direct launch has no targetable window; Windows Terminal access blocked by product policy | Fresh repeat-fix release B, 10 tests; relocated package replay and synthetic enhanced-key PTY pause/repeat check; real-terminal and physical input unverified ([details](docs/packaging/RATATUI-REPEAT-OMARCHY-PACKAGE.md); [earlier run](docs/packaging/RATATUI-C77-OMARCHY-REPLAY.md)) |

Details: [low-level clients](experiments/LOW-LEVEL.md), [GUI toolkits](experiments/GUI-TOOLKITS.md), [Ratatui](experiments/ratatui/README.md). GPUI's initial default Mac check required the separate Metal Toolchain; the runtime-shader path and later native worker results are reflected above.

### Rust web UI frameworks

| Implementation / target | Mac | Windows | Omarchy |
| --- | --- | --- | --- |
| Yew browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | B / bounded localhost focus and pause check ([details](docs/YEW-LEPTOS-FOCUS-CHROME-OMARCHY.md)) |
| Leptos browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | B / bounded localhost focus and pause check ([details](docs/YEW-LEPTOS-FOCUS-CHROME-OMARCHY.md)) |
| Dioxus browser | B / I | B / I, keyboard + pointer, CPU win, restart, narrow viewport | U / U |
| Yew native Tauri wrapper | Fresh `b92b7df` B + state/SVG replay + bounded I; provenance/evidence published `18bc91c` | B / I | Focus-fix debug B, 2 bridge + 13 shared tests; relocated scripted SVG replay matches model; native input/visual unverified ([evidence](docs/packaging/YEW-FOCUS-OMARCHY-PACKAGE.md); [earlier pilot](docs/packaging/YEW-PAUSE-OMARCHY-PILOT.md)) |
| Leptos native Tauri wrapper | Fresh `b92b7df` B + state/SVG replay + bounded I; provenance/evidence published `18bc91c` | B / I | `0cf9e95` debug B, 12 shared tests; relocated scripted SVG replay matches model; visual/input unverified ([evidence](docs/packaging/LEPTOS-PAUSE-OMARCHY-PILOT.md)) |
| Dioxus native desktop | Fresh `4088089` B / I + native replay; held controls/release, full match and restart passed | B / I; maximized layout checked | Focus-fix debug B, 13 shared tests; relocated scripted SVG replay matches model; native input/visual unverified ([evidence](docs/packaging/DIOXUS-FOCUS-OMARCHY-PACKAGE.md); [earlier pilot](docs/packaging/DIOXUS-PAUSE-OMARCHY-PILOT.md)) |

[Commands and detailed web-framework results](docs/web-framework-comparison.md). Browser checks use a localhost HTTP server, never file URLs. Narrow browser layout is not a physical touchscreen or mobile-package test.

Omarchy explicit-pause follow-up: shared Rust web-controller model now clears held keyboard and pointer input on the Pause button or Space/Escape; 12 tests and adapter compilation pass. Browser/native interaction remains pending on Omarchy; Dioxus Desktop, Yew and Leptos have refreshed debug package replays ([Dioxus](docs/packaging/DIOXUS-PAUSE-OMARCHY-PILOT.md), [Yew](docs/packaging/YEW-PAUSE-OMARCHY-PILOT.md), [Leptos](docs/packaging/LEPTOS-PAUSE-OMARCHY-PILOT.md)). [Fix evidence](docs/WEB-EXPLICIT-PAUSE-INPUT-FIX.md).

Historical pointer ownership follow-up: shared Rust controller model tests pass (11), including independent same-button pointer release, unrelated release, opposing pointers, and pause clearing; Yew, Leptos, and Dioxus web adapters pass `cargo check` for `wasm32-unknown-unknown`; Dioxus desktop also passes `cargo check`. These are model and compilation checks, not browser or physical touch interaction. Strict Clippy remains blocked by existing replay-code and test-module lint warnings.

### Mobile targets

| Implementation | Test host / target | Build and tests | Actual runtime evidence | Still outstanding |
| --- | --- | --- | --- | --- |
| Native Kotlin | Windows / Android 15 API35 emulator | Debug APK + 11 unit tests pass (latest replay candidate) | Installed; touch paddle movement, ball motion, pause/resume/restart checked | Full match, physical device, store packaging |
| Native Swift | Mac / iPhone16 Pro iOS18.4 simulator (latest) | `038f85e` Debug/Release builds + 11 core tests + 2 UI tests pass | DEBUG app replay 1440 ticks / 13 states / 6 app-owned PNGs; normal touch/release, Home pause/resume, full match and restart passed (`18bc91c`) | Physical device, store packaging |
| Tauri + Rust WASM iOS | Mac / iPhone16 iOS18.4 simulator | Development build passes via CLI coordinator + xcodebuild | Device Hub visual start, held Up/Down touch, scoring, pause/restart pass | White strip absent in updated portrait simulator shell; landscape controls fit and Start reached Playing on one simulator; [background](docs/TAURI-IOS-SAFE-AREA-20261113.md), [landscape](docs/TAURI-IOS-LANDSCAPE-20261113.md). XCTest match/score reset passed; repeated Start/Pause/Resume had one pass and two failures; physical input and release qualification remain open |
| Tauri + Rust WASM Android | Windows / Android emulator | Build/install pass | Start, scoring and pause verified | Sustained touch, full match, physical device and release packaging |
| Yew/Leptos Tauri mobile; Dioxus mobile | iOS / Android | U | U | Mobile setup, packaging and interaction qualification |
| Godot Rust mobile/web exports | iOS / Android / web | U | U | Export pipeline/templates; mobile touch controls not implemented |

Details: [native Kotlin/Swift](experiments/NATIVE-MOBILE.md), [Tauri mobile](docs/mobile-comparison.md), [Godot scope](experiments/godot/README.md). Other engine mobile exports are not implied by their desktop results.

### Acceptance coverage and remaining work

| Check or issue | Current evidence / remaining work |
| --- | --- |
| Shared simulation | Omarchy: 13 Rust core model tests pass, including scoring, restart and paddle face-crossing cases. Mac also passed core, scenario and WASM model tests and WASM compilation. Windows passed the same model and scenario tests with zero portable replay delta. The timing correction passes 19 Omarchy core model tests; prior Ratatui replay predates it. Independent Mac and Windows model review, client rebuilds, replay and physical interaction remain pending; [collision review](docs/RUST-PADDLE-FACE-COLLISION.md). |
| Adapter regressions | Windows web controller 3/3, low-level controller 3/3, Ratatui 5/5 (Omarchy current), bracket-lib 6/6, GUI support 4/4; other adapter results linked above. |
| Windows engine interaction | Most I cells mean start, scoring/ball motion, pause/resume and reset. Sustained paddle movement/release, full matches, focus/minimize and resize/DPI are mostly still unverified. |
| Windows web interaction | Browser variants exercised keyboard/pointer and CPU win; real focus-loss still unverified. Native wrappers exercised pointer movement and basic controls, not held-key/full-match/physical-touch coverage. |
| Omarchy visual/input testing | Many builds and smoke launches pass, but the automation connection exposes no native/browser surfaces. This is a tool-access blocker, not proof of a gameplay failure. |
| Known defects | Windows Bevy default Vulkan crash (DX12 rendering corrected); Windows bracket-lib bitmap text remains rough despite improved layout; Miniquad supported keys translate to Unknown ([diagnosis](docs/MINIQUAD-WINDOWS-INPUT-DIAGNOSIS.md)); raylib supported start input unsuccessful; Godot headless editor shutdown crash. |
| Snapshot freshness | Windows extra-engine interaction used existing isolated GNU binaries, not a fresh latest-source rebuild. The fresh Mac nannou UI check includes the pacing patch. bracket-lib contrast package header layout has also been inspected on Mac; see [bounded native check](docs/BRACKET-LIB-MAC-NATIVE-CHECK.md). Requalification remains necessary before release. |
| Packaging and compatibility | No complete signed-installer/download-install matrix, physical-mobile coverage, exhaustive OS/DPI/input matrix or accessibility audit. |
| Performance | Fourteen Mac and fifteen Omarchy native adapter pilots recorded; Windows results are listed individually in the timing matrix below. Omarchy has revised-physics release pilots for [Macroquad](docs/benchmarks/MACROQUAD-C77-OMARCHY-PILOT.md), [softbuffer](docs/benchmarks/SOFTBUFFER-C77-OMARCHY-PILOT.md), [SDL3](docs/benchmarks/SDL3-C77-OMARCHY-PILOT.md) and [Miniquad](docs/benchmarks/MINIQUAD-WINNER-OMARCHY-PILOT.md), with differing reported dimensions. Callback intervals only, not controlled cross-engine rankings or full acceptance. |

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
| nannou | Passed | Six saved; pause/reset inspected | Framework window-frame export; pause label added in later follow-up |
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

Benchmark runner follow-up: callback samples must include internally consistent interval statistics to be marked valid, and malformed output is retained as invalid private evidence; [scope and Windows verification](docs/benchmarks/RUNNER-SAMPLE-VALIDATION.md).

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
| SDL3 | [Held-fix relocated replay and desktop smoke](docs/packaging/SDL3-HELD-MAC-RELOCATED.md) | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | Local pilot |
| raylib | [Held-fix relocated replay and desktop smoke](docs/packaging/RAYLIB-HELD-MAC-RELOCATED.md); [earlier pilot](docs/packaging/RAYLIB-MAC-PILOT.md) | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | Local pilot |
| ash | Loader blocked | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | Local pilot |
| Miniquad | [Local replay/capture pilot](docs/packaging/MINIQUAD-MAC-PILOT.md) | [Local ZIP/replay pilot](docs/packaging/MINIQUAD-WINDOWS-PILOT.md) | [Four-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH.md) |
| pixels, raw wgpu | [Local replay/capture pilots](docs/packaging/MAC-PIXELS-WGPU-PILOT.md) | [Local ZIP/replay pilots](docs/packaging/WINDOWS-PORTABLE-BATCH.md) | [Four-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH.md) |
| Piston, ggez | [Piston](docs/packaging/PISTON-HELD-MAC-RELOCATED.md) and [ggez](docs/packaging/GGEZ-HELD-MAC-RELOCATED.md) held-fix relocated checks; [earlier pilots](docs/packaging/MAC-PISTON-GGEZ-PILOT.md) | [Local ZIP/replay pilot](docs/packaging/WINDOWS-PORTABLE-BATCH2.md) | [Six-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH2.md) |
| Fyrox | [Held-pause relocated replay and desktop smoke](docs/packaging/FYROX-HELD-MAC-RELOCATED.md); [earlier corrected HUD check](docs/FYROX-HUD-SCALE-FIX.md) | [Local ZIP/replay; generated state preserved](docs/packaging/WINDOWS-PORTABLE-BATCH3.md) | [Corrected HUD replay passed; full cleanup incomplete](docs/FYROX-HUD-SCALE-FIX.md) |
| Nannou, bracket-lib, Bevy | [Nannou](docs/packaging/NANNOU-HELD-MAC-RELOCATED.md) and [bracket-lib](docs/packaging/BRACKET-HELD-MAC-RELOCATED.md) held-fix relocated checks; [earlier pilots](docs/packaging/MAC-FOUR-ENGINE-PILOT.md) | [Local ZIP/replay pilots; Bevy DX12](docs/packaging/WINDOWS-PORTABLE-BATCH3.md) | [Six-adapter batch](docs/packaging/LINUX-PORTABLE-BATCH2.md) |
| Blue Engine, softbuffer | [Blue held-fix relocated replay and desktop smoke](docs/packaging/BLUE-HELD-MAC-RELOCATED.md); [softbuffer held-fix relocated check](docs/packaging/SOFTBUFFER-HELD-MAC-RELOCATED.md); [earlier pilot](docs/packaging/MAC-SOFTBUFFER-BLUE-PILOT.md) | [Blue state-only](docs/packaging/WINDOWS-PORTABLE-BATCH3.md); [softbuffer replay/capture](docs/packaging/WINDOWS-PORTABLE-BATCH.md) | [Final two local pilots](docs/packaging/LINUX-PORTABLE-BATCH3.md); Blue native captures unsupported |
| Ratatui | [Release archive, replay and synthetic PTY checks](docs/packaging/RATATUI-MAC-RELEASE-PILOT.md); [earlier debug pilot](docs/packaging/RATATUI-MAC-DEBUG-PILOT.md) | [Local ZIP/cell replay only](docs/packaging/RATATUI-WINDOWS-PILOT.md) | [Terminal local pilot](docs/packaging/RATATUI-LINUX-PILOT.md); cells + bounded PTY, not screenshots |
| GTK4 | Not recorded | Not recorded | [Local package pilot](docs/packaging/GTK-LINUX-PILOT.md); existing GTK4 libraries required |
| Slint | [Local debug archive and desktop smoke](docs/packaging/SLINT-MAC-DEBUG-PILOT.md) | [Local ZIP/own-window snapshots](docs/packaging/SLINT-WINDOWS-PILOT.md) | [Local package pilot](docs/packaging/SLINT-LINUX-PILOT.md); own-window snapshots, license text included |
| GPUI | Not recorded | [Local ZIP/state-only replay](docs/packaging/GPUI-WINDOWS-PILOT.md) | [Local archive/state-only replay](docs/packaging/GPUI-LINUX-PILOT.md) |
| Tauri + Rust WASM | Not recorded | Not recorded | [Local archive/relocated replay pilot](docs/packaging/TAURI-LINUX-PILOT.md); WebKitGTK required, notices pending |
| Yew native Tauri wrapper | Not recorded | Not recorded | [Refreshed debug archive/relocated replay](docs/packaging/YEW-PAUSE-OMARCHY-PILOT.md); WebKitGTK required, notices pending |
| Leptos native Tauri wrapper | Not recorded | Not recorded | [Refreshed debug archive/relocated replay](docs/packaging/LEPTOS-PAUSE-OMARCHY-PILOT.md); WebKitGTK required, notices pending |
| Dioxus Desktop | Not recorded | Not recorded | [Local debug archive/relocated replay](docs/packaging/DIOXUS-PAUSE-OMARCHY-PILOT.md); WebKitGTK and libxdo required, notices pending |
| Other adapters | Not recorded | Not recorded | Not recorded |

Evidence and limitations: [packaging index](docs/packaging/README.md), [Mac Macroquad](docs/packaging/MACROQUAD-MAC-PILOT.md), [Windows Macroquad](docs/packaging/MACROQUAD-WINDOWS-PILOT.md), [Mac SDL3](docs/packaging/SDL3-MAC-PILOT.md), [Mac raylib](docs/packaging/RAYLIB-MAC-PILOT.md), [Linux ash](docs/packaging/ASH-LINUX-PILOT.md).
