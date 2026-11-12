# Scripted application-input smoke tests

Macroquad is the first representative adapter. This is **not** desktop automation
or a replacement for interactive/visual QA. The shared `pong-scenario` crate feeds
the same simulation boundary as the adapter's ordinary input handling. It never
injects OS events or captures the desktop. Optional PNG export reads only its own application framebuffer. Other adapters do not yet implement it.

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
