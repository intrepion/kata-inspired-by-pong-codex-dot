# Non-playing movement: macOS regression cross-check

2026-11-13. In an isolated copy of the checkout at b4019e81189ef91d105b6c8d603893d6081d522a, the changed source files for fixes 04b0cbd, 4795e13, b810af1 and df2eb68 were fetched from checkpoint affffcd3881d2521a6622e4fec09959ba8f127de. This was a source overlay, not a complete checkout of that checkpoint. The comparison of those revisions did not report changes to the tested manifests or shared crates. Locked offline release-profile tests ran using separate copy-on-write target directories and the existing cached dependencies. Existing checkouts, targets and evidence were preserved.

| Adapter or controller | Passing tests | Tested source SHA-256 |
| --- | ---: | --- |
| Macroquad | 3 | `920147f465909ee7ed21c7c3e5a4b198de889d65f4dadb6beaa6c2779ba3c2c9` |
| Nannou | 8 | `3eca68fabda1e4ce845aae8b752e800203993140f4755c207d8663faf3c81521` |
| raylib-rs | 8 | `5004981814137bdedd8328fd07f880fbc63fe0c0bd7f0efc7fe645d2f2d7ec4c` |
| SDL3 | 7 | `a7875afdeecf2825e6ca71b760ebd56bc5fbbcaa4ca1a9ad6579dd9003a42ef7` |
| Bevy | 4 | `1c869f8a8fd82fb94c9af63c96febdc6141544cc801ff35ac1a2ca3626cc86b7` |
| Blue Engine | 6 | `fdac3bfb2f4598554457e0a5e2fdc861a81d768ae7bb4572b4fd08a2a3612c99` |
| bracket-lib | 12 | `cc70cfd2720ef7bbec79e999a033f9b82cf33416eb58bebfecf65d9349460e1c` |
| Fyrox | 5 | `4f3b6feb5c4cba2989193bc72fc195ed74a303122ab2dc534ea9c34e85462b05` |
| Shared GUI support | 11 | `9c02b55aeba64094b837ea3d6303daad36ca4ad5548d1c624c8722dfd45814dc` |
| Shared Rust web controller | 14 | `6b87519e24519eb8fca61b777ea62bd34414e5ccd121f63ba6e5fdbecd8d87df` |

These adapter/model tests exercise held movement, pause or non-playing transitions and release/fresh press as applicable. They do not test native event delivery, real keyboard input, graphical frontends or physical interaction. The independent web-wasm JavaScript controller passed two Node unit tests, with input.mjs SHA-256 `59b9bee9e5213ba0ae915ab3618f37c33cac40d9ef6533bc5f6da2037823e0a6`; no browser or Tauri runtime test was performed.

Godot 4.5.1 headless ran the updated injected-event input smoke in a copied prior project with its existing native extension, and exited 0 with GODOT_INPUT_SMOKE_PASS for Ready, held-across-pause, pressed-while-paused, four movement bindings, three pause keys, four resume/restart controls, fresh press and an independent alias. The executable SHA-256 was `0d214c6c85e47e7060ac13b035b404ce4dda653992503914963e149a45cd0dca`, and extension SHA-256 `cbd5fa8f5c7d848fa68187bf5bb77bc508c01c80b645affec8e1b80e9a3d82b6`. Updated main.gd SHA-256 `8f3d3ca61c4aec6a66ce7d0636c3475f2953b6c9ef6533bc5f6da2037823e0a6` and input_smoke.gd SHA-256 `997474a3fd91bce5e00f4dd6c4c91d67ac0c0f227c3de228800987b00b34d4f3`. The engine logged a system CA certificate lookup error; the input smoke still passed. These injected events do not verify physical keys or graphical focus/compositor behavior.

The first SDL3 test attempt was interrupted when it started rebuilding SDL; a separately cloned existing SDL target then completed its seven tests. No rebuilt frontend replay, clean-machine package, signing, licensing clearance or release qualification is claimed. Isolated source and targets remain under `/tmp/pong-mac-input-recheck-20261113-v1`, `/tmp/pong-mac-input-target-20261113-v1`, `/tmp/pong-sdl3-nonplaying-mac-target-20261113-v1` and `/tmp/pong-bevy-nonplaying-mac-target-20261113-v1`; Godot project and log use the `/tmp/pong-godot-nonplaying-mac-*` prefix.
