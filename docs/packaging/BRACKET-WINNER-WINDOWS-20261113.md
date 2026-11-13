# bracket-lib winner header: Windows cross-check

At checkpoint `924fab54357eb246c9497dfd2d3f26abdf05224e`, the bracket-lib source Git blob was `dd0b975487a1cd81130abfe009b1ddd224a2704d`. On Windows stable MSVC Rust, `cargo test --manifest-path experiments/bracket-lib/Cargo.toml --locked --offline` passed all 15 tests, including winner labels and restart, and the locked offline debug build succeeded.

The executable SHA-256 was `ae8918ef2cd5bedb8a715589f11767c8bb1655655cdb5d9e3a892c4f5be7395b`. Its scripted run exited 0 with empty stderr, and all 13 JSON telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned captures were generated. The inspected tick-480 capture (SHA-256 `2e577938b2e9d793a1ddd233929dc3b0be61af82c88cfb89092829d647c9cfab`) showed the paused header, score, court and paddles. The previous executable, logs and captures remain in sibling workspace directory `bracket-winner-windows-20261113`.

The scripted scenario does not reach game over, so the rendered winner label remains unverified. These results do not establish physical input, a complete interactive match, clean-machine operation, dependency-license clearance or release qualification.
