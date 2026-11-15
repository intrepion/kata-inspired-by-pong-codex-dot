# Fyrox held movement across pause

On Omarchy ARM64, the Fyrox keyboard adapter retained movement pressed through explicit pause and resume. A focused adapter regression failed before the fix with W still producing upward movement after Space and Enter. The extracted key handler now suppresses each movement binding held when Space or Escape pauses Playing, until its release; repeated key-down events do not unsuppress it. Focus loss retains the existing held-key reset and also resets suppression. Fresh aliases and opposite directions remain independent.

Four offline locked release tests pass: two benchmark argument tests and two adapter tests covering all four movement bindings with Space and Escape, repeat, release and fresh press, aliases and opposing directions. A release build passed. With host-display access, the rebuilt executable's separate scripted run exited 0 and all 13 telemetry records exactly matched `evidence/rust-core-contact-timing-model-20261112.jsonl`. This is adapter/model and scripted replay evidence, not native key delivery or physical interaction; focus event delivery and other platforms remain unverified.

Tested source based on `7b86153` with `experiments/fyrox/src/main.rs` Git blob `c55c2b7f104eb66a5f179f29d2e5e58926ff6c0e`; binary SHA-256 `6853cba1ce1adaf16f1c84cffbd7910e8c8decd68c3b23db0a27309b3455e6a8`. Replay is retained at `/tmp/pong-fyrox-held-pause-run-20261113/replay.log`. The build reused the existing Cargo target and replaced its prior build output; the previous relocated package binary remains at `/tmp/pong-fyrox-c77-relocated-20261113/pong-fyrox-linux-arm64/pong-fyrox`, SHA-256 `168379470733f427e48a63c4b7b1b974f18225d1eaba640a1f68e7dd397d4703`. No new package or release qualification is claimed.

## Mac recheck

macOS ARM64 source `f680d1075e6ee6575be107273a8ee8758711a7a3`. Offline locked release build reused a copy-on-write cache; all four Fyrox frontend tests passed, including held-movement suppression. Executable SHA-256 `c68c5c4f7039fca32da063008336af4978a0db04c18f3fe2f99e6ed92db67cc7`.

The packaged app opened through supported desktop automation. Sequential Enter, held W, Space, Enter, held S and Space produced visible upward movement, Paused, resumed play, downward movement and Paused again. The app exited. W was released before pause: held-across-pause native delivery and physical input were not demonstrated. No fresh scripted replay or capture is claimed for this Mac build.
