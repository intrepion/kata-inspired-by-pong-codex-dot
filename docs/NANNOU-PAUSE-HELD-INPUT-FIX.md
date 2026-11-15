# Nannou explicit-pause held-input fix

Nannou polls current key states each frame. Before this change, a movement key held while Space paused remained active after resume without a new press. A new polled-input regression failed before the fix. Shared physics was not changed.

The adapter now suppresses each movement binding held when Space enters pause until a later poll observes its release. Fresh presses can move; independently held aliases remain separate and opposing directions cancel. Tests cover all four movement bindings and Space start/pause/resume. All five Nannou release tests passed on Omarchy, as did an offline locked release build. The tested tree was based on `31a6c4565c1e7f6d32aab4c60fc449f34ac6bf07`, with `experiments/nannou/src/main.rs` Git blob `899b956044c964dbcf09091f622f526848987c6a`. Tested binary SHA-256 was `071bbe5f32aabe0e2068da12986874f3f2801fe1d76acac43140316eefeee2ab`; the prior binary remains at `/tmp/pong-nannou-pre-pause-20261113/pong-nannou`.

With permitted display access, the new binary completed its bounded scripted replay with 13 telemetry records exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl` and six app-owned PNGs. Outputs remain under `/tmp/pong-nannou-held-pause-20261113/` and `/tmp/pong-nannou-held-pause-output-20261113.txt`. These captures were not visually inspected in this run. The replay bypasses real key input and does not independently verify pause behavior.

Native/physical input, polling behavior across focus changes, other platforms, rebuilt packages, full matches and release qualification remain pending. The existing unfocus path was not changed. Existing evidence is historical; other engines have independent input paths.

## Mac recheck

macOS ARM64 source `986cbf89703320c54b53b669537f81feb348a78a`. Offline locked release build reused a copy-on-write cache and all five Nannou tests passed, including polled held-key suppression. Executable SHA-256 `9393413e9c4c74c2102e141611413366780ed3af73d4abef80cfd1dedc9f908a`.

The packaged app opened through supported desktop automation. Sequential Enter, held W, Space, Space, held S and Space produced upward movement, the Paused label, resumed play, downward movement and Paused again. The app exited. W was released before pause: held-across-pause native delivery and physical input were not demonstrated. No fresh scripted replay or capture is claimed for this Mac binary.
