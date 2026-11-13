# Windows engine non-playing input recheck

At checkpoint `1f001d9684d3ad32168e2a706f563ef75734c6b2`, including `4795e13`, the bracket-lib source Git blob was `ebb54dfa3e149d926f2883a66e22a94799323d92`. Windows MSVC stable Rust 1.99.1 passed all 12 locked offline debug tests, including the non-playing press regression.

A fresh locked offline debug build produced `pong-bracket-lib.exe` SHA-256 `7cac18cf92fafdddf221045bc4ea00aeac3174fbbf501e94196b8570e64f4b6e`. Its scripted run exited 0 and all 13 JSON telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Stdout also logged OpenGL 3.2.0 and shader version 1.50 (build 31.0.101.2145); stderr was empty. No captures or native key interactions were checked. The separate build target is `target-four-engine-nonplaying-windows-20261113`; replay output remains in sibling workspace directory `four-engine-nonplaying-windows-20261113`.

The Bevy, Blue Engine and Fyrox Windows tests and replays for this fix remain outstanding. Adapter tests and scripted replay do not verify physical keyboard input, accessibility, clean-machine operation, license clearance or release readiness.
