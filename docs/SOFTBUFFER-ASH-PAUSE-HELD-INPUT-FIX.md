# Softbuffer and ash explicit-pause held-input fix

Softbuffer and ash use separate winit key-event adapters over `pong-low-level-common::App`. Their Space pause paths previously left held movement active, and repeated movement key-down events could restore a cleared hold. A new adapter-boundary test failed on both before the fix. Shared physics and the common model were not changed.

Each adapter now clears movement when Space transitions Playing to Paused and ignores repeated movement key-down events; releases still clear bindings and a fresh press after resume moves normally. Tests cover W, Up, S and Down individually, and preserve start, pause, repeat, restart and quit behavior. All six softbuffer and eight ash release tests passed on Omarchy, as did locked offline release builds. The tested tree was based on `679e0040688ff388d9b26628100dde09016870d6`; source blobs were `2d20d034bcff1de3826d49f511e367b8fac574e8` (softbuffer) and `cdea89cffee622cda2a6b7b68c04fd11b8d12d6a` (ash). Tested binary SHA-256 values were `8fe8c65efb230c92e1bd67117232e4c9cdb3a694d5e30ae0290b297e56e3e3b6` and `3ae1c14f98acb0cd54ff4dc57277e164fdbc23efad5f7104005abe6f7c266c93`. Prior binaries remain in `/tmp/pong-softbuffer-ash-pre-pause-20261113/`.

With permitted display access, both binaries completed bounded scripted replays with 13 telemetry records exactly matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl` and six app-owned PNGs each. Outputs remain under `/tmp/pong-softbuffer-pause-fix-20261113/`, `/tmp/pong-ash-pause-fix-20261113/` and corresponding `-output-20261113.txt` files. These captures were not visually inspected in this run. Replays bypass real key input and do not independently verify the pause behavior.

Native/physical input, other platforms, rebuilt packages, full matches and release qualification remain pending. Previous evidence is historical; other adapters have separate key paths and are not covered by this fix.

## Mac recheck

macOS ARM64 source `caca70820acbb49046878fae07eb69509dc5eca6`. An offline locked softbuffer release build reused a copy-on-write cache and all six frontend tests passed, including the held-input action-boundary regression. Binary SHA-256 `51ebf96596fc6e92d1c1499eb03604599db19b4d8308e86f4d79a4b2139a5694`.

The packaged app opened via supported desktop automation. Sequential Space, held W, Space, Space, held S and Space produced visible upward movement, Paused, resumed play, downward movement and Paused. The app exited. W was released before pause; native held-across-pause and physical keyboard input were not tested. No fresh scripted replay or capture is claimed. Ash was not rebuilt or run in this check; its previously documented Mac Vulkan loader/MoltenVK runtime blocker remains.
