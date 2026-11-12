# Post-manifest regression gate: Omarchy

Tested source: `0cbe38f8da6f57173c5a53e692eedbf5b01ed573`, 2026-11-12.
This closes the tested-source gap between the earlier adapter screenshot runs
and introduction of the shared portable scenario manifest. It is not a new
interactive QA claim or benchmark.

- `cargo test --locked --offline -p pong-core -p pong-scenario -p pong-wasm`:
  **16 tests passed**, including exact original Rust baseline comparison.
- `cargo check --locked --offline -p pong-wasm --target wasm32-unknown-unknown`
  passed; cached `cargo build --locked --offline -p pong-bevy` passed. Bevy
  was not rerun visually in this batch.
- Locked/offline builds and strict all-target Clippy passed for Macroquad and
  event-driven Piston, using existing local dependency caches.
- Both rebuilt native apps completed the shared manifest run at tick 1440,
  exited zero, emitted **13 byte-identical original baseline records** and
  saved **six app-owned PNGs** each.
- Paused tick 480 and reset tick 1440 images inspected for both: ball, paddles,
  court, reset positions remain present. Macroquad shows 0:1 paused then 0:0;
  Piston's score indicator resets. Its window-title status is outside its PNG.
  Tall-window letterboxing persists, as previously documented. Macroquad's
  output dimensions changed during this run; no resize input was injected,
  and no claim about who/what caused the layout change is made.

Macroquad exports its own framebuffer via the framework API. Piston draws the
same scene into an app-owned offscreen GPU texture. Neither captures another
application or the OS desktop. These samples do not establish compositor
presentation, smooth animation, keyboard, focus or touch behavior.

Evidence: [hashes/dimensions](evidence/omarchy-manifest-regression-20261112.json),
[Macroquad telemetry](evidence/omarchy-macroquad-manifest-regression-20261112.jsonl),
[Piston telemetry](evidence/omarchy-piston-manifest-regression-20261112.jsonl).

Commands (use a new output directory and a 90-second timeout per run):

```
cargo build --locked --offline -p pong-macroquad
cargo clippy --locked --offline -p pong-macroquad -p pong-scenario --all-targets -- -D warnings
cargo build --locked --offline --manifest-path experiments/piston/Cargo.toml
cargo clippy --locked --offline --manifest-path experiments/piston/Cargo.toml --all-targets -- -D warnings
# Rebuilt app binaries:
pong-macroquad --scripted-test --scripted-captures <new-directory>
pong-piston --scripted-test --scripted-captures <new-directory>
```
