# Scripted application-input smoke tests

Macroquad, softbuffer, raylib-rs, Pixels and SDL3 support the shared demo. This is **not** desktop automation
or a replacement for interactive/visual QA. The shared `pong-scenario` crate feeds
the same simulation boundary as the adapter's ordinary input handling. It never
injects OS events or captures the desktop. Optional PNG export reads only its own application framebuffer. Remaining adapters do not yet implement it.

## Run and watch

```sh
cargo run -p pong-macroquad -- --scripted-test > macroquad-smoke.jsonl
```

A normal game window renders, visibly labeled `SCRIPTED INPUT`. It exits after
1,440 simulation ticks (approximately 12 seconds on a responsive desktop). Start
without the flag for ordinary keyboard play. Scripted mode deliberately ignores
keyboard/focus pause signals so runs remain reproducible; closing the window can
still end the run early. A complete log must end at tick 1440 with `finished:true`.
A stalled/minimized renderer can take longer; a shell timeout can bound wall time.

| Tick / simulation time | Stimulus |
|---|---|
| 0 / 0 s | Seed 42, start, hold up |
| 120 / 1 s | Release up, hold down |
| 240 / 2 s | Release movement |
| 360 / 3 s | Pause |
| 480 / 4 s | Resume |
| 1200 / 10 s | Restart (scores and paddles reset) |
| 1440 / 12 s | Stop and close after final rendered frame |

The clock uses fixed 120 Hz simulation ticks, not wall-clock input timestamps.
Rendering may process several ticks in a frame; exact event order remains fixed.
Paused simulation does not pause the scenario clock. Stdout emits JSONL at tick
0 and every 120 ticks with state, paddle/ball positions, scores and completion.
Compare repeated logs on the same platform; cross-platform floating-point
bit-identical output is not promised. Stderr remains available for renderer errors.

## Checks and limits

```sh
cargo test -p pong-core -p pong-scenario
cargo check -p pong-macroquad
```

Scenario regression tests exercise the public driver seam: start, bounds under
held up/down, release, frozen state during pause, resume, restart, deterministic
replay across different frame batch sizes, and bounded completion. The nine core
tests separately cover simulation rules. These do **not** verify real key events,
window focus, touch, resize/DPI, audio, visible drawing, or OS integration. Script
completion is telemetry evidence only; do not promote a matrix row to interactive
or visual PASS without separately observing and testing those behaviors.

2026-11-12 Omarchy ARM64: build and 11 tests passed. Live-session replay result is
recorded in the root README. No desktop screenshot/control was used.

## Optional application-framebuffer PNGs (native only)

```sh
cargo run -p pong-macroquad -- --scripted-test --scripted-captures ./new-smoke-images > macroquad-smoke.jsonl
```

The output directory must not already exist (prevents overwriting earlier evidence).
Macroquad's supported `get_screen_data().export_png()` flushes draw commands and
exports this application's framebuffer at ticks 120, 240, 480, 600, 1320, and 1440.
No desktop screenshot, other-app capture, OS input injection, or native-control
permission workaround is used. This export API is native-only, not WebAssembly.

2026-11-12 Omarchy run: exited 0, six PNGs saved and inspected; telemetry identical
to the no-capture run. Images show the top/bottom paddle movement, pause label,
resume, and reset score/paddles. Text, court and ball render correctly in the
observed frames. The tall tiled window has substantial unused space below the
court; this remains a layout-polish opportunity. These snapshots do not prove
smooth animation, real keyboard input, compositor presentation, focus or touch.
All six images were also delivered to the owner's Library.

## Shared core and rollout matrix

The shared `pong-scenario` crate owns input events, the 120 Hz clock, seed,
telemetry, argument parsing and six capture checkpoints. It depends on the pure
`pong-core` simulation; keeping this separate avoids adding native filesystem/PNG
concerns to gameplay. Adapters implement only their ordinary drawing and the
appropriate application-buffer export. No per-engine copy of the timeline.

| Adapter | Replay and export | Omarchy verification |
|---|---|---|
| Macroquad | `get_screen_data` current app buffer | Rerun after shared-helper refactor: exit 0, six PNGs, matching telemetry |
| softbuffer | Exact CPU RGB buffer submitted to `present` | Exit 0, six PNGs, matching telemetry; paused frame inspected |
| raylib-rs | Own backbuffer, explicit draw-batch flush before swap | Exit 0, six PNGs, matching telemetry; tick-480 image verified |
| Pixels | Exact 900x500 CPU RGBA source, encoded RGB | Exit 0, six PNGs, matching telemetry; paused source frame inspected |
| SDL3 | Current renderer target readback before present | Exit 0, six PNGs, matching telemetry; paused frame inspected |
| Bevy | Framework screenshot observer; awaits completed PNG | Exit 0, six PNGs; pause/reset inspected |
| nannou | Framework main-window export; awaits completed PNG | Exit 0, six PNGs; pause/reset inspected |
| ggez | App-owned render texture, presented then read after submit | Exit 0, six PNGs; pause/reset inspected |
| Miniquad | Own active OpenGL framebuffer read before swap; flipped rows | Exit 0, six PNGs; paused frame inspected |
| raw wgpu | Current surface texture COPY_SRC with padded-row readback | Exit 0, six PNGs; paused frame inspected |
| Piston | Same scene on app-owned wgpu target | Exit 0, six PNGs; pause/reset inspected |
| bracket-lib | Native BTerm framebuffer export | Exit 0, six PNGs; pause/reset inspected |
| Fyrox | App-owned UI render target | Exit 0, six PNGs; pause/reset inspected |
| Blue Engine | Supported headless own-buffer renderer | Exit 0, six PNGs; pause/reset inspected; no window capture |
| Vulkan via ash | Acquired swapchain image copied before present | Exit 0, six PNGs; all inspected |
| Godot + Rust | Actual scene viewport export after frame draw | Exit 0, six PNGs; pause/reset inspected |
| Ratatui | Supported TestBackend cell/text exports, no PNG | Exit 0, six text + six style JSON; no terminal presentation claim |
| Slint | Supported software component snapshot | Exit 0, six PNGs; pause/reset inspected |
| GTK4 | Same draw_scene on app-owned Cairo surface | Exit 0, six PNGs; pause/reset inspected |
| Other Rust adapters | Not wired yet; API-specific work remains | No demo/capture claim |
| Independent Swift/Kotlin | Not wired; should consume a portable scenario fixture | No Rust FFI/timeline duplication assumed |

Run experiment adapters with `cargo run --manifest-path experiments/softbuffer/Cargo.toml -- --scripted-test --scripted-captures ./new-softbuffer-images` (substitute `raylib-rs` for raylib).

Batch evidence: [frame dimensions/hashes and run outcomes](evidence/scripted-batch1-20261112.json).
Shared checks: nine core tests plus four driver/PNG tests. Raylib also passes its
viewport test. Renderer snapshots remain separate from real-input QA. Raylib's
convenience screenshot function mishandled absolute paths and a post-swap read
returned the prior frame; the adapter now exports the current app buffer before
swap and checks output exists. No generic desktop capture fallback.

Batch2 adds Pixels and SDL3 using the same arguments and common Demo.
Pixels PNGs intentionally contain the source raster only; they cannot establish
GPU scaling, letterboxing or final display correctness. Its score/status remain
in the window title, which is not part of its app-buffer capture. SDL3 reads its
renderer target via the supported API and respects converted-surface row pitch.
Both pass builds and strict Clippy; Pixels has one framebuffer regression test;
low-level-common has three behavior/raster tests.

Some renderers (notably raylib) add their own stdout diagnostics. When comparing
telemetry, retain only lines beginning with `{"scenario"`; do not mistake renderer
log messages for malformed scenario JSON.

Batch2 recorded evidence: [Pixels/SDL3 PNG hashes and outcomes](evidence/scripted-batch2-20261112.json).

Batch3 adds Bevy, nannou, ggez, Miniquad and raw wgpu. All five pass builds and strict Clippy; Miniquad also passes its geometry test. Shared core/scenario tests remain green. All five runs produce 13 baseline-identical telemetry records and six PNGs. Bevy and nannou hold the simulation until asynchronous exports finish (bounded timeout); ggez reads the exact app render texture after submission. Miniquad explicitly supports OpenGL capture only, not Metal. wgpu requires surface COPY_SRC and an RGBA8/BGRA8 format, with bounded map waits; unsupported capture fails rather than using a desktop fallback. Existing render styles differ: Miniquad has a square ball, wgpu a round ball and lighter sRGB court. These are not evidence of equal pixel output or compositor correctness.

Batch3 evidence: [GPU framework outcomes](evidence/scripted-gpu-batch-20261112.json), [low-level outcomes](evidence/scripted-lowlevel-batch3-20261112.json).

Batch4: [Piston/bracket-lib verification and capture boundaries](SCRIPTED-PISTON-BRACKET.md). Shared tests now total fourteen. Piston exposed a pending-capture timing bug across multiple update callbacks; the common driver now holds each unconsumed checkpoint until render, with a red/green regression test.

Batch5: [Fyrox/Blue verification and boundaries](SCRIPTED-FYROX-BLUE.md) and [ash evidence](evidence/scripted-ash-20261112.json). Ash checks swapchain transfer support and packed RGBA/BGRA formats, uses explicit image/buffer barriers and waits for completion. Both with/without capture runs match all 13 baseline records. Two adapter tests, build and strict Clippy pass. Vulkan validation layers were unavailable. [Ash contact sheet](evidence/omarchy-ash-scripted-20261112.jpg) shows all six frames; this is still not compositor, interactive input or smooth-motion verification.

Batch6: [Ratatui/Godot commands and evidence limits](SCRIPTED-RATATUI-GODOT.md). Sixteen adapters now export app-owned images; Ratatui separately exports terminal cells, not screenshots. Godot normal integration smoke passes; existing paused overlay/ball overlap remains documented.

Batch7: [Slint/GTK evidence and limits](SCRIPTED-SLINT-GTK.md). Eighteen adapters export app-owned images and Ratatui separately exports cells. Godot pause overlay no longer covers the ball; the footer retains its paused status, verified by a fresh viewport replay. Slint attribution clipping remains documented. No compositor or real-input pass is implied.

Raw web-WASM now exposes the shared Scenario via Rust ABI and optional Canvas export: [commands and validation limits](../clients/web-wasm/SCRIPTED-DEMO.md). WASM compilation and actual ABI Node execution pass; browser rendering/export remains unverified, not added to the nineteen-render-adapter count. Slint attribution height now fits its complete URL/version, verified with fresh snapshots. GPUI candidate remains unpublished: existing xattr0.2.3 references missing libc::ENOATTR on Linux before adapter typecheck; no supported own-frame export API was found.

GPUI blocker resolved: [native state-only replay and dependency provenance](SCRIPTED-GPUI.md). The nineteen prior renderer adapters remain the verified render-evidence count; GPUI adds telemetry only. Yew/Leptos/Dioxus now use the common Scenario with tests and successful WASM builds; their optional SVG exports are scene source, not screenshots, and browser execution remains unverified. See [framework instructions](../experiments/web-ui-common/SCRIPTED-DEMO.md).

Tauri native WebKitGTK replay now verified: [scoped exporter and Canvas evidence](SCRIPTED-TAURI.md). It reuses the same shared Rust/WASM scenario, validates complete incoming states against native Scenario and saves only to a fixed opt-in NEW directory. Six PNGs cover the court canvas, not DOM score/status or full webview. Existing pause-overlay overlap is documented. This brings verified render-evidence adapters to twenty (nineteen image paths plus Ratatui cells); GPUI remains separate state-only evidence.

[Native Yew/Leptos verification](SCRIPTED-NATIVE-WEB-FRAMEWORKS.md) adds bounded shared replay with six SVG-source exports each and thirteen baseline-identical states. Scene geometry and score source match checkpoints. These are not raster screenshots or visual QA; the twenty prior renderer-evidence count is unchanged. The opt-in bridge validates evidence, writes only fixed names in a new selected directory, and does not broaden normal-play capabilities.
