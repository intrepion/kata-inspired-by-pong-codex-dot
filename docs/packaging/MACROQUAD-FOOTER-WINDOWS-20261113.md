# Macroquad footer: Windows cross-check

At checkpoint `97ac63031efc3d6077893e069d5b12f19eda43b3`, `clients/macroquad/src/main.rs` Git blob `1191a206f68c5bdefd6dc82a27c158e60618ccee` includes the footer fitting change. On Windows stable MSVC Rust, `cargo test --locked --offline -p pong-macroquad` passed all six tests, and a locked offline debug build succeeded. Cargo reported incremental-cache hard-link failures and copied files instead.

The resulting executable SHA-256 was `9e96d51cbc0b6448b1d92060638e01dc22cda1e56fdb5c619e6122ebab1d2375`. Its scripted run exited 0 with empty stderr, and all 13 JSON telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned captures were generated. The inspected 1000 x 680 tick-480 capture (SHA-256 `e4b6caba848b0a3c11fae0741a69da46ed42aea51e77dbea33ea01c7e1cab62a`) showed the complete controls footer, paused prompt and court at that default captured size. The prior executable, logs and captures remain in sibling workspace directory `macroquad-footer-windows-20261113`.

This does not verify Windows resize behavior at narrower sizes, physical input, an interactive match, clean-machine operation, license clearance or release qualification. The Mac resize check is documented separately in `MACROQUAD-FOOTER-RESIZE-MAC-20261113.md`.
