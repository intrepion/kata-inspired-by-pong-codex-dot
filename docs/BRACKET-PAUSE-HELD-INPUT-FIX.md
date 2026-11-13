# bracket-lib explicit-pause held-input fix

The graphical bracket-lib adapter retains a set of pressed keys. Previously, Space pause left held movement active, so the paddle resumed moving without a fresh press; an event-boundary regression failed before the fix. Shared physics was not changed.

On Space transition from Playing to Paused, each currently held movement binding is now suppressed until its key-up event. The pressed-key set remains intact so repeat events do not re-enable the binding or toggle pause. Fresh presses work independently of other still-suppressed aliases; focus and resize clear suppression alongside the existing input reset. Tests cover W, Up, S and Down, pause repeat, aliases and focus reset. All eleven bracket-lib release tests passed on Omarchy, as did an offline locked release build; the existing vendored FreeType warnings remain. The tested tree was based on `765075a7b68097a82d33f6364e61cda3dd5411e8`, with `experiments/bracket-lib/src/main.rs` Git blob `5cd7e8315692fae800b7de0c641fecefa90c13ad`. Tested binary SHA-256 was `9664eb0794f72cbdfaded2899c76bf326d6bf2cc78d8c95bdcb0cd1550e695e5`; the prior binary remains at `/tmp/pong-bracket-pre-pause-20261113/pong-bracket-lib`.

With permitted display access, the new binary completed a bounded scripted replay with 13 telemetry records exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl` and six app-owned PNGs. Outputs remain under `/tmp/pong-bracket-held-pause-20261113/` and `/tmp/pong-bracket-held-pause-output-20261113.txt`. The captures were not visually inspected in this run. Replay bypasses real keys and does not independently verify pause behavior.

Native/physical input, other platforms, rebuilt packages, full matches and release qualification remain pending. Earlier evidence is historical. Other engine adapters have independent input paths.

## Mac recheck

macOS ARM64 source `198f9a46c560f71eeafc782fd9db10318308cd28`. Offline locked release build reused a copy-on-write cache; all eleven bracket-lib frontend tests passed, including held-input suppression. Cargo reported a future-incompatibility warning for `block` 0.1.6. Executable SHA-256 `2518e0228d105b89f392c3e50efd09774d6591a745b5b6c4e3e83c36d62b90e7`.

The packaged app opened through supported desktop automation. Space started play; sequential held W, Space, Space, held S and Space produced visible upward movement, Paused, resumed play, downward movement and Paused again. Held Q exited. W was released before pause: held-across-pause native delivery and physical input were not demonstrated. No fresh scripted replay or capture is claimed for this Mac build.
