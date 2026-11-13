# raylib-rs explicit-pause held-input fix

The raylib adapter polls key states each frame. Previously a movement key held while Space paused remained active after resume without a new press. A new polled-input regression failed before the fix. Shared physics was not changed.

The adapter now suppresses each movement binding held when Space enters pause until a later poll observes its release. Fresh presses can move; aliases remain independent and opposing directions cancel. Tests cover all four movement bindings and Space start/pause/resume. All five raylib adapter release tests passed on Omarchy, as did an offline locked release build. The tested tree was based on `6825930b8c486b39e0926a19ec8016eca9be6a5e`, with `experiments/raylib-rs/src/main.rs` Git blob `3af8746708053353e433160469f4f56ab4b56620`. Tested binary SHA-256 was `cd992966ff02823608d86ef2064d995dacd47b635c92674091f9eea60de5f99c`; the earlier binary remains at `/tmp/pong-raylib-pre-pause-20261113/pong-raylib-rs`.

With permitted display access, the new binary completed its bounded scripted replay with 13 telemetry records exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl` and six app-owned PNGs. Outputs remain under `/tmp/pong-raylib-held-pause-20261113/` and `/tmp/pong-raylib-held-pause-output-20261113.txt`. These captures were not visually inspected in this run. Replay bypasses real key input and does not independently verify the pause behavior.

Native/physical input, polling behavior across focus changes, other platforms, rebuilt packages, full matches and release qualification remain pending. The existing unfocus path was not changed; the separate Windows start-input issue remains open. Existing evidence is historical; other engines have independent input paths.
