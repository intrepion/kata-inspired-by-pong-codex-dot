# Windows ggez non-playing movement recheck

On Windows, source checkpoint `9a3b6f48067fc9141a39b1a543f659239fe87931` includes fix `b4019e81189ef91d105b6c8d603893d6081d522a`; `experiments/ggez/src/main.rs` has Git blob `754f43a5b7b37fa4e6dd8b2db78fea3721e6b41e`. The relevant source and local dependency paths were clean before building. The isolated target preserved earlier binaries.

With stable MSVC Rust 1.99.1, `cargo test --manifest-path experiments/ggez/Cargo.toml --release --locked --offline` passed all five tests, including the regression for movement presses in Ready, Paused and Over, repeats and a fresh press after resuming. A locked offline release build succeeded. Its `pong-ggez.exe` SHA-256 is `39899a0e9b5aceca7fcbcd09d3860c3f191365f67fb3c7bcab53bc0662bb83b3`.

The built executable's `--scripted-test --scripted-captures` run exited successfully; all 13 telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned captures were generated. The tick 480 capture was visually inspected and shows the paused court and prompt. These scripted results do not exercise native key delivery; the regression is a model-level test, not a physical keyboard or accessibility test. This is not clean-machine, distribution or license qualification.

The executable and test artifacts remain locally in `target-ggez-nonplaying-windows-20261113`; the replay and captures remain in the sibling workspace directory `ggez-nonplaying-windows-20261113`.
