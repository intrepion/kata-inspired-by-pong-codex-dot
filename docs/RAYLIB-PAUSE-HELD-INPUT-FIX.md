# raylib-rs explicit-pause held-input fix

The raylib adapter polls key states each frame. Previously a movement key held while Space paused remained active after resume without a new press. A new polled-input regression failed before the fix. Shared physics was not changed.

The adapter now suppresses each movement binding held when Space enters pause until a later poll observes its release. Fresh presses can move; aliases remain independent and opposing directions cancel. Tests cover all four movement bindings and Space start/pause/resume. All five raylib adapter release tests passed on Omarchy, as did an offline locked release build. The tested tree was based on `6825930b8c486b39e0926a19ec8016eca9be6a5e`, with `experiments/raylib-rs/src/main.rs` Git blob `3af8746708053353e433160469f4f56ab4b56620`. Tested binary SHA-256 was `cd992966ff02823608d86ef2064d995dacd47b635c92674091f9eea60de5f99c`; the earlier binary remains at `/tmp/pong-raylib-pre-pause-20261113/pong-raylib-rs`.

With permitted display access, the new binary completed its bounded scripted replay with 13 telemetry records exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl` and six app-owned PNGs. Outputs remain under `/tmp/pong-raylib-held-pause-20261113/` and `/tmp/pong-raylib-held-pause-output-20261113.txt`. These captures were not visually inspected in this run. Replay bypasses real key input and does not independently verify the pause behavior.

Native/physical input, polling behavior across focus changes, other platforms, rebuilt packages, full matches and release qualification remain pending. The existing unfocus path was not changed; the separate Windows start-input issue remains open. Existing evidence is historical; other engines have independent input paths.

## Mac recheck

macOS ARM64 source `15dabc39be97533adb47bf6317e7ec8518ac8bad`. Offline locked release build reused a copy-on-write cache; all five raylib frontend tests passed, including the held-input suppression regression. Executable SHA-256 `e04297cc55daf6fc23c383145a21849ad1cd3ed0f20061796e90c4c1df1e1897`.

The packaged app opened through supported desktop automation. Initial short Return actions left it Ready, including after clicking the court; a 300 ms Return press started play. Sequential held W, 300 ms Space, 300 ms Space, held S and 300 ms Space then produced visible upward movement, Paused, resumed play, downward movement and Paused again. Escape exited the app. The cause of missed short actions was not established. W was released before pause: held-across-pause native delivery and physical keyboard input were not demonstrated. No fresh scripted replay or capture is claimed for this Mac build.
