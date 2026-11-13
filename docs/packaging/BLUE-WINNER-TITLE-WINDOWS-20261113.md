# Blue Engine winner title: Windows cross-check

At checkpoint `f99d949031b6fc40a56029d42c5ad2ea1e5972bc`, Blue Engine source Git blob `a71f3cc5f004bb61e7cd419f9f0e0b5903995bac` was tested on Windows stable MSVC Rust. `cargo test --manifest-path experiments/blue-engine/Cargo.toml --locked --offline` passed all nine tests, including the winner-title and restart regression. A locked offline windowed debug build succeeded.

The executable SHA-256 was `386f77822b3f4a63b0b8c44d977ebe7c7ae75ed4a04718120d4b1b2fa3013fc4`. Its `--scripted-test` run exited 0 and all 13 JSON telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Stderr was not empty: graphics loader diagnostics included a missing `VK_LAYER_KHRONOS_validation` warning. No capture was requested. The previous executable and local logs remain in sibling workspace directory `blue-winner-windows-20261113`.

The scenario does not reach game over, so the window title at match end was not visually verified. These results do not establish physical input, a full interactive match, clean-machine operation, dependency-license clearance or release qualification. The winner is in the window title and may be hidden by the window manager.
