# Shared scripted browser replay

Build `cargo build -p pong-wasm --target wasm32-unknown-unknown`, copy
`target/wasm32-unknown-unknown/debug/pong_wasm.wasm` beside `app.js`, and serve
this directory over HTTP. Open `http://127.0.0.1:PORT/?scripted-test`.

This opt-in mode runs the Rust `pong-scenario::Scenario` timeline and seed.
Buttons and keyboard gameplay controls are ignored during replay. It stops at
1440 fixed steps, retaining thirteen JSON telemetry records and six actual
app-owned Canvas2D PNG data URLs. A download link appears at completion for one
JSON bundle containing both. `window.pongDemo` exposes the same evidence for
an authorized browser test. Nothing is uploaded automatically.

Captures stop simulation at the shared capture ticks so slow rendering cannot
skip a checkpoint. They show the court canvas, not the DOM status line or the
OS desktop. Scripted state/render evidence is not a physical input, compositor,
continuous motion, packaging, or benchmark pass. Reload without the query
parameter to restore interactive play.
