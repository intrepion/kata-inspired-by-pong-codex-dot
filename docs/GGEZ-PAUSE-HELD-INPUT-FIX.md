# ggez explicit-pause held-input fix

ggez's Space and Escape pause paths previously left held movement active, and repeated movement key-down events could restore a cleared hold. A new adapter-boundary test failed before the fix. The shared physics was not changed.

The key-down adapter now clears movement when Space or Escape transitions Playing to Paused and ignores repeated movement key-down events. Existing key-up handling still releases bindings; a fresh press after resume moves normally. Tests cover W, Up, S and Down, both pause keys, Enter resume, repeat suppression and restart. All four ggez release tests passed on Omarchy, as did an offline locked release build. The tested tree was based on `b220e8f2eaf1549611200ab70b09c8b1632b9674`, with `experiments/ggez/src/main.rs` Git blob `8f709732d087003b6a690664eaf6f4587c75d4a2`. Tested binary SHA-256 was `607c5f3d9507b6f99b7045a683b0b3f2016f3decd17cc19ac4a15966bffd408d`; the previous binary remains at `/tmp/pong-ggez-pre-pause-20261113/pong-ggez`.

With permitted display access, the new binary completed its bounded scripted replay with 13 telemetry records exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl` and six app-owned PNGs. Outputs remain under `/tmp/pong-ggez-pause-fix-20261113/` and `/tmp/pong-ggez-pause-fix-output-20261113.txt`. These captures were not visually inspected in this run. The replay bypasses real key input and does not independently verify the pause behavior.

Native/physical input, other platforms, rebuilt packages, full matches and release qualification remain pending. Existing evidence is historical; other engine adapters have independent key paths and are not covered by this fix.
