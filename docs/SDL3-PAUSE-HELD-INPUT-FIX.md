# SDL3 explicit-pause held-input fix

SDL3 polls scancode state once after each event batch. Before this change, a movement key held while Space paused remained active after resume without a new press. A new input-gate regression failed before the fix. Shared physics and the common low-level model were not changed.

The adapter now marks each movement scancode held in the snapshot after an explicit transition into pause as suppressed until a later snapshot observes its release. Fresh presses can move; aliases remain independent and opposing directions can still cancel. Tests cover all four movement bindings, aliases and Space start/pause/resume. All four SDL3 adapter release tests passed on Omarchy, as did an offline locked release build. The tested tree was based on `5af5e9a46cdff89fad4070a5579a02ccd2d35a04`, with `experiments/sdl3/src/main.rs` Git blob `7e49c99768254f28d53a69f62a140ce1af7eb977`. Tested binary SHA-256 was `4a821dab5acdbfbec07364dee6e6fff1b672be66067a1fccd9e6d03524b9abb2`; the prior binary remains at `/tmp/pong-sdl3-pre-pause-20261113/pong-sdl3`.

With permitted display access, the new binary completed its bounded scripted replay with 13 telemetry records exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl` and six app-owned PNGs. Outputs remain under `/tmp/pong-sdl3-held-pause-20261113/` and `/tmp/pong-sdl3-held-pause-output-20261113.txt`. These captures were not visually inspected in this run. Replay bypasses real keys and does not independently verify the pause behavior.

The gate observes snapshots, not every transition: a release and repress entirely between polls cannot be distinguished from a continuous hold. Native/physical input, polling behavior across focus changes, other platforms, rebuilt packages, full matches and release qualification remain pending. The existing focus-loss path was not changed; the Windows SDL3 debug-runtime warning is separate. Earlier evidence remains historical; other engines have independent input paths.

## Mac recheck

macOS ARM64 source `765075a7b68097a82d33f6364e61cda3dd5411e8`. Offline locked release build reused a copy-on-write cache; all four SDL3 frontend tests passed, including held-scancode suppression. Executable SHA-256 `86b03c3e482d42e073f0e136010be620ac831f32e96079e863640cad8cc8ef7b`.

The packaged app opened through supported desktop automation. Sequential 300 ms Space, held W, Space, Space, held S and Space produced visible upward movement, Paused, resumed play, downward movement and Paused again. Escape exited. W was released before pause: held-across-pause native delivery and physical input were not demonstrated. No fresh scripted replay or capture is claimed for this Mac build.
