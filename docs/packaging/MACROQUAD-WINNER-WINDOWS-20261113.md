# Windows Macroquad winner-message recheck

At checkpoint `550905c72d272c625a8ff2db543c4426c1eb442c`, `clients/macroquad/src/main.rs` Git blob `85381e00aa9298ff1f8be166bb1b7b983b0639df`, Windows MSVC stable Rust 1.99.1 passed all six locked offline debug tests. The winner-message test drives a winning score for each side, verifies the respective status string and checks restart clears it. Cargo fell back to copying incremental files when hard-linking failed. This is adapter/model evidence, not a rendered winner screen.

A fresh locked offline debug executable had SHA-256 `8c44b4e598f7d2fb8674c207fb1a049b05a23b05712f5c26779db58cc7335893`. Its scripted run exited 0 with empty stderr and all 13 parsed telemetry records matching `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned captures were generated; the inspected tick 480 image shows the paused court and prompt. The standard replay does not reach game over, so winner rendering remains unverified. Output and captures remain in sibling workspace directory `macroquad-winner-windows-20261113`, with prior binaries copied into its `prior` directory before reusing `target-polling-active-windows-20261113`.

No native key delivery, physical input, full interactive match, clean-machine installation or release qualification is claimed.
