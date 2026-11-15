# Pixels and raw wgpu explicit-pause held-input fix

Pixels and raw wgpu each use a winit key-event adapter. Space pause previously left held movement bindings active, and repeated movement key-down events could restore a cleared hold. New tests at each adapter's key-event boundary reproduced the stale axis before the fix. The shared physics was not changed.

Both adapters now clear movement when Space transitions Playing to Paused and ignore repeated movement key-down events; releases are still processed and a fresh press after resume moves normally. Regression coverage exercises W, Up, S and Down individually, plus existing start, pause, repeat, restart and quit actions. All seven Pixels and six wgpu release tests passed on Omarchy, as did locked offline release builds. The tested tree was based on `bbdd24ae63047bc2371457b0aab9f7ab8bf859a1`; final source blobs were `e9a893d71e21b1ebcb619133a25095b213421194` (Pixels) and `3386e42063488bfc5477d483400fef1c739e1875` (wgpu). Tested binary SHA-256 values were `5b590f308a7146f4b17ffcd78b54ce63b75d5964adb00769bba9924033e0dd19` and `b7c4eca516b1bd976ba6d6e4121aa32836bc3435c3f5222d4cb330d6925939f2`, respectively. Prior binaries remain in `/tmp/pong-pixels-wgpu-pre-pause-20261113/`.

With permitted display access, both final binaries completed bounded scripted replays with 13 telemetry records exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl` and six app-owned PNGs each. Outputs are retained under `/tmp/pong-pixels-pause-final-20261113/`, `/tmp/pong-wgpu-pause-final-20261113/` and corresponding `-output-20261113.txt` files. Earlier captures from the first post-fix builds were inspected; the final captures were not separately inspected. These replays bypass real key input and do not independently verify the pause behavior.

Native/physical input, other platforms, rebuilt packages, full matches and release qualification remain pending. Existing evidence is historical. Additional low-level adapters have separate key paths and are not covered by this fix.

## Mac recheck

macOS ARM64 source `caca70820acbb49046878fae07eb69509dc5eca6`. Offline locked release builds reused copy-on-write caches; Pixels passed seven tests and raw wgpu six, including each held-input action-boundary regression. Binary SHA-256: Pixels `3214f50d8272e15e847a8cbe15784b17004eb4111830c9192f2454c64bf5a152`; wgpu `ab52b528613e139f2ef46b607271fee5c55d81f148485539197e34fa947f2a5b`.

Each packaged app opened via supported desktop automation. Sequential Enter, held W, Space, Space, held S and Space produced visible upward movement, Paused, resumed play, downward movement and Paused. Apps exited. W was released before pause; native held-across-pause and physical keyboard input were not tested. No fresh scripted replay or capture is claimed.
