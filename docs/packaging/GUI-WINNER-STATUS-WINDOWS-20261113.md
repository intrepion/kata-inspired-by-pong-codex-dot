# Shared GUI winner status: Windows cross-check

At checkpoint `608263befc4149274f95ac3a1010058a79d35cce`, Windows stable MSVC Rust ran `cargo test --manifest-path experiments/gui-support/Cargo.toml --locked --offline`: all 14 tests passed, including both winner labels and restart. Shared model source blob was `9cf40e771e7e841af30d9324c5dbfb7b02d8e25c`.

A locked offline debug build of `experiments/gui-slint` produced `pong-slint.exe` SHA-256 `584c086310a7b2bf8db4f6304b5940a9ad3734016b89d9def1bc8ebe78d79115`. Its isolated scripted run exited 0 with empty stderr and all 13 JSON telemetry records equal to `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned captures were generated. The inspected tick-480 image (SHA-256 `b8ee9d310615e19f4bb6cc228ccab8bc21f96c4333a6ceca98d655ce39091a27`) showed the paused status, score and court. Logs, captures and the prior executable are preserved in sibling workspace directory `gui-winner-windows-20261113`.

The replay does not reach game over and therefore does not visually verify the winner label. Model tests, scripted replay and app-owned captures do not establish physical input, a complete interactive match, GTK or GPUI runtime behavior, clean-machine operation, dependency-license clearance or release readiness.
